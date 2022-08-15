use crate::geometry::Size;
use crate::layout::AvailableSpace;
use crate::node::Node;
use crate::style::{Dimension, MaxTrackSizingFunction, MinTrackSizingFunction};
use crate::sys::GridTrackVec;
use grid::Grid;
use std::cmp::max;

pub(super) struct AreaOccupancyMatrix {
    areas: Vec<u16>,
    num_rows: u16,
}

impl AreaOccupancyMatrix {
    pub fn new() -> AreaOccupancyMatrix {
        AreaOccupancyMatrix { areas: Vec::new(), num_rows: 0 }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) enum RowColumn {
    Row,
    Column,
}

/// The abstract axis in CSS Grid
#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) enum GridAxis {
    /// The axis in the block dimension, i.e. the vertical axis in horizontal writing modes and the horizontal axis in vertical writing modes.
    Block,
    /// The axis in the inline dimension, i.e. the horizontal axis in horizontal writing modes and the vertical axis in vertical writing modes.
    Inline,
}

impl GridAxis {
    pub fn other(&self) -> GridAxis {
        match *self {
            GridAxis::Block => GridAxis::Inline,
            GridAxis::Inline => GridAxis::Block,
        }
    }
}

/// Whether a GridTrack represents an actual track or a gutter.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) enum GridTrackKind {
    Track,
    Gutter { name: Option<u16> },
}

/// Internal sizing information for a single grid track (row/column)
/// Gutters between tracks are sized similarly to actual tracks, so they
/// are also represented by this struct
pub(super) struct GridTrack {
    pub kind: GridTrackKind,
    pub min_track_sizing_function: MinTrackSizingFunction,
    pub max_track_sizing_function: MaxTrackSizingFunction,
    pub base_size: f32,
    pub growth_limit: f32,         // Note: can be infinity
    pub infinitely_growable: bool, // https://www.w3.org/TR/css3-grid-layout/#infinitely-growable
}

impl GridTrack {
    pub fn new(
        min_track_sizing_function: MinTrackSizingFunction,
        max_track_sizing_function: MaxTrackSizingFunction,
    ) -> GridTrack {
        GridTrack {
            kind: GridTrackKind::Track,
            min_track_sizing_function,
            max_track_sizing_function,
            base_size: 0.0,
            growth_limit: 0.0,
            infinitely_growable: false,
        }
    }

    pub fn gutter(size: Dimension) -> GridTrack {
        GridTrack {
            kind: GridTrackKind::Gutter { name: None },
            min_track_sizing_function: MinTrackSizingFunction::Fixed(size),
            max_track_sizing_function: MaxTrackSizingFunction::Fixed(size),
            base_size: 0.0,
            growth_limit: 0.0,
            infinitely_growable: false,
        }
    }

    #[inline]
    pub fn is_flexible(&self) -> bool {
        match self.max_track_sizing_function {
            MaxTrackSizingFunction::Flex(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn flex_factor(&self) -> f32 {
        match self.max_track_sizing_function {
            MaxTrackSizingFunction::Flex(flex_factor) => flex_factor,
            _ => 0.0,
        }
    }
}

pub(super) trait GridAxisExt {
    fn flex_factor_sum(&self) -> f32;
    fn leftover_space(&self) -> f32;
}

pub(super) struct GridAxisTracks {
    pub tracks: GridTrackVec<GridTrack>,
    pub origin: u16,
    pub explicit_track_count: u16,
}

impl GridAxisTracks {
    pub fn new() -> GridAxisTracks {
        Self::with_capacity(0)
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> GridAxisTracks {
        Self::with_capacity_and_origin(capacity, 0)
    }

    #[inline]
    pub fn with_capacity_and_origin(capacity: usize, origin: u16) -> GridAxisTracks {
        GridAxisTracks { tracks: GridTrackVec::with_capacity(capacity), origin, explicit_track_count: 0 }
    }

    #[inline]
    pub fn len(&mut self) -> u16 {
        self.len() as u16
    }

    #[inline]
    pub fn push(&mut self, item: GridTrack) {
        self.push(item)
    }

    /// Retrieve a track by its index as defined in CSS grid coordinates
    pub fn get_track(&self, index: i16) -> Option<&GridTrack> {
        use std::cmp::Ordering;

        // Compute the index of the track in the tracks vector based on its CSS grid index
        // taking into account:
        //   - Zero is not a valid index
        //   - CSS grid indexes are 1-based, but the tracks vector is 0-based
        //   - Gutters are also stored in the tracks vector
        //   - Tracks in the tracks vector may be offset due to negative tracks
        //   - The passed index may be negative, which should resolve backwards from the end of the explicit grid
        let computed_index: i16 = match index.cmp(&0) {
            Ordering::Equal => {
                return None;
            }
            Ordering::Less => (index * 2 - 1) + (self.origin as i16),
            Ordering::Greater => {
                max(0, (self.origin + self.explicit_track_count) as i16 - (index.abs() as i16 * 2 - 1))
            }
        };

        // If the computed index is less than zero, then we can be sure that it doesn't exist in our vector
        // so we return None to indicate a non-existent track
        if computed_index < 0 {
            return None;
        }

        self.tracks.get(computed_index as usize)
    }

    /// The sum of the flex factors (fr units) of the flexible tracks.
    /// If this value is less than 1, set it to 1 instead.
    fn flex_factor_sum(&self) -> f32 {
        self.tracks.iter().map(|track| track.flex_factor()).sum::<f32>().max(1.0)
    }

    /// The space to fill minus the base sizes of the non-flexible grid tracks.
    fn leftover_space(&self) -> f32 {
        self.tracks.iter().filter(|track| !track.is_flexible()).map(|track| track.base_size).sum()
    }

    /// Let the hypothetical fr size be the leftover space divided by the flex factor sum.
    fn hypothetical_fr_size(&self) -> f32 {
        self.leftover_space() / self.flex_factor_sum()
    }
}

pub(super) enum GridPosition {
    Auto,
    LineIndex(i16),
    LineName(u16),
    // GridAreaStart(u16),
    // GridAreaEnd(u16),
}

pub(super) struct NamedArea {
    name: u16,
    row_start: u16,
    row_end: u16,
    column_start: u16,
    column_end: u16,
}

pub(super) struct GridItem {
    pub node: Node,
    pub min_content_contribution: Option<Size<f32>>,
    pub max_content_contribution: Option<Size<f32>>,
    pub row_start: GridPosition,
    pub row_end: GridPosition,
    pub column_start: GridPosition,
    pub column_end: GridPosition,
}

impl GridItem {
    pub fn new(node: Node) -> Self {
        GridItem {
            node,
            min_content_contribution: None,
            max_content_contribution: None,
            row_start: GridPosition::Auto,
            row_end: GridPosition::Auto,
            column_start: GridPosition::Auto,
            column_end: GridPosition::Auto,
        }
    }

    pub fn span(&self, axis: GridAxis) -> u16 {
        use GridPosition::LineIndex;
        match axis {
            GridAxis::Block => match (&self.row_start, &self.row_end) {
                (LineIndex(start), LineIndex(end)) => max(end - start, 0) as u16,
                _ => 0,
            },
            GridAxis::Inline => match (&self.column_start, &self.column_end) {
                (LineIndex(start), LineIndex(end)) => max(end - start, 0) as u16,
                _ => 0,
            },
        }
    }
}

pub(super) struct CssGrid {
    pub available_space: Size<AvailableSpace>,
    pub columns: GridAxisTracks,
    pub rows: GridAxisTracks,
    pub area_occupancy_matrix: Grid<u16>,
    pub named_areas: Vec<NamedArea>,
    pub items: Vec<GridItem>,
}
