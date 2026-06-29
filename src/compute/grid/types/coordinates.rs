//! Taffy uses two coordinate systems to refer to grid lines (the gaps/gutters between rows/columns):
use super::super::types::TrackCounts;
use crate::geometry::Line;
use core::cmp::{max, Ordering};
use core::ops::{Add, AddAssign, Sub};

/// The maximum number of grid tracks (rows or columns) that Taffy will create in a single axis.
///
/// Grid lines are stored as `i16`, and a number of intermediate computations (line + span,
/// implicit-grid expansion, placement search, etc.) can produce values up to roughly
/// `3 * MAX_GRID_TRACKS`. Clamping both the number of explicit tracks and the magnitude of grid
/// line indices (and spans) to this value keeps all of those computations comfortably within
/// `i16` range, preventing the integer-overflow / out-of-range panics that pathologically large
/// grids would otherwise trigger.
///
/// The value (10,000) matches the limit used by other browser engines such as Firefox.
pub(crate) const MAX_GRID_TRACKS: u16 = 10_000;

/// The maximum (positive) grid line index, as an `i16`. Equal to [`MAX_GRID_TRACKS`].
const MAX_GRID_LINE: i16 = MAX_GRID_TRACKS as i16;

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
        // cannot overflow `i16`. The explicit track count is also clamped at its source, this is
        // a defensive belt-and-braces measure.
        let explicit_track_count = explicit_track_count.min(MAX_GRID_TRACKS);
        let explicit_line_count = explicit_track_count + 1;
        let oz_line = match self.0.cmp(&0) {
            Ordering::Greater => self.0 - 1,
            Ordering::Less => self.0 + explicit_line_count as i16,
            Ordering::Equal => panic!("Grid line of zero is invalid"),
        };
        // Clamp the resulting line to the maximum grid size. Without this a single grid item with a
        // very large (positive or negative) line index could force the implicit grid to grow past
        // `i16::MAX`, causing overflow panics elsewhere.
        OriginZeroLine(oz_line).clamp_to_max_grid_tracks()
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
impl Add<OriginZeroLine> for OriginZeroLine {
    type Output = Self;
    fn add(self, rhs: OriginZeroLine) -> Self::Output {
        OriginZeroLine(self.0 + rhs.0)
    }
}
impl Sub<OriginZeroLine> for OriginZeroLine {
    type Output = Self;
    fn sub(self, rhs: OriginZeroLine) -> Self::Output {
        OriginZeroLine(self.0 - rhs.0)
    }
}

// Add and Sub with u16
impl Add<u16> for OriginZeroLine {
    type Output = Self;
    fn add(self, rhs: u16) -> Self::Output {
        OriginZeroLine(self.0 + rhs as i16)
    }
}
impl AddAssign<u16> for OriginZeroLine {
    fn add_assign(&mut self, rhs: u16) {
        self.0 += rhs as i16;
    }
}
impl Sub<u16> for OriginZeroLine {
    type Output = Self;
    fn sub(self, rhs: u16) -> Self::Output {
        OriginZeroLine(self.0 - rhs as i16)
    }
}

impl OriginZeroLine {
    /// Clamp the line index to `±MAX_GRID_TRACKS` so that the implicit grid (and all the `i16`
    /// arithmetic that depends on line positions) cannot overflow `i16`.
    pub(crate) fn clamp_to_max_grid_tracks(self) -> Self {
        OriginZeroLine(self.0.clamp(-MAX_GRID_LINE, MAX_GRID_LINE))
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
}

/// A trait for the different coordinates used to define grid lines.
pub trait GridCoordinate: Copy {}
impl GridCoordinate for GridLine {}
impl GridCoordinate for OriginZeroLine {}
