/// This module is a partial implementation of the CSS Grid Level 2 specification
/// https://www.w3.org/TR/css-grid-2/
use crate::geometry::Size;
use crate::layout::AvailableSpace;
use crate::node::Node;
use crate::style::{
    Dimension, FlexboxLayout, GridLine, MaxTrackSizingFunction, MinTrackSizingFunction, TrackSizingFunction,
};
use crate::sys::GridTrackVec;
use crate::tree::LayoutTree;
use std::cmp::max;

struct AreaOccupancyMatrix {
    areas: Vec<u16>,
    num_rows: u16,
}

impl AreaOccupancyMatrix {
    pub fn new() -> AreaOccupancyMatrix {
        AreaOccupancyMatrix { areas: Vec::new(), num_rows: 0 }
    }
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
    Gutter { name: Option<u16> },
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
    fn new(
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

    fn gutter(size: Dimension) -> GridTrack {
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
    fn new() -> GridAxisTracks {
        GridAxisTracks { inner: GridTrackVec::new() }
    }

    fn with_capacity(capacity: usize) -> GridAxisTracks {
        GridAxisTracks { inner: GridTrackVec::with_capacity(capacity) }
    }

    fn push(&mut self, item: GridTrack) {
        self.push(item)
    }

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
    available_space: Size<AvailableSpace>,
    columns: GridAxisTracks,
    rows: GridAxisTracks,
    area_occupancy_matrix: AreaOccupancyMatrix,
    named_areas: Vec<NamedArea>,
    items: Vec<GridItem>,
}

pub fn compute(tree: &mut impl LayoutTree, root: Node, available_space: Size<AvailableSpace>) {
    // Estimate the number of rows and columns in the grid as a perf optimisation to reduce allocations
    // Note that columns and rows GridAxisTracks below have size (estimate*2 - 1) to account for gutters
    let grid_size_estimate = compute_grid_size_estimate(tree, root);

    let mut grid = Grid {
        available_space,
        columns: GridAxisTracks::with_capacity((grid_size_estimate.width as usize * 2) - 1),
        rows: GridAxisTracks::with_capacity((grid_size_estimate.height as usize * 2) - 1),
        area_occupancy_matrix: AreaOccupancyMatrix::new(),
        named_areas: Vec::new(),
        items: Vec::new(),
    };

    // 7.1. The Explicit Grid
    let style = tree.style(root);
    resolve_explicit_grid_track(&mut grid.columns, &style.grid_template_columns, style.gap.width);
    resolve_explicit_grid_track(&mut grid.rows, &style.grid_template_rows, style.gap.height);
}


/// Estimate the number of rows and columns in the grid
/// This is used as a performance optimisation to pre-size vectors and reduce allocations
fn compute_grid_size_estimate(tree: &mut impl LayoutTree, node: Node) -> Size<u16> {

    // Initialise estimates with explicit track lengths (flooring at 1)
    let style = tree.style(node);
    let mut col_count_estimate = max(style.grid_template_columns.len(), 1) as u16;
    let mut row_count_estimate = max(style.grid_template_rows.len(), 1) as u16;

    // Iterate over children, producing an estimate of the implicit rows used by each child
    tree.children(node).into_iter().copied().map(|child_node| tree.style(child_node)).for_each(
        |child_style: &FlexboxLayout| {
            let col_usage = child_max_track_size_estimate(child_style.grid_column_start, child_style.grid_column_end);
            let row_usage = child_max_track_size_estimate(child_style.grid_row_start, child_style.grid_row_end);
            col_count_estimate = max(col_count_estimate, col_usage);
            row_count_estimate = max(row_count_estimate, row_usage);
        },
    );

    Size { width: col_count_estimate, height: row_count_estimate }
}

/// Helper function for `compute_grid_size_estimate`
#[inline]
fn child_max_track_size_estimate(start: GridLine, end: GridLine) -> u16 {
    use GridLine::*;
    match (start, end) {
        (Auto, Auto) => 0,
        (Auto | Track(_), Track(track)) => max(track, 0) as u16,
        (Auto, Span(span)) => span as u16,
        (Track(track), Auto) => max(track, 0) as u16,
        (Track(track), Span(span)) => (max(track, 0) as u16 + span) as u16,
        (Span(span), Auto) => span as u16,
        (Span(span1), Span(span2)) => max(span1, span2) as u16,
        (Span(span), Track(track)) => max(span, max(track, 0) as u16) as u16,
    }
}

/// 7.1. The Explicit Grid
/// Initialise the `rows` and `columns` fields of the `Grid` based on following style properties:
/// - `grid-template-rows`
/// - `grid-template-columns`
fn resolve_explicit_grid_track(
    track_list: &mut GridAxisTracks,
    track_template: &GridTrackVec<TrackSizingFunction>,
    gap: Dimension,
) {
    track_template.iter().enumerate().for_each(|(index, track_sizing_function): (usize, &TrackSizingFunction)| {
        // Generate gutter in between each track
        if index != 0 {
            track_list.push(GridTrack::gutter(gap))
        }

        // Generate track
        track_list.push(GridTrack::new(
            track_sizing_function.min_sizing_function(),
            track_sizing_function.max_sizing_function(),
        ))
    })
}
