use crate::geometry::{AbsoluteAxis, Line, Size};
use crate::style::{AvailableSpace, Dimension};
use crate::tree::{LayoutPartialTreeExt, NodeId, SizingMode};

pub(crate) fn maybe_resolve_intrinsic_dimension(
    tree: &mut impl LayoutPartialTreeExt,
    node: NodeId,
    dimension: Dimension,
    axis: AbsoluteAxis,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    box_sizing_adjustment: Size<f32>,
    vertical_margins_are_collapsible: Line<bool>,
) -> Option<f32> {
    let parent_axis_size = parent_size.get_abs(axis);
    let box_sizing_axis_adjustment = box_sizing_adjustment.get_abs(axis);

    if let Some(size) = dimension.maybe_definite_value(parent_axis_size, |val, basis| tree.calc(val, basis)) {
        return Some(size + box_sizing_axis_adjustment);
    }

    if dimension.is_min_content() {
        return Some(measure_intrinsic_size(
            tree,
            node,
            axis,
            known_dimensions,
            parent_size,
            available_space,
            AvailableSpace::MinContent,
            vertical_margins_are_collapsible,
        ));
    }

    if dimension.is_max_content() {
        return Some(measure_intrinsic_size(
            tree,
            node,
            axis,
            known_dimensions,
            parent_size,
            available_space,
            AvailableSpace::MaxContent,
            vertical_margins_are_collapsible,
        ));
    }

    if dimension.is_fit_content() {
        let min_content_size = measure_intrinsic_size(
            tree,
            node,
            axis,
            known_dimensions,
            parent_size,
            available_space,
            AvailableSpace::MinContent,
            vertical_margins_are_collapsible,
        );
        let max_content_size = measure_intrinsic_size(
            tree,
            node,
            axis,
            known_dimensions,
            parent_size,
            available_space,
            AvailableSpace::MaxContent,
            vertical_margins_are_collapsible,
        );
        let limit = dimension
            .fit_content_limit(parent_axis_size, |val, basis| tree.calc(val, basis))
            .map(|limit| limit + box_sizing_axis_adjustment)
            .unwrap_or(f32::INFINITY);

        return Some(min_content_size.max(max_content_size.min(limit)));
    }

    None
}

pub(crate) fn maybe_resolve_intrinsic_size(
    tree: &mut impl LayoutPartialTreeExt,
    node: NodeId,
    size: Size<Dimension>,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    box_sizing_adjustment: Size<f32>,
    vertical_margins_are_collapsible: Line<bool>,
) -> Size<Option<f32>> {
    let width = maybe_resolve_intrinsic_dimension(
        tree,
        node,
        size.width,
        AbsoluteAxis::Horizontal,
        known_dimensions,
        parent_size,
        available_space,
        box_sizing_adjustment,
        vertical_margins_are_collapsible,
    );
    let height = maybe_resolve_intrinsic_dimension(
        tree,
        node,
        size.height,
        AbsoluteAxis::Vertical,
        Size { width, height: known_dimensions.height },
        parent_size,
        available_space,
        box_sizing_adjustment,
        vertical_margins_are_collapsible,
    );

    Size { width, height }
}

fn measure_intrinsic_size(
    tree: &mut impl LayoutPartialTreeExt,
    node: NodeId,
    axis: AbsoluteAxis,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    axis_available_space: AvailableSpace,
    vertical_margins_are_collapsible: Line<bool>,
) -> f32 {
    tree.measure_child_size(
        node,
        known_dimensions_with_axis(known_dimensions, axis, None),
        parent_size,
        available_space_with_axis(available_space, axis, axis_available_space),
        SizingMode::ContentSize,
        axis,
        vertical_margins_are_collapsible,
    )
}

fn known_dimensions_with_axis(
    known_dimensions: Size<Option<f32>>,
    axis: AbsoluteAxis,
    value: Option<f32>,
) -> Size<Option<f32>> {
    match axis {
        AbsoluteAxis::Horizontal => Size { width: value, height: known_dimensions.height },
        AbsoluteAxis::Vertical => Size { width: known_dimensions.width, height: value },
    }
}

fn available_space_with_axis(
    available_space: Size<AvailableSpace>,
    axis: AbsoluteAxis,
    value: AvailableSpace,
) -> Size<AvailableSpace> {
    match axis {
        AbsoluteAxis::Horizontal => Size { width: value, height: available_space.height },
        AbsoluteAxis::Vertical => Size { width: available_space.width, height: value },
    }
}
