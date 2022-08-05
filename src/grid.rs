use crate::geometry::Size;
use crate::node::Node;
use crate::style::{Dimension, MinTrackSizingFunction, MaxTrackSizingFunction};
use crate::sys::GridTrackVec;
use std::cmp::max;

struct AreaOccupancyMatrix {
    areas: Vec<u16>,
    num_rows: u16,
}

/// The abstract axis in CSS Grid
#[derive(Copy, Clone, Debug, PartialEq)]
enum GridAxis {
    /// The axis in the block dimension, i.e. the vertical axis in horizontal writing modes and the horizontal axis in vertical writing modes.
    Block,
    /// The axis in the inline dimension, i.e. the horizontal axis in horizontal writing modes and the vertical axis in vertical writing modes.
    Inline,
}

/// Whether a GridTrack represents an actual track or a gutter.
#[derive(Copy, Clone, Debug, PartialEq)]
enum GridTrackKind {
    Track,
    Gutter { name: u16 },
}

/// Internal sizing information for a single grid track (row/column)
/// Gutters between tracks are sized similarly to actual tracks, so they
/// are also represented by this struct
struct GridTrack {
    kind: GridTrackKind,
    min_track_sizing_function: MinTrackSizingFunction,
    max_track_sizing_function: MaxTrackSizingFunction,
    base_size: f32,
    growth_limit: f32,         // Note: can be infinity
    infinitely_growable: bool, // https://www.w3.org/TR/css3-grid-layout/#infinitely-growable
}

impl GridTrack {
    #[inline]
    fn is_flexible(&self) -> bool {
        match self.max_track_sizing_function {
            MaxTrackSizingFunction::Flex(_) => true,
            _ => false,
        }
    }

    #[inline]
    fn flex_factor(&self) -> f32 {
        match self.max_track_sizing_function {
            MaxTrackSizingFunction::Flex(flex_factor) => flex_factor,
            _ => 0.0,
        }
    }
}

trait GridAxisExt {
    
    fn flex_factor_sum(&self) -> f32;

   
    fn leftover_space(&self) -> f32;
}

struct GridAxisTracks {
    inner: GridTrackVec<GridTrack>,
}

impl GridAxisTracks {

    /// The sum of the flex factors (fr units) of the flexible tracks.
    /// If this value is less than 1, set it to 1 instead.
    fn flex_factor_sum(&self) -> f32 {
        self.inner.iter().map(|track| track.flex_factor()).sum::<f32>().max(1.0)
    }

    /// The space to fill minus the base sizes of the non-flexible grid tracks.
    fn leftover_space(&self) -> f32 {
        self.inner.iter().filter(|track| !track.is_flexible()).map(|track| track.base_size).sum()
    }

    /// Let the hypothetical fr size be the leftover space divided by the flex factor sum.
    fn hypothetical_fr_size(&self) -> f32 {
        self.leftover_space() / self.flex_factor_sum()
    }
}

struct GridLine {}

enum AvailableSpace {
    Definite(f32),
    MinContent,
    MaxContent,
}

enum GridPosition {
    Auto,
    LineIndex(i16),
    LineName(u16),
    GridAreaStart(u16),
    GridAreaEnd(u16),
}

struct NamedArea {
    name: u16,
    row_start: u16,
    row_end: u16,
    column_start: u16,
    column_end: u16,
}

struct GridItem {
    node: Node,
    min_content_contribution: Option<Size<f32>>,
    max_content_contribution: Option<Size<f32>>,
    row_start: GridPosition,
    row_end: GridPosition,
    column_start: GridPosition,
    column_end: GridPosition,
}


impl GridItem {
    fn new(node: Node) -> Self {
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

    fn span(&self, axis: GridAxis) -> u16 {
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

struct Grid {
    width: AvailableSpace,
    height: AvailableSpace,
    columns: GridAxisTracks,
    rows: GridAxisTracks,
    area_occupancy_matrix: AreaOccupancyMatrix,
    column_gutters: GridTrackVec<GridLine>,
    row_gutters: GridTrackVec<GridLine>,
    named_areas: Vec<NamedArea>,
    items: Vec<GridItem>,
}
