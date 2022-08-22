//! Final and cached data structures that represent the high-level UI layout

use crate::geometry::{Point, Size};
use crate::sys::{abs};

/// The amount of space available to a node in a given axis
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AvailableSpace {
    /// The amount of space available is the specified number of pixels
    Definite(f32),
    /// The amount of space available is indefinite and the node should be laid out under a min-content constraint
    MinContent,
    /// The amount of space available is indefinite and the node should be laid out under a max-content constraint
    MaxContent,
}

impl AvailableSpace {
    const ZERO : AvailableSpace = AvailableSpace::Definite(0.0);

    /// Returns true for definite values, else false
    pub fn is_definite(self) -> bool {
        matches!(self, AvailableSpace::Definite(_))
    }

    /// Convert to Option
    /// Definite values become Some(value). Contraints become None.
    pub fn as_option(self) -> Option<f32> {
        match self {
            AvailableSpace::Definite(value) => Some(value),
            _ => None,
        }
    }

    /// Return the definite value or a default value
    pub fn unwrap_or(self, default: f32) -> f32 {
        self.as_option().unwrap_or(default)
    }

    /// Return the definite value. Panic is the value is not definite.
    #[track_caller]
    pub fn unwrap(self) -> f32 {
        self.as_option().unwrap()
    }

    /// Compare equality with another AvailableSpace, treating definite values
    /// that are within f32::EPSILON of each other as equal 
    pub fn is_roughly_equal(self, other: AvailableSpace) -> bool {
        use AvailableSpace::*;
        match (self, other)  {
            (Definite(a), Definite(b)) => abs(a - b) < f32::EPSILON,
            (MinContent, MinContent) => true,
            (MaxContent, MaxContent) => true,
            _ => false,
        }
    }
}

impl From<Option<f32>> for AvailableSpace {
    fn from(option: Option<f32>) -> Self {
        match option {
            Some(value) => Self::Definite(value),
            None => Self::MaxContent,
        }
    }
}

/// The final result of a layout algorithm for a single [`Node`](crate::node::Node).
#[derive(Copy, Debug, Clone)]
pub struct Layout {
    /// The relative ordering of the node
    ///
    /// Nodes with a higher order should be rendered on top of those with a lower order.
    /// This is effectively a topological sort of each tree.
    pub order: u32,
    /// The width and height of the node
    pub size: Size<f32>,
    /// The bottom-left corner of the node
    pub location: Point<f32>,
}

impl Layout {
    /// Creates a new [`Layout`] struct with zero size positioned at the origin
    #[must_use]
    pub(crate) fn new() -> Self {
        Self { order: 0, size: Size::ZERO, location: Point::ZERO }
    }
}

/// Cached intermediate layout results
#[derive(Debug, Clone)]
pub struct Cache {
    /// The initial cached size of the node itself
    pub(crate) node_size: Size<Option<f32>>,
    /// The initial cached size of the parent's node
    pub(crate) parent_size: Size<Option<f32>>,
    /// Whether or not layout should be recomputed
    pub(crate) perform_layout: bool,

    /// The cached size of the item
    pub(crate) size: Size<f32>,
}

/// Cached intermediate layout results
#[derive(Debug, Clone, Copy)]
pub struct AvailableSpaceCache {
    /// The available space constraint passed in when measuring the node
    pub(crate) constraint: Size<AvailableSpace>,
    /// The cached size of the item
    pub(crate) cached_size: Size<f32>,
}

impl AvailableSpaceCache {
    pub fn empty() -> Self {
        Self {
            constraint: Size { width: AvailableSpace::ZERO, height: AvailableSpace::ZERO },
            cached_size: Size { width: 0.0, height: 0.0 },
        }
    }
}
