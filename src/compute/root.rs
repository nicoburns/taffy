use super::taffy_tree::{perform_node_layout, perform_taffy_tree_hidden_layout};
use crate::geometry::{Line, Point, Rect, Size};
use crate::style::{AvailableSpace, Position, Display};
use crate::tree::{Layout, LayoutTree, NodeId, SizeBaselinesAndMargins, SizingMode, Taffy};
use crate::util::sys::f32_max;
use crate::util::{MaybeMath, MaybeResolve, ResolveOrZero};

pub(crate) fn perform_root_node_layout(
    tree: &mut Taffy,
    node: NodeId,
    available_space: Size<AvailableSpace>,
) -> SizeBaselinesAndMargins {

    // Load node styles
    let style = tree.style(node).unwrap();

    // Handle Display::None case
    if style.display == Display::None {
        *tree.layout_mut(node) = Layout::with_order(1 as u32);
        perform_taffy_tree_hidden_layout(tree, node);
        return SizeBaselinesAndMargins::HIDDEN;
    }

    let viewport_size = available_space.into_options();
    let viewport_size_or_zero =
        Size { width: viewport_size.width.unwrap_or(0.0), height: viewport_size.height.unwrap_or(0.0) };

    let position = style.position;
    let aspect_ratio = style.aspect_ratio;
    let inset = style.inset.resolve_or_zero(available_space.into_options());

    let inset_horizontal =
        style.inset.horizontal_components().map(|size| size.resolve_to_option(viewport_size_or_zero.width));
    let inset_vertical =
        style.inset.vertical_components().map(|size| size.resolve_to_option(viewport_size_or_zero.height));
    let padding = style.padding.map(|p| p.resolve_or_zero(viewport_size.width));
    let border = style.border.map(|p| p.resolve_or_zero(viewport_size.width));
    let padding_border_size = (padding + border).sum_axes();
    let inherent_size = style.size.maybe_resolve(viewport_size).maybe_apply_aspect_ratio(aspect_ratio);
    let min_size = style
        .min_size
        .maybe_resolve(viewport_size)
        .or(padding_border_size.map(Some))
        .maybe_max(padding_border_size)
        .maybe_apply_aspect_ratio(aspect_ratio);
    let max_size = style.max_size.maybe_resolve(viewport_size).maybe_apply_aspect_ratio(aspect_ratio);

    // Note: This is not a bug. It is part of the CSS spec that both horizontal and vertical margins
    // resolve against the WIDTH of the grid area.
    let margin = style.margin.map(|margin| margin.resolve_to_option(viewport_size_or_zero.width));

    let viewport_area_area_minus_item_margins_size = Size {
        width: viewport_size_or_zero.width.maybe_sub(margin.left).maybe_sub(margin.right),
        height: viewport_size_or_zero.height.maybe_sub(margin.top).maybe_sub(margin.bottom),
    };

    // If node is absolutely positioned and width is not set explicitly, then deduce it
    // from left, right and container_content_box if both are set.
    let width = inherent_size.width.or_else(|| {
        // Apply width derived from both the left and right properties of an absolutely
        // positioned element being set
        if position == Position::Absolute {
            if let (Some(left), Some(right)) = (inset_horizontal.start, inset_horizontal.end) {
                return Some(f32_max(viewport_area_area_minus_item_margins_size.width - left - right, 0.0));
            }
        }
        None
    });

    // Reapply aspect ratio after stretch and absolute position width adjustments
    let Size { width, height } = Size { width, height: inherent_size.height }.maybe_apply_aspect_ratio(aspect_ratio);

    let height = height.or_else(|| {
        if position == Position::Absolute {
            if let (Some(top), Some(bottom)) = (inset_vertical.start, inset_vertical.end) {
                return Some(f32_max(viewport_area_area_minus_item_margins_size.height - top - bottom, 0.0));
            }
        }
        None
    });
    // Reapply aspect ratio after stretch and absolute position height adjustments
    let Size { width, height } = Size { width, height }.maybe_apply_aspect_ratio(aspect_ratio);

    // Clamp size by min and max width/height
    let Size { width, height } = Size { width, height }.maybe_clamp(min_size, max_size);

    let size_and_baselines = perform_node_layout(
        tree,
        node,
        Size { width, height },
        available_space.into_options(),
        available_space,
        SizingMode::InherentSize,
        Line::FALSE,
    );

    // Resolve final size
    let Size { width, height } =
        Size { width, height }.unwrap_or(size_and_baselines.size).maybe_clamp(min_size, max_size);

    let non_auto_margin = margin.map(|m| m.unwrap_or(0.0));

    let free_space = Size {
        width: viewport_size_or_zero.width - width - non_auto_margin.horizontal_axis_sum(),
        height: viewport_size_or_zero.height - height - non_auto_margin.vertical_axis_sum(),
    }
    .f32_max(Size::ZERO);

    // Expand auto margins to fill available space
    let resolved_margin = {
        let auto_margin_size = Size {
            width: {
                let auto_margin_count = margin.left.is_none() as u8 + margin.right.is_none() as u8;
                if auto_margin_count > 0 {
                    free_space.width / auto_margin_count as f32
                } else {
                    0.0
                }
            },
            height: {
                let auto_margin_count = margin.top.is_none() as u8 + margin.bottom.is_none() as u8;
                if auto_margin_count > 0 {
                    free_space.height / auto_margin_count as f32
                } else {
                    0.0
                }
            },
        };

        Rect {
            left: margin.left.unwrap_or(auto_margin_size.width),
            right: margin.right.unwrap_or(auto_margin_size.width),
            top: margin.top.unwrap_or(auto_margin_size.height),
            bottom: margin.bottom.unwrap_or(auto_margin_size.height),
        }
    };

    // Set layout
    let size = Size { width, height };
    let location = Point { x: resolved_margin.left + inset.left, y: resolved_margin.top + inset.top };
    let layout = Layout { order: 0, size, location };
    *tree.layout_mut(node) = layout;

    size_and_baselines
}
