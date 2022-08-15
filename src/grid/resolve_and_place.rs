use super::types::{CssGrid, GridAxisTracks, GridTrack, RowColumn};
use crate::node::Node;
use crate::style::{Dimension, FlexboxLayout, GridAutoFlow, TrackSizingFunction};
use crate::sys::GridTrackVec;
use crate::tree::LayoutTree;

/// 7.1. The Explicit Grid
/// Initialise the `rows` and `columns` fields of the `Grid` based on following style properties:
/// - `grid-template-rows`
/// - `grid-template-columns`
pub(super) fn resolve_explicit_grid_track(
    track_list: &mut GridAxisTracks,
    track_template: &GridTrackVec<TrackSizingFunction>,
    gap: Dimension,
) {
    let mut track_count = 0;
    track_template.iter().enumerate().for_each(|(index, track_sizing_function): (usize, &TrackSizingFunction)| {
        // Generate gutter in between each track
        if index != 0 {
            track_list.push(GridTrack::gutter(gap))
        }

        // Generate track
        track_list.push(GridTrack::new(
            track_sizing_function.min_sizing_function(),
            track_sizing_function.max_sizing_function(),
        ));

        // Count track
        track_count += 1;
    });

    track_list.explicit_track_count = track_count;
}

/// 8.5. Grid Item Placement Algorithm
/// Place items into the grid, generating new rows/column into the implicit grid as required
///
/// [Specification](https://www.w3.org/TR/css-grid-2/#auto-placement-algo)
pub(super) fn place_grid_items(grid: &mut CssGrid, tree: &impl LayoutTree, node: Node) {
    let style = tree.style(node);
    let use_dense_placement = style.grid_auto_flow.is_dense();
    let mut grid_position: (u16, u16) = (0, 0);

    use RowColumn::*;
    let flow_direction = match style.grid_auto_flow {
        GridAutoFlow::Row | GridAutoFlow::RowDense => RowColumn::Row,
        GridAutoFlow::Column | GridAutoFlow::ColumnDense => RowColumn::Column,
    };
    // let secondary_fill_direction = primary_fill_direction.other();

    // Create a vector of children's styles that we can iterate over multiple times
    let children_styles: Vec<&FlexboxLayout> =
        tree.children(node).into_iter().copied().map(|child_node| tree.style(child_node)).collect();

    // Place children with definite positions
    children_styles
        .iter()
        .filter(|child_style| child_style.grid_row.is_definite() && child_style.grid_column.is_definite())
        .for_each(|child_style| {
            let placed_grid_position = place_grid_item(grid, tree, node, flow_direction, grid_position, child_style);
            if !use_dense_placement {
                grid_position = placed_grid_position
            }
        });

    // Place remaining children with definite row positions
    children_styles
        .iter()
        .filter(|child_style| match flow_direction {
            Row => child_style.grid_row.is_definite() && !child_style.grid_column.is_definite(),
            Column => !child_style.grid_row.is_definite() && child_style.grid_column.is_definite(),
        })
        .for_each(|child_style| {
            let placed_grid_position = place_grid_item(grid, tree, node, flow_direction, grid_position, child_style);
            if !use_dense_placement {
                grid_position = placed_grid_position
            }
        });

    // Place remaining children with definite column positions
    children_styles
        .iter()
        .filter(|child_style| !child_style.grid_row.is_definite() && child_style.grid_column.is_definite())
        .for_each(|child_style| {
            let placed_grid_position = place_grid_item(grid, tree, node, flow_direction, grid_position, child_style);
            if !use_dense_placement {
                grid_position = placed_grid_position
            }
        });

    // Place children with no definite position in either axis
    children_styles
        .iter()
        .filter(|child_style| !child_style.grid_row.is_definite() && !child_style.grid_column.is_definite())
        .for_each(|child_style| {
            let placed_grid_position = place_grid_item(grid, tree, node, flow_direction, grid_position, child_style);
            if !use_dense_placement {
                grid_position = placed_grid_position
            }
        });
}

/// 8.5. Grid Item Placement Algorithm
/// Place a single item into the grid
pub(super) fn place_grid_item(
    grid: &mut CssGrid,
    tree: &impl LayoutTree,
    node: Node,
    flow_direction: RowColumn,
    grid_position: (u16, u16),
    item: &FlexboxLayout,
) -> (u16, u16) {
    (1, 1)
}
