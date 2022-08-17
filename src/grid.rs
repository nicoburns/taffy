/// This module is a partial implementation of the CSS Grid Level 2 specification
/// https://www.w3.org/TR/css-grid-2/
use crate::geometry::Size;
use crate::layout::AvailableSpace;
use crate::node::Node;
use crate::tree::LayoutTree;
use types::{CssGrid, GridAxisTracks, GridTrack};

mod resolve_and_place;
#[cfg(test)]
mod test_helpers;
mod types;

use self::resolve_and_place::CellOccupancyMatrix;
pub use types::AbsoluteAxis;

pub fn compute(tree: &mut impl LayoutTree, root: Node, available_space: Size<AvailableSpace>) {
    // Estimate the number of rows and columns in the grid as a perf optimisation to reduce allocations
    // The axis_track_sizes have size (grid_size_estimate*2 - 1) to account for gutters
    let grid_size_estimate = resolve_and_place::compute_grid_size_estimate(tree, root);
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
    resolve_and_place::place_grid_items(&mut grid, tree, root);

    // Push "uninitialized" placeholder tracks to negative grid tracks (< origin)
    populate_negative_grid_tracks(&mut grid.columns);
    populate_negative_grid_tracks(&mut grid.rows);

    // 7.1. The Explicit Grid
    let style = tree.style(root);
    resolve_and_place::resolve_explicit_grid_tracks(&mut grid.columns, &style.grid_template_columns, style.gap.width);
    resolve_and_place::resolve_explicit_grid_tracks(&mut grid.rows, &style.grid_template_rows, style.gap.height);
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
