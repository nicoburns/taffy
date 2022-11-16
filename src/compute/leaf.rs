use crate::geometry::Size;
use crate::layout::{Cache, LayoutMode, SizingConstraint};
use crate::math::MaybeMath;
use crate::node::Node;
use crate::resolve::{MaybeResolve, ResolveOrDefault};
use crate::tree::LayoutTree;

// Define some general constants we will need for the remainder of the algorithm.
// let mut constants = compute_constants(tree.style(node), node_size, available_space);

pub(crate) fn compute(
    tree: &mut impl LayoutTree,
    node: Node,
    sizing_constraint: Size<SizingConstraint>,
    layout_mode: LayoutMode,
) -> Size<f32> {
    let style = tree.style(node);

    // Resolve node's min/max sizes (width/heights) against the available space (percentages resolve to pixel values)

    // // Resolve node's preferred sizes (width/heights) against the available space (percentages resolve to pixel values)
    let sizing_constaint = sizing_constraint
        .zip_map(style.size, |constraint, style_size| constraint.maybe_resolve_nominal_size(style_size));
    let node_min_size = style.min_size.maybe_resolve(sizing_constraint.available_space());
    let node_max_size = style.max_size.maybe_resolve(sizing_constraint.available_space());
    let node_size = sizing_constaint.known_size();

    // println!("LEAF");
    // dbg!(node_size);
    // dbg!(node_min_size);
    // dbg!(node_max_size);

    if sizing_constaint.width.is_known() && sizing_constaint.height.is_known() {
        return Size {
            width: node_size.width.maybe_max(node_min_size.width).maybe_min(node_max_size.width).unwrap_or(0.0),
            height: node_size.height.maybe_max(node_min_size.height).maybe_min(node_max_size.height).unwrap_or(0.0),
        };
    }

    if tree.needs_measure(node) {
        // Measure node
        let measured_size = tree.measure_node(node, sizing_constaint);

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

    let padding = style.padding.resolve_or_default(sizing_constraint.width.available_space());
    let border = style.border.resolve_or_default(sizing_constraint.width.available_space());
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
