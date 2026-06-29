//! Taffy uses two coordinate systems to refer to grid lines (the gaps/gutters between rows/columns):
use super::super::types::TrackCounts;
use crate::geometry::Line;
use core::cmp::{max, Ordering};
use core::ops::{Add, AddAssign, Sub};

/// The maximum number of explicit grid tracks (rows or columns) that Taffy will create in a single axis.
///
/// This is equal to [`MAX_GRID_LINE`]: the explicit grid is never allowed to be larger than the
/// limited grid (see below).
pub(crate) const MAX_GRID_TRACKS: u16 = MAX_GRID_LINE as u16;

/// Taffy implements [CSS Grid §5.4 "Limiting Large Grids"][spec] by clamping the implicit grid to a
/// "limited grid" whose grid lines lie within the range `[MIN_GRID_LINE, MAX_GRID_LINE]` (in
/// OriginZero coordinates). Grid areas that fall outside this range are clamped back into it (see
/// [`Line::<OriginZeroLine>::clamp_to_limited_grid`]).
///
/// This serves two purposes:
///   - It matches the spec's recommendation that the implicit grid accommodate at least the line
///     range `[-10000, 10000]`.
///   - Grid lines are stored as `i16`, so bounding the grid this way (combined with the saturating
///     arithmetic on [`OriginZeroLine`]) guarantees that the track/line bookkeeping can never
///     overflow `i16`, no matter how large the supplied track counts, line indices, or spans are,
///     and no matter how many items accumulate during placement.
///
/// [spec]: https://www.w3.org/TR/css-grid-1/#overlarge-grids
pub(crate) const MAX_GRID_LINE: i16 = 10_000;

/// The minimum (most negative) grid line index of the limited grid. See [`MAX_GRID_LINE`].
pub(crate) const MIN_GRID_LINE: i16 = -MAX_GRID_LINE;

/// Represents a grid line position in "CSS Grid Line" coordinates
///
/// "CSS Grid Line" coordinates are those used in grid-row/grid-column in the CSS grid spec:
///   - The line at left hand (or top) edge of the explicit grid is line 1
///     (and counts up from there)
///   - The line at the right hand (or bottom) edge of the explicit grid is -1
///     (and counts down from there)
///   - 0 is not a valid index
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct GridLine(i16);

impl From<i16> for GridLine {
    fn from(value: i16) -> Self {
        Self(value)
    }
}

impl GridLine {
    /// Returns the underlying i16
    pub fn as_i16(self) -> i16 {
        self.0
    }

    /// Convert into OriginZero coordinates using the specified explicit track count
    pub(crate) fn into_origin_zero_line(self, explicit_track_count: u16) -> OriginZeroLine {
        // Clamp the explicit track count so that `explicit_line_count` (and the addition below)
        // cannot overflow `i16`. The explicit track count is also clamped at its source; this is a
        // defensive belt-and-braces measure. The line index itself is *not* clamped here: large
        // line indices are instead handled when the resolved grid area is clamped to the limited
        // grid (see `Line::<OriginZeroLine>::clamp_to_limited_grid`).
        let explicit_track_count = explicit_track_count.min(MAX_GRID_TRACKS);
        let explicit_line_count = explicit_track_count + 1;
        let oz_line = match self.0.cmp(&0) {
            Ordering::Greater => self.0 - 1,
            Ordering::Less => self.0.saturating_add(explicit_line_count as i16),
            Ordering::Equal => panic!("Grid line of zero is invalid"),
        };
        OriginZeroLine(oz_line)
    }
}

/// Represents a grid line position in "OriginZero" coordinates
///
/// "OriginZero" coordinates are a normalized form:
///   - The line at left hand (or top) edge of the explicit grid is line 0
///   - The next line to the right (or down) is 1, and so on
///   - The next line to the left (or up) is -1, and so on
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct OriginZeroLine(pub i16);

// Add and Sub with Self
//
// All `OriginZeroLine` arithmetic saturates at the `i16` bounds rather than wrapping or panicking.
// Combined with clamping resolved grid areas to the limited grid (see `clamp_to_limited_grid`), this
// guarantees that pathologically large line indices, spans, or numbers of items can never cause an
// integer-overflow panic during placement.
impl Add<OriginZeroLine> for OriginZeroLine {
    type Output = Self;
    fn add(self, rhs: OriginZeroLine) -> Self::Output {
        OriginZeroLine(self.0.saturating_add(rhs.0))
    }
}
impl Sub<OriginZeroLine> for OriginZeroLine {
    type Output = Self;
    fn sub(self, rhs: OriginZeroLine) -> Self::Output {
        OriginZeroLine(self.0.saturating_sub(rhs.0))
    }
}

// Add and Sub with u16
impl Add<u16> for OriginZeroLine {
    type Output = Self;
    fn add(self, rhs: u16) -> Self::Output {
        OriginZeroLine(self.0.saturating_add_unsigned(rhs))
    }
}
impl AddAssign<u16> for OriginZeroLine {
    fn add_assign(&mut self, rhs: u16) {
        self.0 = self.0.saturating_add_unsigned(rhs);
    }
}
impl Sub<u16> for OriginZeroLine {
    type Output = Self;
    fn sub(self, rhs: u16) -> Self::Output {
        OriginZeroLine(self.0.saturating_sub_unsigned(rhs))
    }
}

impl OriginZeroLine {
    /// Clamp a single grid line index into the limited grid (`[MIN_GRID_LINE, MAX_GRID_LINE]`).
    ///
    /// Used when estimating the implicit grid size (where only individual lines are available); the
    /// spec-compliant clamping of full grid areas is done by
    /// [`Line::<OriginZeroLine>::clamp_to_limited_grid`].
    pub(crate) fn clamp_to_limited_grid(self) -> Self {
        OriginZeroLine(self.0.clamp(MIN_GRID_LINE, MAX_GRID_LINE))
    }

    /// Converts a grid line in OriginZero coordinates into the index of that same grid line in the GridTrackVec.
    pub(crate) fn into_track_vec_index(self, track_counts: TrackCounts) -> usize {
        self.try_into_track_vec_index(track_counts).unwrap_or_else(|| {
            if self.0 > 0 {
                panic!("OriginZero grid line cannot be more than the number of positive grid lines");
            } else {
                panic!("OriginZero grid line cannot be less than the number of negative grid lines");
            }
        })
    }

    /// Converts a grid line in OriginZero coordinates into the index of that same grid line in the GridTrackVec.
    ///
    /// This fallible version is used for the placement of absolutely positioned grid items:
    ///
    ///    If a grid-placement property refers to a non-existent line either by explicitly specifying such a line or by
    ///    spanning outside of the existing implicit grid, it is instead treated as specifying auto (instead of creating
    ///    new implicit grid lines).
    ///
    /// The infallible version above if used when placing regular in-flow grid items.
    pub(crate) fn try_into_track_vec_index(self, track_counts: TrackCounts) -> Option<usize> {
        // OriginZero grid line cannot be less than the number of negative grid lines
        if self.0 < -(track_counts.negative_implicit as i16) {
            return None;
        };
        // OriginZero grid line cannot be more than the number of positive grid lines
        if self.0 > (track_counts.explicit + track_counts.positive_implicit) as i16 {
            return None;
        };

        Some(2 * ((self.0 + track_counts.negative_implicit as i16) as usize))
    }

    /// The minimum number of negative implicit track there must be if a grid item starts at this line.
    pub(crate) fn implied_negative_implicit_tracks(self) -> u16 {
        if self.0 < 0 {
            self.0.unsigned_abs()
        } else {
            0
        }
    }

    /// The minimum number of positive implicit track there must be if a grid item end at this line.
    pub(crate) fn implied_positive_implicit_tracks(self, explicit_track_count: u16) -> u16 {
        if self.0 > explicit_track_count as i16 {
            self.0 as u16 - explicit_track_count
        } else {
            0
        }
    }
}

impl Line<OriginZeroLine> {
    /// The number of tracks between the start and end lines
    pub(crate) fn span(self) -> u16 {
        max(self.end.0 - self.start.0, 0) as u16
    }

    /// Clamp a resolved grid area to the limited grid (grid lines in `[MIN_GRID_LINE, MAX_GRID_LINE]`),
    /// implementing the "clamp a grid area" algorithm from
    /// [CSS Grid §5.4 "Limiting Large Grids"](https://www.w3.org/TR/css-grid-1/#overlarge-grids):
    ///
    ///   - If the grid area would span outside the limited grid, its span is clamped to the last
    ///     line of the limited grid.
    ///   - If the grid area would be placed completely outside the limited grid, its span is
    ///     truncated to 1 and the area is repositioned into the last grid track on that side.
    ///
    /// This is what keeps the implicit grid (and therefore the total track count) bounded, no matter
    /// how large the supplied line indices/spans are or how many items accumulate during placement.
    pub(crate) fn clamp_to_limited_grid(self) -> Self {
        // Resolved in-flow grid areas always have `start <= end`.
        if self.start.0 >= MAX_GRID_LINE {
            // Completely outside on the positive (end) side: last track of the limited grid.
            Line { start: OriginZeroLine(MAX_GRID_LINE - 1), end: OriginZeroLine(MAX_GRID_LINE) }
        } else if self.end.0 <= MIN_GRID_LINE {
            // Completely outside on the negative (start) side: first track of the limited grid.
            Line { start: OriginZeroLine(MIN_GRID_LINE), end: OriginZeroLine(MIN_GRID_LINE + 1) }
        } else {
            // Partially outside (or fully inside): clamp each edge to the last line of the grid.
            Line {
                start: OriginZeroLine(self.start.0.max(MIN_GRID_LINE)),
                end: OriginZeroLine(self.end.0.min(MAX_GRID_LINE)),
            }
        }
    }
}

/// A trait for the different coordinates used to define grid lines.
pub trait GridCoordinate: Copy {}
impl GridCoordinate for GridLine {}
impl GridCoordinate for OriginZeroLine {}
