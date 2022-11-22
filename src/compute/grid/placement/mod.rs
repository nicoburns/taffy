pub mod cell_occupancy;
pub mod coordinates;
pub mod estimate_size;
pub mod placement_algo;

pub(in super) use cell_occupancy::{CellOccupancyMatrix, CellOccupancyState, TrackCounts};
pub(in super) use estimate_size::compute_grid_size_estimate;
pub(in super) use placement_algo::place_grid_items;
