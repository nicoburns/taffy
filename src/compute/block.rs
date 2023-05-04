//! Computes the [flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) layout algorithm on [`Taffy`](crate::Taffy) according to the [spec](https://www.w3.org/TR/css-flexbox-1/)
use crate::compute::LayoutAlgorithm;
use crate::geometry::{Point, Rect, Size};
use crate::style::{AvailableSpace, Display, LengthPercentageAuto, Overflow, Position};
use crate::tree::{Layout, RunMode, SizeAndBaselines, SizingMode};
use crate::tree::{LayoutTree, NodeId};
use crate::util::sys::f32_max;
use crate::util::sys::Vec;
use crate::util::MaybeMath;
use crate::util::{MaybeResolve, ResolveOrZero};

#[cfg(feature = "debug")]
use crate::util::debug::NODE_LOGGER;

/// The public interface to Taffy's Flexbox algorithm implementation
pub struct BlockAlgorithm;
impl LayoutAlgorithm for BlockAlgorithm {
    const NAME: &'static str = "FLEXBOX";

    fn perform_layout(
        tree: &mut impl LayoutTree,
        node: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        _sizing_mode: SizingMode,
    ) -> SizeAndBaselines {
        compute(tree, node, known_dimensions, parent_size, available_space, RunMode::PeformLayout)
    }

    fn measure_size(
        tree: &mut impl LayoutTree,
        node: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        _sizing_mode: SizingMode,
    ) -> Size<f32> {
        compute(tree, node, known_dimensions, parent_size, available_space, RunMode::ComputeSize).size
    }
}

/// The intermediate results of a flexbox calculation for a single item
struct BlockItem {
    /// The identifier for the associated node
    node_id: NodeId,

    /// The base size of this item
    size: Size<Option<f32>>,
    /// The minimum allowable size of this item
    min_size: Size<Option<f32>>,
    /// The maximum allowable size of this item
    max_size: Size<Option<f32>>,

    /// The overflow style of the item
    overflow: Point<Overflow>,

    /// The final offset of this item
    inset: Rect<LengthPercentageAuto>,
    /// The margin of this item
    margin: Rect<f32>,
    /// Whether each margin is an auto margin or not
    margin_is_auto: Rect<bool>,
    /// The padding of this item
    padding: Rect<f32>,
    /// The border of this item
    border: Rect<f32>,

    /// The computed border box size of this item
    computed_size: Size<f32>,
    /// The computed "static position" of this item. The static position is the position
    /// taking into account padding, border, margins, and scrollbar_gutters but not inset
    static_position: Point<f32>,

    /// The position of the bottom edge of this item
    baseline: f32,
}

/// Computes the layout of [`LayoutTree`] according to the block layout algorithm
pub fn compute(
    tree: &mut impl LayoutTree,
    node_id: NodeId,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
) -> SizeAndBaselines {
    let style = tree.style(node_id);

    // Pull these out earlier to avoid borrowing issues
    let aspect_ratio = style.aspect_ratio;
    let margin = style.margin.resolve_or_zero(parent_size.width);
    let min_size = style.min_size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio);
    let max_size = style.max_size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio);
    let clamped_style_size =
        style.size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);

    // If both min and max in a given axis are set and max <= min then this determines the size in that axis
    let min_max_definite_size = min_size.zip_map(max_size, |min, max| match (min, max) {
        (Some(min), Some(max)) if max <= min => Some(min),
        _ => None,
    });

    // Block nodes automatically stretch fit their width to fit available space if available space is definite
    let available_space_based_size =
        Size { width: available_space.width.into_option().maybe_sub(margin.horizontal_axis_sum()), height: None };

    let styled_based_known_dimensions =
        known_dimensions.or(min_max_definite_size).or(clamped_style_size).or(available_space_based_size);

    // Short-circuit layout if the container's size is fully determined by the container's size and the run mode
    // is ComputeSize (and thus the container's size is all that we're interested in)
    if run_mode == RunMode::ComputeSize {
        if let Size { width: Some(width), height: Some(height) } = styled_based_known_dimensions {
            return Size { width, height }.into();
        }
    }

    #[cfg(feature = "debug")]
    NODE_LOGGER.log("BLOCK");
    compute_inner(tree, node_id, styled_based_known_dimensions, parent_size, available_space, run_mode)
}

/// Computes the layout of [`LayoutTree`] according to the block layout algorithm
fn compute_inner(
    tree: &mut impl LayoutTree,
    node_id: NodeId,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
) -> SizeAndBaselines {
    let style = tree.style(node_id);
    let raw_padding = style.padding;
    let raw_border = style.border;
    let padding = style.padding.resolve_or_zero(parent_size.width);
    let border = style.border.resolve_or_zero(parent_size.width);

    // Scrollbar gutters are reserved when the `overflow` property is set to `Overflow::Scroll`.
    // However, the axis are switched (transposed) because a node that scrolls vertically needs
    // *horizontal* space to be reserved for a scrollbar
    let scrollbar_gutter = style.overflow.transpose().map(|overflow| match overflow {
        Overflow::Scroll => style.scrollbar_width,
        _ => 0.0,
    });
    // TODO: make side configurable based on the `direction` property
    let mut content_box_inset = padding + border;
    content_box_inset.right += scrollbar_gutter.x;
    content_box_inset.bottom += scrollbar_gutter.y;

    let container_content_box_size = known_dimensions.maybe_sub(content_box_inset.sum_axes());
    let mut items = generate_item_list(tree, node_id, container_content_box_size);

    // Compute container width
    let container_outer_width = known_dimensions.width.unwrap_or_else(|| {
        let available_width = available_space.width.maybe_sub(content_box_inset.horizontal_axis_sum());
        determine_content_based_container_width(tree, &items, available_width) + content_box_inset.horizontal_axis_sum()
    });

    let resolved_padding = raw_padding.resolve_or_zero(Some(container_outer_width));
    let resolved_border = raw_border.resolve_or_zero(Some(container_outer_width));
    let mut resolved_content_box_inset = resolved_padding + resolved_border;
    resolved_content_box_inset.right += scrollbar_gutter.x;
    resolved_content_box_inset.bottom += scrollbar_gutter.y;

    // Perform item layout and return content height
    let intrinsic_outer_height = perform_final_layout(
        tree,
        node_id,
        &mut items,
        container_outer_width,
        content_box_inset,
        resolved_content_box_inset,
    );
    let container_outer_height = known_dimensions.height.unwrap_or(intrinsic_outer_height);

    known_dimensions.unwrap_or(Size { width: container_outer_width, height: container_outer_height }).into()
}

/// Create a `Vec` of `BlockItem` structs where each item in the `Vec` represents a child of the current node
#[inline]
fn generate_item_list(tree: &impl LayoutTree, node: NodeId, node_inner_size: Size<Option<f32>>) -> Vec<BlockItem> {
    tree.children(node)
        .map(|child_node_id| (child_node_id, tree.style(child_node_id)))
        .filter(|(_, style)| style.position != Position::Absolute)
        .filter(|(_, style)| style.display != Display::None)
        .map(|(child_node_id, child_style)| {
            let aspect_ratio = child_style.aspect_ratio;
            BlockItem {
                node_id: child_node_id,
                size: child_style.size.maybe_resolve(node_inner_size).maybe_apply_aspect_ratio(aspect_ratio),
                min_size: child_style.min_size.maybe_resolve(node_inner_size).maybe_apply_aspect_ratio(aspect_ratio),
                max_size: child_style.max_size.maybe_resolve(node_inner_size).maybe_apply_aspect_ratio(aspect_ratio),

                inset: child_style.inset,
                margin: child_style.margin.resolve_or_zero(node_inner_size.width),
                margin_is_auto: child_style.margin.map(|m| m == LengthPercentageAuto::Auto),
                padding: child_style.padding.resolve_or_zero(node_inner_size.width),
                border: child_style.border.resolve_or_zero(node_inner_size.width),
                overflow: child_style.overflow,

                // Fields to be computed later (for now we initialise with dummy values)
                computed_size: Size::zero(),
                static_position: Point::zero(),
                baseline: 0.0,
            }
        })
        .collect()
}

/// Compute the content-based width in the case that the width of the container is not known
#[inline]
fn determine_content_based_container_width(
    tree: &mut impl LayoutTree,
    items: &[BlockItem],
    available_width: AvailableSpace,
) -> f32 {
    let available_space = Size { width: available_width, height: AvailableSpace::MinContent };

    let mut max_child_width = 0.0;
    for item in items {
        let known_dimensions = item.size.maybe_clamp(item.min_size, item.max_size);
        let width = known_dimensions.width.unwrap_or_else(|| {
            let size_and_baselines = tree.perform_child_layout(
                item.node_id,
                known_dimensions,
                Size::NONE,
                available_space,
                SizingMode::InherentSize,
            );

            size_and_baselines.size.width
        });

        max_child_width = f32_max(max_child_width, width);
    }

    max_child_width
}

/// Compute each child's final size and position
#[inline]
fn perform_final_layout(
    tree: &mut impl LayoutTree,
    node_id: NodeId,
    items: &mut [BlockItem],
    container_outer_width: f32,
    content_box_inset: Rect<f32>,
    resolved_content_box_inset: Rect<f32>,
) -> f32 {
    // Resolve container_inner_width for sizing child nodes using intial content_box_inset
    let container_inner_width = container_outer_width - content_box_inset.horizontal_axis_sum();
    let parent_size = Size { width: Some(container_outer_width), height: None };
    let available_space =
        Size { width: AvailableSpace::Definite(container_inner_width), height: AvailableSpace::MinContent };

    let mut total_y_offset = resolved_content_box_inset.top;
    for item in items.iter_mut() {
        let known_dimensions = item
            .size
            .maybe_clamp(item.min_size, item.max_size)
            .map_width(|width| Some(width.unwrap_or(container_inner_width)));

        let size_and_baselines = tree.perform_child_layout(
            item.node_id,
            known_dimensions,
            parent_size,
            available_space,
            SizingMode::InherentSize,
        );

        item.computed_size = size_and_baselines.size;
        item.static_position = Point { x: resolved_content_box_inset.left, y: total_y_offset };

        let order = tree.children(node_id).position(|n| n == item.node_id).unwrap() as u32;

        // Resolve item inset
        let inset = item.inset.zip_size(Size { width: container_inner_width, height: 0.0 }, |p, s| p.maybe_resolve(s));
        let inset_offset = Point {
            x: inset.left.or(inset.right.map(|x| -x)).unwrap_or(0.0),
            y: inset.top.or(inset.bottom.map(|x| -x)).unwrap_or(0.0),
        };

        *tree.layout_mut(item.node_id) = Layout {
            order,
            size: size_and_baselines.size,
            location: Point { x: resolved_content_box_inset.left + inset_offset.x, y: total_y_offset + inset_offset.y },
        };

        total_y_offset += size_and_baselines.size.height;
    }

    total_y_offset += resolved_content_box_inset.bottom;
    total_y_offset
}
