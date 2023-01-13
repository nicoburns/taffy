//! The baseline requirements of any UI Tree so Taffy can efficiently calculate the layout

use slotmap::DefaultKey;

use crate::{
    layout::{Cache, Layout, SizeAndBaselines, SizingMode},
    prelude::*,
};

/// Any item that implements the LayoutTree can be layed out using Taffy's algorithms.
///
/// Generally, Taffy expects your Node tree to be indexable by stable indices. A "stable" index means that the Node's ID
/// remains the same between re-layouts.
pub trait LayoutTree {
    /// Type representing an iterator of the children of a node
    type ChildIter<'a>: Iterator<Item = &'a DefaultKey>
    where
        Self: 'a;

    /// Get the ID for the given node
    fn current_node_id(&self) -> Node;

    /// Get the list of children IDs for the given node
    fn children(&self) -> Self::ChildIter<'_>;

    /// Get the number of children for the given node
    fn child_count(&self) -> usize;

    /// Returns true if the node has no children
    fn is_childless(&self) -> bool;

    /// Get a specific child of a node, where the index represents the nth child
    fn child(&self, index: usize) -> Node;

    // todo: allow abstractions over this so we don't prescribe how layout works
    // for reference, CSS cascades require context, and storing a full flexbox layout for each node could be inefficient
    //
    /// Get the [`Style`] for this Node.
    fn style(&self) -> &Style;

    /// Get the node's output "Final Layout"
    fn layout(&self) -> &Layout;

    /// Modify the node's output layout
    fn layout_mut(&mut self) -> &mut Layout;

    /// Get the [`Style`] for this child.
    fn child_style(&self, child_node_id: Node) -> &Style;

    /// Get the child's output "Final Layout"
    fn child_layout(&self, child_node_id: Node) -> &Layout;

    /// Modify the child's output layout
    fn child_layout_mut(&mut self, child_node_id: Node) -> &mut Layout;

    /// Measure a node. Taffy uses this to force reflows of things like text and overflowing content.
    fn measure_node(&self, known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>) -> Size<f32>;

    /// Node needs to be measured
    fn needs_measure(&self) -> bool;

    /// Get a cache entry for this Node by index
    fn cache_mut(&mut self, index: usize) -> &mut Option<Cache>;

    /// Compute the size of the node given the specified constraints
    fn measure_child_size(
        &mut self,
        child_node_id: Node,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> Size<f32>;

    /// Perform a full layout on the node given the specified constraints
    fn perform_child_layout(
        &mut self,
        child_node_id: Node,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> SizeAndBaselines;

    /// Perform a hidden layout (mark the node as invisible)
    fn perform_child_hidden_layout(&mut self, child_node_id: Node, order: u32);
}
