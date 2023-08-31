use super::taffy_tree::perform_node_layout;
use crate::geometry::{Line, Point, Size};
use crate::style::AvailableSpace;
use crate::tree::{Layout, LayoutTree, NodeId, SizeBaselinesAndMargins, SizingMode, Taffy};
use crate::util::ResolveOrZero;

pub(crate) fn perform_root_node_layout(
    tree: &mut Taffy,
    node: NodeId,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
) -> SizeBaselinesAndMargins {
    let size_and_baselines = perform_node_layout(
        tree,
        node,
        known_dimensions,
        available_space.into_options(),
        available_space,
        SizingMode::InherentSize,
        Line::FALSE,
    );

    // Compute location
    let style = tree.style(node).unwrap();
    let margin = style.margin.resolve_or_zero(available_space.width.into_option());
    let inset = style.inset.resolve_or_zero(available_space.into_options());
    let location = Point { x: margin.left + inset.left, y: margin.top + inset.top };

    // Set layout
    let layout = Layout { order: 0, size: size_and_baselines.size, location };
    *tree.layout_mut(node) = layout;

    size_and_baselines
}
