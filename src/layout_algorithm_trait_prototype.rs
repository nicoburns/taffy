//! The baseline requirements of any UI Tree so Taffy can efficiently calculate the layout
#![allow(dead_code)]
#![allow(unused_variables)]

/// Whether we are performing a full layout, or we merely need to size the node
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RunMode {
    /// A full layout for this node and all children should be computed
    PeformLayout,
    /// The layout algorithm should be executed such that an accurate container size for the node can be determined.
    /// Layout steps that aren't necessary for determining the container size of the current node can be skipped.
    ComputeSize,
}

/// Whether styles should be taken into account when computing size
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SizingMode {
    /// Only content contributions should be taken into account
    ContentSize,
    /// Inherent size styles should be taken into account in addition to content contributions
    InherentSize,
}

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

/// Represents a width and a height
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Size<T> {
    /// The x extent of the rectangle
    pub width: T,
    /// The y extent of the rectangle
    pub height: T,
}

/// A 2-dimensional coordinate.
/// When used in association with a [`Rect`], represents the bottom-left corner.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point<T> {
    /// The x-coordinate
    pub x: T,
    /// The y-coordinate
    pub y: T,
}

/// The final result of a layout algorithm for a single [`Node`](crate::node::Node).
#[derive(Copy, Debug, Clone)]
pub struct Layout {
    /// The relative ordering of the node
    /// Nodes with a higher order should be rendered on top of those with a lower order.
    /// This is effectively a topological sort of each tree.
    pub order: u32,
    /// The width and height of the node
    pub size: Size<f32>,
    /// The bottom-left corner of the node
    pub location: Point<f32>,
}

/// Cached intermediate layout results
#[derive(Debug, Clone, Copy)]
pub struct Cache {
    /// The initial cached size of the node itself
    pub(crate) known_dimensions: Size<Option<f32>>,
    /// The initial cached size of the parent's node
    pub(crate) available_space: Size<AvailableSpace>,
    /// Whether or not layout should be recomputed
    pub(crate) run_mode: RunMode,
    /// The cached size of the item
    pub(crate) cached_size: Size<f32>,
}

/// Indicates which dimensions depend on each other for a given node
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DimensionDependencies {
    /// The height of the node depends on the width, but the width does not depend on the height
    HeightDependsOnWidth,
    /// The width of the node depends on the height, but the height does not depend on the width
    WidthDependsOnHeight,
    /// The height and the width of the node are completely independent of each other
    Neither,
    /// The height and the width both depend on each other.
    /// This variant should also be used as a default if the dependencies are unknown.
    Both,
}

/// Any type implementing LayoutAlgorithm can be used by Taffy to compute a Node's layout
pub trait LayoutAlgorithm {
    /// Measure the size of this node. Taffy uses this to force reflows of things like text and overflowing content.
    fn measure<Node: LayoutNode>(
        node: &mut Node,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> Size<f32>;

    /// Perform full recursive layout of this node
    fn perform_layout<Node: LayoutNode>(
        node: &mut Node,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> Size<f32>;

    /// Opt-in performance optimization. If this function returns true the cached results from `measure`
    /// will be assumed to be valid for requests to `perform_layout`.
    /// A layout algorithm should only opt-in to this optimization if both:
    ///   - the work performed by `measure` and `perform_layout` is 100% identical
    ///   - Computing this value is cheap
    fn measurement_requires_full_layout<Node: LayoutNode>(node: &mut Node) -> bool {
        false
    }

    /// Opt-in performance optimization. Values other than `Both` allow layout algorithms
    /// to skip recomputations in some cases.
    fn dimension_dependencies<Node: LayoutNode>(node: &mut Node) -> DimensionDependencies {
        DimensionDependencies::Both
    }
}

/// Any item that implements Node can be laid out using Taffy's algorithms.
pub trait LayoutNode {
    /// Get the number of children for this node
    fn child_count(&self) -> usize;

    /// Get an immutable reference to a new instance of Self (the type implementing LayoutNode)
    fn child(&self, child_index: usize) -> &Self;

    /// Get a mutable reference to a new instance of Self (the type implementing LayoutNode)
    fn child_mut(&mut self, child_index: usize) -> &mut Self;

    /// Get the style of the type specified by the generic parameter for this node
    fn style<T: Clone>(&self) -> Option<&T>;

    /// Get the node's output "Final Layout"
    fn layout(&self) -> &Layout;

    /// Modify the node's output layout
    fn layout_mut(&mut self) -> &mut Layout;

    /// Get a cache entry for this node by index
    fn cache_mut(&mut self, index: usize) -> &mut Option<Cache>;

    /// See LayoutAlgorithm trait for documentation
    fn measure(
        &mut self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> Size<f32>;

    /// See LayoutAlgorithm trait for documentation
    fn perform_layout(
        &mut self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> Size<f32>;

    /// See LayoutAlgorithm trait for documentation
    fn measurement_requires_full_layout(&mut self) -> bool {
        false
    }

    /// See LayoutAlgorithm trait for documentation
    fn dimension_dependencies(&mut self) -> DimensionDependencies {
        DimensionDependencies::Both
    }
}
