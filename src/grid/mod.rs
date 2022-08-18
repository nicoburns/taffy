//! This module is a partial implementation of the CSS Grid Level 2 specification
//! https://www.w3.org/TR/css-grid-2/
use crate::geometry::Size;
use crate::layout::AvailableSpace;
use crate::node::Node;
use crate::sys::Vec;
use crate::tree::LayoutTree;
use placement::{compute_grid_size_estimate, place_grid_items};
use explicit_grid::{compute_explicit_grid_size, resolve_explicit_grid_tracks};
use placement::CellOccupancyMatrix;
use types::{CssGrid, GridAxisTracks, GridTrack};

mod explicit_grid;
mod placement;
#[cfg(test)]
mod test_helpers;
mod types;

pub use types::AbsoluteAxis;

pub fn compute(tree: &mut impl LayoutTree, root: Node, available_space: Size<AvailableSpace>) {
    let get_child_styles_iter = |node| tree.children(node).into_iter().map(|child_node: &Node| tree.style(*child_node));
    let style = tree.style(root);
    let child_styles_iter = get_child_styles_iter(root);

    // Resolve the number of rows and columns in the explicit grid
    let (explicit_col_count, explicit_row_count) = compute_explicit_grid_size(style);

    // Estimate the number of rows and columns in the grid as a perf optimisation to reduce allocations
    // The axis_track_sizes have size (grid_size_estimate*2 - 1) to account for gutters
    let grid_size_estimate = compute_grid_size_estimate(explicit_col_count, explicit_row_count, child_styles_iter);
    let axis_origins = grid_size_estimate.map(|counts| (counts.negative_implicit * 2) + 1 - 1); // min: 0
    let axis_track_sizes = grid_size_estimate.map(|counts| (counts.len() * 2) - 1); // min: 1

    let mut grid = CssGrid {
        available_space,
        columns: GridAxisTracks::with_capacity_and_origin(axis_track_sizes.width, axis_origins.width),
        rows: GridAxisTracks::with_capacity_and_origin(axis_track_sizes.height, axis_origins.height),
        cell_occupancy_matrix: CellOccupancyMatrix::with_track_counts(
            grid_size_estimate.height,
            grid_size_estimate.width,
        ),
        named_areas: Vec::new(),
        items: Vec::with_capacity(tree.children(root).len()),
    };

    // 8. Placing Grid Items
    let grid_auto_flow = style.grid_auto_flow;
    let children_iter = tree.children(root).into_iter().copied().map(|child_node| (child_node, tree.style(child_node)));
    placement::place_grid_items(&mut grid.cell_occupancy_matrix, &mut grid.items, children_iter, grid_auto_flow);

    // Push "uninitialized" placeholder tracks to negative grid tracks (< origin)
    populate_negative_grid_tracks(&mut grid.columns);
    populate_negative_grid_tracks(&mut grid.rows);

    // 7.1. The Explicit Grid
    let style = tree.style(root);
    resolve_explicit_grid_tracks(&mut grid.columns, &style.grid_template_columns, style.gap.width);
    resolve_explicit_grid_tracks(&mut grid.rows, &style.grid_template_rows, style.gap.height);
}

fn populate_negative_grid_tracks(axis: &mut GridAxisTracks) {
    debug_assert!(
        axis.tracks.len() != 0,
        "populate_negative_grid_tracks should only ever be called on an empty grid axis"
    );
    debug_assert!(axis.origin % 2 != 0, "axis.origin should always be even");

    // If origin is zero then there are no negative grid tracks
    if axis.origin == 0 {
        return;
    }

    for _ in 0..axis.origin {
        axis.push(GridTrack::uninit());
    }
}
