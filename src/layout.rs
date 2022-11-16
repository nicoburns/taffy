//! Final and cached data structures that represent the high-level UI layout

use crate::geometry::{Point, Size};
use crate::style::Dimension;
use crate::sys::abs;

/// Whether we are performing a full layout, or we merely need to size the node
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LayoutMode {
    /// A full layout for this node and all children should be computed
    FullLayout,
    /// The layout algorithm should be executed such that an accurate container size for the node can be determined.
    /// Layout steps that aren't necessary for determining the container size of the current node can be skipped.
    ContainerSize,
}

/// The amount of space available to a node in a given axis
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SizingConstraint {
    /// A fixed (already computed) dimension, which can be used to compute the other dimension
    KnownSize(f32),
    /// The amount of space available is the specified number of pixels
    AvailableSpace(f32),
    /// The amount of space available is indefinite and the node should be laid out under a min-content constraint
    MinContent,
    /// The amount of space available is indefinite and the node should be laid out under a max-content constraint
    MaxContent,
}

impl SizingConstraint {
    /// Returns true for KnownSize values, else false
    pub fn is_known(self) -> bool {
        matches!(self, SizingConstraint::KnownSize(_))
    }

    /// Returns true for definite AvailableSpace values, else false
    pub fn is_definite(self) -> bool {
        matches!(self, SizingConstraint::AvailableSpace(_))
    }

    /// Returns true for definite AvailableSpace values, else false
    pub fn maybe_resolve_nominal_size(self, size: Dimension) -> Self {
        use Dimension::*;
        use SizingConstraint::*;

        match (self, size) {
            (KnownSize(px), _) => KnownSize(px),
            (_, Points(px)) => KnownSize(px),
            (AvailableSpace(space), Percent(percentage)) => KnownSize(space * percentage),
            _ => self,
        }
    }

    /// Get AvailableSpace value (if any)
    /// Definite AvailableSpace values become Some(value). KnownSize and min/max size constaints become None.
    pub fn available_space(self) -> Option<f32> {
        match self {
            SizingConstraint::AvailableSpace(value) => Some(value),
            _ => None,
        }
    }

    /// Get AvailableSpace value (if any)
    /// Definite KnownSize values become Some(value). AvailableSpace and min/max size constaints become None.
    pub fn known_size(self) -> Option<f32> {
        match self {
            SizingConstraint::KnownSize(value) => Some(value),
            _ => None,
        }
    }

    // /// Return the definite value or a default value
    // pub fn unwrap_or(self, default: f32) -> f32 {
    //     self.as_option().unwrap_or(default)
    // }

    // /// Return the definite value. Panic is the value is not definite.
    // #[track_caller]
    // pub fn unwrap(self) -> f32 {
    //     self.as_option().unwrap()
    // }

    // /// If passed value is Some then return SizingConstraint::Definite containing that value, else return self
    // pub fn maybe_set(self, value: Option<f32>) -> SizingConstraint {
    //     match value {
    //         Some(value) => SizingConstraint::Definite(value),
    //         None => self,
    //     }
    // }

    /// Compare equality with another SizingConstraint, treating definite values
    /// that are within f32::EPSILON of each other as equal
    pub fn is_roughly_equal(self, other: SizingConstraint) -> bool {
        use SizingConstraint::*;
        match (self, other) {
            (KnownSize(a), KnownSize(b)) => abs(a - b) < f32::EPSILON,
            (AvailableSpace(a), AvailableSpace(b)) => abs(a - b) < f32::EPSILON,
            (MinContent, MinContent) => true,
            (MaxContent, MaxContent) => true,
            _ => false,
        }
    }
}

impl Size<SizingConstraint> {
    pub fn max_content() -> Size<SizingConstraint> {
        Size { width: SizingConstraint::MaxContent, height: SizingConstraint::MaxContent }
    }

    pub fn min_content() -> Size<SizingConstraint> {
        Size { width: SizingConstraint::MinContent, height: SizingConstraint::MinContent }
    }

    pub fn is_fully_known(&self) -> bool {
        self.width.is_known() && self.height.is_known()
    }

    pub fn available_space(&self) -> Size<Option<f32>> {
        self.map(|constraint| constraint.available_space())
    }

    pub fn known_size(&self) -> Size<Option<f32>> {
        self.map(|constraint| constraint.known_size())
    }

    pub fn maybe_resolve_nominal_size(self, size: Size<Dimension>) -> Self {
        self.zip_map(size, |constraint, size| constraint.maybe_resolve_nominal_size(size))
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
    /// Creates a new zero-[`Layout`].
    ///
    /// The Zero-layout has size and location set to ZERO.
    /// The `order` value of this layout is set to the minimum value of 0.
    /// This means it should be rendered below all other [`Layout`]s.
    #[must_use]
    pub const fn new() -> Self {
        Self { order: 0, size: Size::ZERO, location: Point::ZERO }
    }

    /// Creates a new zero-[`Layout`] with the supplied `order` value.
    ///
    /// Nodes with a higher order should be rendered on top of those with a lower order.
    /// The Zero-layout has size and location set to ZERO.
    #[must_use]
    pub const fn with_order(order: u32) -> Self {
        Self { order, size: Size::ZERO, location: Point::ZERO }
    }
}

/// Cached intermediate layout results
#[derive(Debug, Clone)]
pub struct Cache {
    /// The sizing constraint passed in when measuring the node
    pub(crate) sizing_constraint: Size<SizingConstraint>,
    /// The cached size of the item
    pub(crate) cached_size: Size<f32>,
}
