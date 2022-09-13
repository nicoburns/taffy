use crate::geometry::Size;
use crate::layout::{AvailableSpace, Cache};
use crate::math::MaybeMath;
use crate::node::Node;
use crate::resolve::{MaybeResolve, ResolveOrDefault};
use crate::tree::LayoutTree;

// Define some general constants we will need for the remainder of the algorithm.
// let mut constants = compute_constants(tree.style(node), node_size, available_space);

pub(crate) fn compute(
    tree: &mut impl LayoutTree,
    node: Node,
    available_space: Size<AvailableSpace>,
    size_override: Size<Option<f32>>,
) -> Size<f32> {
    let style = tree.style(node);

    // Resolve node's preferred/min/max sizes (width/heights) against the available space
    // (percentages resolve to pixel values)
    let node_size = style
        .size
        .maybe_resolve(available_space.as_options())
        .zip_map(size_override, |style_size, size_override| size_override.or(style_size));
    let node_min_size = style.min_size.maybe_resolve(available_space.as_options());
    let node_max_size = style.max_size.maybe_resolve(available_space.as_options());

    println!("LEAF");
    dbg!(node_size);
    dbg!(node_min_size);
    dbg!(node_max_size);

    if node_size.width.is_some() && node_size.height.is_some() {
        return Size {
            width: node_size.width.maybe_max(node_min_size.width).maybe_min(node_max_size.width).unwrap_or(0.0),
            height: node_size.height.maybe_max(node_min_size.height).maybe_min(node_max_size.height).unwrap_or(0.0),
        };
    }

    if tree.needs_measure(node) {
        // Compute available space
        let available_space = Size {
            width: available_space.width.maybe_set(node_size.width),
            height: available_space.height.maybe_set(node_size.height),
        };

        // Measure node
        let measured_size = tree.measure_node(node, available_space);

        let clamped_measured_size = Size {
            width: node_size
                .width
                .unwrap_or(measured_size.width)
                .maybe_max(node_min_size.width)
                .maybe_min(node_max_size.width),
            height: node_size
                .height
                .unwrap_or(measured_size.height)
                .maybe_max(node_min_size.height)
                .maybe_min(node_max_size.height),
        };

        return clamped_measured_size;
    }

    let padding = style.padding.resolve_or_default(available_space.width.as_option());
    let border = style.border.resolve_or_default(available_space.width.as_option());
    return Size {
        width: node_size
            .width
            .unwrap_or(0.0 + padding.horizontal_axis_sum() + border.horizontal_axis_sum())
            .maybe_max(node_min_size.width)
            .maybe_min(node_max_size.width),
        height: node_size
            .height
            .unwrap_or(0.0 + padding.horizontal_axis_sum() + border.horizontal_axis_sum())
            .maybe_max(node_min_size.height)
            .maybe_min(node_max_size.height),
    };
}
