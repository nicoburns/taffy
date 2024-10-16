//! Implements placing items in the grid and resolving the implicit grid.
//! https://www.w3.org/TR/css-grid-1/#placement
use super::types::{CellOccupancyMatrix, CellOccupancyState, GridItem};
use super::util::css_grid_line_into_origin_zero_coords;
use crate::axis::{AbsoluteAxis, InBothAbsAxis};
use crate::geometry::Line;
use crate::node::Node;
use crate::style::{GridAutoFlow, GridPlacement, Style};
use crate::sys::Vec;

/// 8.5. Grid Item Placement Algorithm
/// Place items into the grid, generating new rows/column into the implicit grid as required
///
/// [Specification](https://www.w3.org/TR/css-grid-2/#auto-placement-algo)
pub(super) fn place_grid_items<'a, ChildIter>(
    cell_occupancy_matrix: &mut CellOccupancyMatrix,
    items: &mut Vec<GridItem>,
    children_iter: impl Fn() -> ChildIter,
    grid_auto_flow: GridAutoFlow,
) where
    ChildIter: Iterator<Item = (usize, Node, &'a Style)>,
{
    let primary_axis = grid_auto_flow.primary_axis();
    let secondary_axis = primary_axis.other_axis();

    let map_child_style_to_origin_zero_placement = {
        let explicit_col_count = cell_occupancy_matrix.track_counts(AbsoluteAxis::Horizontal).explicit;
        let explicit_row_count = cell_occupancy_matrix.track_counts(AbsoluteAxis::Vertical).explicit;
        move |(index, node, style): (usize, Node, &'a Style)| -> (_, _, _, &'a Style) {
            let origin_zero_placement = InBothAbsAxis {
                horizontal: style.grid_column.map(|placement| {
                    placement.map_track(|track| css_grid_line_into_origin_zero_coords(track, explicit_col_count))
                }),
                vertical: style.grid_row.map(|placement| {
                    placement.map_track(|track| css_grid_line_into_origin_zero_coords(track, explicit_row_count))
                }),
            };
            (index, node, origin_zero_placement, style)
        }
    };

    // 1. Place children with definite positions
    children_iter()
        .filter(|(_, _, child_style)| child_style.grid_row.is_definite() && child_style.grid_column.is_definite())
        .map(map_child_style_to_origin_zero_placement)
        .for_each(|(index, child_node, child_placement, style)| {
            let (row_span, col_span) = place_definite_grid_item(child_placement, primary_axis);
            record_grid_placement(
                cell_occupancy_matrix,
                items,
                child_node,
                index,
                style,
                primary_axis,
                row_span,
                col_span,
                CellOccupancyState::DefinitelyPlaced,
            );
        });

    // 2. Place remaining children with definite secondary axis positions
    children_iter()
        .filter(|(_, _, child_style)| {
            child_style.grid_placement(secondary_axis).is_definite()
                && !child_style.grid_placement(primary_axis).is_definite()
        })
        .map(map_child_style_to_origin_zero_placement)
        .for_each(|(index, child_node, child_placement, style)| {
            let (primary_span, secondary_span) =
                place_definite_secondary_axis_item(&*cell_occupancy_matrix, child_placement, grid_auto_flow);

            record_grid_placement(
                cell_occupancy_matrix,
                items,
                child_node,
                index,
                style,
                primary_axis,
                primary_span,
                secondary_span,
                CellOccupancyState::AutoPlaced,
            );
        });

    // 3. Determine the number of columns in the implicit grid
    // By the time we get to this point in the execution, this is actually already accounted for:
    //
    // 3.1 Start with the columns from the explicit grid
    //        => Handled by grid size estimate which is used to pre-size the GridOccupancyMatrix
    //
    // 3.2 Among all the items with a definite column position (explicitly positioned items, items positioned in the previous step,
    //     and items not yet positioned but with a definite column) add columns to the beginning and end of the implicit grid as necessary
    //     to accommodate those items.
    //        => Handled by expand_to_fit_range which expands the GridOccupancyMatrix as necessary
    //            -> Called by mark_area_as
    //            -> Called by record_grid_placement
    //
    // 3.3 If the largest column span among all the items without a definite column position is larger than the width of
    //     the implicit grid, add columns to the end of the implicit grid to accommodate that column span.
    //        => Handled by grid size estimate which is used to pre-size the GridOccupancyMatrix

    // 4. Position the remaining grid items
    // (which either have definite position only in the secondary axis or indefinite positions in both axis)
    let mut grid_position: (u16, u16) = (0, 0);
    let mut idx = 0;
    children_iter()
        .filter(|(_, _, child_style)| !child_style.grid_placement(secondary_axis).is_definite())
        .map(map_child_style_to_origin_zero_placement)
        .for_each(|(index, child_node, child_placement, style)| {
            idx += 1;
            #[cfg(test)]
            println!("\nItem {idx}\n==============");

            // Compute placement
            let (primary_span, secondary_span) = place_indefinitely_positioned_item(
                &*cell_occupancy_matrix,
                child_placement,
                grid_auto_flow,
                grid_position,
            );

            // Record item
            record_grid_placement(
                cell_occupancy_matrix,
                items,
                child_node,
                index,
                style,
                primary_axis,
                primary_span,
                secondary_span,
                CellOccupancyState::AutoPlaced,
            );

            // If using the "dense" placement algorithm then reset the grid position back to (0, 0) ready for the next item
            // Otherwise set it to the position of the current item so that the next item it placed after it.
            grid_position = match grid_auto_flow.is_dense() {
                true => (0, 0),
                false => (primary_span.end as u16, secondary_span.start as u16),
            }
        });
}

/// 8.5. Grid Item Placement Algorithm
/// Place a single definitely placed item into the grid
fn place_definite_grid_item(
    placement: InBothAbsAxis<Line<GridPlacement>>,
    primary_axis: AbsoluteAxis,
) -> (Line<i16>, Line<i16>) {
    // Resolve spans to tracks
    let primary_span = placement.get(primary_axis).resolve_definite_grid_tracks();
    let secondary_span = placement.get(primary_axis.other_axis()).resolve_definite_grid_tracks();

    (primary_span, secondary_span)
}

/// 8.5. Grid Item Placement Algorithm
/// Step 2. Place remaining children with definite secondary axis positions
fn place_definite_secondary_axis_item(
    cell_occupancy_matrix: &CellOccupancyMatrix,
    placement: InBothAbsAxis<Line<GridPlacement>>,
    auto_flow: GridAutoFlow,
) -> (Line<i16>, Line<i16>) {
    let primary_axis = auto_flow.primary_axis();
    let secondary_axis = primary_axis.other_axis();

    let secondary_axis_placement = placement.get(secondary_axis).resolve_definite_grid_tracks();
    let starting_position = match auto_flow.is_dense() {
        true => 0,
        false => cell_occupancy_matrix
            .last_of_type(primary_axis, secondary_axis_placement.start, CellOccupancyState::AutoPlaced)
            .unwrap_or(0),
    };

    let mut position: i16 = starting_position;
    loop {
        let primary_axis_placement = placement.get(primary_axis).resolve_indefinite_grid_tracks(position);

        let does_fit = cell_occupancy_matrix.line_area_is_unoccupied(
            primary_axis,
            primary_axis_placement,
            secondary_axis_placement,
        );

        if does_fit {
            return (primary_axis_placement, secondary_axis_placement);
        } else {
            position += 1;
        }
    }
}

/// 8.5. Grid Item Placement Algorithm
/// Step 4. Position the remaining grid items.
fn place_indefinitely_positioned_item(
    cell_occupancy_matrix: &CellOccupancyMatrix,
    placement: InBothAbsAxis<Line<GridPlacement>>,
    auto_flow: GridAutoFlow,
    grid_position: (u16, u16),
) -> (Line<i16>, Line<i16>) {
    let primary_axis = auto_flow.primary_axis();

    let primary_placement_style = placement.get(primary_axis);
    let secondary_placement_style = placement.get(primary_axis.other_axis());

    let primary_span = primary_placement_style.indefinite_span();
    let secondary_span = secondary_placement_style.indefinite_span();
    let has_definite_primary_axis_position = primary_placement_style.is_definite();
    let primary_axis_length = cell_occupancy_matrix.track_counts(primary_axis).len() as i16;

    let track_area_is_unoccupied = |primary_range, secondary_range| {
        cell_occupancy_matrix.track_area_is_unoccupied(primary_axis, primary_range, secondary_range)
    };
    let tracks_to_lines = |range| cell_occupancy_matrix.tracks_to_lines(primary_axis, range);

    let (mut primary_idx, mut secondary_idx) = grid_position;

    if has_definite_primary_axis_position {
        let definite_primary_placement = primary_placement_style.resolve_definite_grid_tracks();
        let defined_primary_idx =
            cell_occupancy_matrix.lines_to_tracks(primary_axis.other_axis(), definite_primary_placement).start as u16;

        // Compute starting position for search
        if defined_primary_idx < primary_idx && secondary_idx != 0 {
            secondary_idx = 0;
            primary_idx = defined_primary_idx + 1;
        } else {
            primary_idx = defined_primary_idx;
        }

        // Item has fixed primary axis position: so we simply increment the secondary axis position
        // until we find a space that the item fits in
        loop {
            let primary_range = (primary_idx as i16)..((primary_idx + primary_span) as i16);
            let secondary_range = (secondary_idx as i16)..((secondary_idx + secondary_span) as i16);

            let place_here = track_area_is_unoccupied(primary_range.clone(), secondary_range.clone());
            if place_here {
                let primary_span = tracks_to_lines(primary_range);
                let secondary_span = tracks_to_lines(secondary_range);
                return (primary_span, secondary_span);
            }

            secondary_idx += 1;
        }
    } else {
        // Item does not have any fixed axis, so we search along the primary axis until we hit the end of the already
        // existent tracks, and then we reset the primary axis back to zero and increment the secondary axis index.
        // We continue in this vein until we find a space that the item fits in.
        loop {
            let primary_range = (primary_idx as i16)..((primary_idx + primary_span) as i16);
            let secondary_range = (secondary_idx as i16)..((secondary_idx + secondary_span) as i16);

            // Check if item fits in it's current position, and if so place it there
            let primary_out_of_bounds = primary_range.end > primary_axis_length;
            let place_here =
                !primary_out_of_bounds && track_area_is_unoccupied(primary_range.clone(), secondary_range.clone());
            if place_here {
                let primary_span = tracks_to_lines(primary_range);
                let secondary_span = tracks_to_lines(secondary_range);
                return (primary_span, secondary_span);
            }

            // Else check the next position
            if primary_out_of_bounds {
                secondary_idx += 1;
                primary_idx = 0;
            } else {
                primary_idx += 1;
            }
        }
    }
}

/// Record the grid item in both CellOccupancyMatric and the GridItems list
/// once a definite placement has been determined
#[allow(clippy::too_many_arguments)]
fn record_grid_placement(
    cell_occupancy_matrix: &mut CellOccupancyMatrix,
    items: &mut Vec<GridItem>,
    node: Node,
    index: usize,
    style: &Style,
    primary_axis: AbsoluteAxis,
    primary_span: Line<i16>,
    secondary_span: Line<i16>,
    placement_type: CellOccupancyState,
) {
    #[cfg(test)]
    println!("BEFORE placement:");
    #[cfg(test)]
    println!("{cell_occupancy_matrix:?}");

    // Mark area of grid as occupied
    cell_occupancy_matrix.mark_area_as(primary_axis, primary_span, secondary_span, placement_type);

    // Create grid item
    let (col_span, row_span) = match primary_axis {
        AbsoluteAxis::Horizontal => (primary_span, secondary_span),
        AbsoluteAxis::Vertical => (secondary_span, primary_span),
    };
    items.push(GridItem::new_with_placement_style_and_order(node, col_span, row_span, style, index as u16));

    #[cfg(test)]
    println!("AFTER placement:");
    #[cfg(test)]
    println!("{cell_occupancy_matrix:?}");
    #[cfg(test)]
    println!("\n");
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {
    // It's more readable if the test code is uniform, so we tolerate unnecessary clones in tests
    #![allow(clippy::redundant_clone)]

    mod test_placement_algorithm {
        use crate::compute::grid::implicit_grid::compute_grid_size_estimate;
        use crate::compute::grid::types::TrackCounts;
        use crate::compute::grid::util::*;
        use crate::compute::grid::CellOccupancyMatrix;
        use crate::prelude::*;
        use crate::style::{GridAutoFlow, GridPlacement::*};
        use slotmap::SlotMap;

        use super::super::place_grid_items;

        type ExpectedPlacement = (i16, i16, i16, i16);

        fn placement_test_runner(
            explicit_col_count: u16,
            explicit_row_count: u16,
            children: Vec<(usize, Node, Style, ExpectedPlacement)>,
            expected_col_counts: TrackCounts,
            expected_row_counts: TrackCounts,
            flow: GridAutoFlow,
        ) {
            // Setup test
            let children_iter = || children.iter().map(|(index, node, style, _)| (*index, *node, style));
            let child_styles_iter = children.iter().map(|(_, _, style, _)| style);
            let estimated_sizes = compute_grid_size_estimate(explicit_col_count, explicit_row_count, child_styles_iter);
            let mut items = Vec::new();
            let mut cell_occupancy_matrix =
                CellOccupancyMatrix::with_track_counts(estimated_sizes.0, estimated_sizes.1);

            // Run placement algorithm
            place_grid_items(&mut cell_occupancy_matrix, &mut items, children_iter, flow);

            // Assert that each item has been placed in the right location
            let mut sorted_children = children.clone();
            sorted_children.sort_by_key(|child| child.0);
            for (idx, ((_sort_order, node, _style, expected_placement), item)) in
                sorted_children.iter().zip(items.iter()).enumerate()
            {
                assert_eq!(item.node, *node);
                let actual_placement = (item.column.start, item.column.end, item.row.start, item.row.end);
                assert_eq!(actual_placement, *expected_placement, "Item {idx} (0-indexed)");
            }

            // Assert that the correct number of implicit rows have been generated
            let actual_row_counts = *cell_occupancy_matrix.track_counts(crate::compute::grid::AbsoluteAxis::Vertical);
            assert_eq!(actual_row_counts, expected_row_counts, "row track counts");
            let actual_col_counts = *cell_occupancy_matrix.track_counts(crate::compute::grid::AbsoluteAxis::Horizontal);
            assert_eq!(actual_col_counts, expected_col_counts, "column track counts");
        }

        #[test]
        fn test_only_fixed_placement() {
            let flow = GridAutoFlow::Row;
            let explicit_col_count = 2;
            let explicit_row_count = 2;
            let children = {
                let mut sm = SlotMap::new();
                vec![
                    // node, style (grid coords), expected_placement (oz coords)
                    (1, sm.insert(()), (Line(1), Auto, Line(1), Auto).into_grid_child(), (0, 1, 0, 1)),
                    (2, sm.insert(()), (Line(-4), Auto, Line(-3), Auto).into_grid_child(), (-1, 0, 0, 1)),
                    (3, sm.insert(()), (Line(-3), Auto, Line(-4), Auto).into_grid_child(), (0, 1, -1, 0)),
                    (4, sm.insert(()), (Line(3), Span(2), Line(5), Auto).into_grid_child(), (2, 4, 4, 5)),
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 1, explicit: 2, positive_implicit: 2 };
            let expected_rows = TrackCounts { negative_implicit: 1, explicit: 2, positive_implicit: 3 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_only_auto_placement_row_flow() {
            let flow = GridAutoFlow::Row;
            let explicit_col_count = 2;
            let explicit_row_count = 2;
            let children = {
                let mut sm = SlotMap::new();
                let auto_child = (Auto, Auto, Auto, Auto).into_grid_child();
                vec![
                    // output order, node, style (grid coords), expected_placement (oz coords)
                    (1, sm.insert(()), auto_child.clone(), (0, 1, 0, 1)),
                    (2, sm.insert(()), auto_child.clone(), (1, 2, 0, 1)),
                    (3, sm.insert(()), auto_child.clone(), (0, 1, 1, 2)),
                    (4, sm.insert(()), auto_child.clone(), (1, 2, 1, 2)),
                    (5, sm.insert(()), auto_child.clone(), (0, 1, 2, 3)),
                    (6, sm.insert(()), auto_child.clone(), (1, 2, 2, 3)),
                    (7, sm.insert(()), auto_child.clone(), (0, 1, 3, 4)),
                    (8, sm.insert(()), auto_child.clone(), (1, 2, 3, 4)),
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 0 };
            let expected_rows = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 2 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_only_auto_placement_column_flow() {
            let flow = GridAutoFlow::Column;
            let explicit_col_count = 2;
            let explicit_row_count = 2;
            let children = {
                let mut sm = SlotMap::new();
                let auto_child = (Auto, Auto, Auto, Auto).into_grid_child();
                vec![
                    // output order, node, style (grid coords), expected_placement (oz coords)
                    (1, sm.insert(()), auto_child.clone(), (0, 1, 0, 1)),
                    (2, sm.insert(()), auto_child.clone(), (0, 1, 1, 2)),
                    (3, sm.insert(()), auto_child.clone(), (1, 2, 0, 1)),
                    (4, sm.insert(()), auto_child.clone(), (1, 2, 1, 2)),
                    (5, sm.insert(()), auto_child.clone(), (2, 3, 0, 1)),
                    (6, sm.insert(()), auto_child.clone(), (2, 3, 1, 2)),
                    (7, sm.insert(()), auto_child.clone(), (3, 4, 0, 1)),
                    (8, sm.insert(()), auto_child.clone(), (3, 4, 1, 2)),
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 2 };
            let expected_rows = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 0 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_oversized_item() {
            let flow = GridAutoFlow::Row;
            let explicit_col_count = 2;
            let explicit_row_count = 2;
            let children = {
                let mut sm = SlotMap::new();
                vec![
                    // output order, node, style (grid coords), expected_placement (oz coords)
                    (1, sm.insert(()), (Span(5), Auto, Auto, Auto).into_grid_child(), (0, 5, 0, 1)),
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 3 };
            let expected_rows = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 0 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_fixed_in_secondary_axis() {
            let flow = GridAutoFlow::Row;
            let explicit_col_count = 2;
            let explicit_row_count = 2;
            let children = {
                let mut sm = SlotMap::new();
                vec![
                    // output order, node, style (grid coords), expected_placement (oz coords)
                    (1, sm.insert(()), (Span(2), Auto, Line(1), Auto).into_grid_child(), (0, 2, 0, 1)),
                    (2, sm.insert(()), (Auto, Auto, Line(2), Auto).into_grid_child(), (0, 1, 1, 2)),
                    (3, sm.insert(()), (Auto, Auto, Line(1), Auto).into_grid_child(), (2, 3, 0, 1)),
                    (4, sm.insert(()), (Auto, Auto, Line(4), Auto).into_grid_child(), (0, 1, 3, 4)),
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 1 };
            let expected_rows = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 2 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_definite_in_secondary_axis_with_fully_definite_negative() {
            let flow = GridAutoFlow::Row;
            let explicit_col_count = 2;
            let explicit_row_count = 2;
            let children = {
                let mut sm = SlotMap::new();
                vec![
                    // output order, node, style (grid coords), expected_placement (oz coords)
                    (2, sm.insert(()), (Auto, Auto, Line(2), Auto).into_grid_child(), (0, 1, 1, 2)),
                    (1, sm.insert(()), (Line(-4), Auto, Line(2), Auto).into_grid_child(), (-1, 0, 1, 2)),
                    (3, sm.insert(()), (Auto, Auto, Line(1), Auto).into_grid_child(), (0, 1, 0, 1)),
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 1, explicit: 2, positive_implicit: 0 };
            let expected_rows = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 0 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_dense_packing_algorithm() {
            let flow = GridAutoFlow::RowDense;
            let explicit_col_count = 4;
            let explicit_row_count = 4;
            let children = {
                let mut sm = SlotMap::new();
                vec![
                    // output order, node, style (grid coords), expected_placement (oz coords)
                    (1, sm.insert(()), (Line(2), Auto, Line(1), Auto).into_grid_child(), (1, 2, 0, 1)), // Definitely positioned in column 2
                    (2, sm.insert(()), (Span(2), Auto, Auto, Auto).into_grid_child(), (2, 4, 0, 1)), // Spans 2 columns, so positioned after item 1
                    (3, sm.insert(()), (Auto, Auto, Auto, Auto).into_grid_child(), (0, 1, 0, 1)), // Spans 1 column, so should be positioned before item 1
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 0, explicit: 4, positive_implicit: 0 };
            let expected_rows = TrackCounts { negative_implicit: 0, explicit: 4, positive_implicit: 0 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_sparse_packing_algorithm() {
            let flow = GridAutoFlow::Row;
            let explicit_col_count = 4;
            let explicit_row_count = 4;
            let children = {
                let mut sm = SlotMap::new();
                vec![
                    // output order, node, style (grid coords), expected_placement (oz coords)
                    (1, sm.insert(()), (Auto, Span(3), Auto, Auto).into_grid_child(), (0, 3, 0, 1)), // Width 3
                    (2, sm.insert(()), (Auto, Span(3), Auto, Auto).into_grid_child(), (0, 3, 1, 2)), // Width 3 (wraps to next row)
                    (3, sm.insert(()), (Auto, Span(1), Auto, Auto).into_grid_child(), (3, 4, 1, 2)), // Width 1 (uses second row as we're already on it)
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 0, explicit: 4, positive_implicit: 0 };
            let expected_rows = TrackCounts { negative_implicit: 0, explicit: 4, positive_implicit: 0 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }
    }
}
