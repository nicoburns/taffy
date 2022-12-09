//! Axis enums representing CSS Grid axis
use crate::geometry::Size;

/// The simple absolute horizontal and vertical axis
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AbsoluteAxis {
    /// The horizontal axis
    Horizontal,
    /// The vertical axis
    Vertical,
}

impl AbsoluteAxis {
    /// Returns the other variant of the enum
    #[inline]
    pub const fn other_axis(&self) -> Self {
        match *self {
            AbsoluteAxis::Horizontal => AbsoluteAxis::Vertical,
            AbsoluteAxis::Vertical => AbsoluteAxis::Horizontal,
        }
    }
}

impl<T> Size<T> {
    #[inline(always)]
    /// Get either the width or height depending on the AbsoluteAxis passed in
    pub fn get_abs(self, axis: AbsoluteAxis) -> T {
        match axis {
            AbsoluteAxis::Horizontal => self.width,
            AbsoluteAxis::Vertical => self.height,
        }
    }
}
