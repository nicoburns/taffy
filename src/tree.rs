//! The baseline requirements of any UI Tree so Taffy can efficiently calculate the layout

use crate::{
    layout::{Cache, Layout, SizeAndBaselines, SizingMode},
    prelude::*,
};
use core::fmt::Debug;

/// Trait alias for Copy + PartialEq + Debug
pub trait ChildIdBounds: Copy + PartialEq + Debug {}
impl<T: Copy + PartialEq + Debug> ChildIdBounds for T {}

/// Any item that implements the LayoutTree can be layed out using Taffy's algorithms.
///
/// Generally, Taffy expects your Node tree to be indexable by stable indices. A "stable" index means that the Node's ID
/// remains the same between re-layouts.
pub trait LayoutTree<ChildId: ChildIdBounds> {
    /// Type for the reborrow method. Normally this will be the same type you are implementing this trait for,
    /// but with a shorter lifegsttime
    type Reborrow<'a>: LayoutTree<ChildId>
    where
        Self: 'a;

    /// Type representing an iterator of the children of a node
    type ChildIter<'a>: Iterator<Item = ChildId>
    where
        Self: 'a;

    // Current node methods

    /// Reborrow the node
    fn reborrow<'a, 'this: 'a>(&'this mut self) -> Self::Reborrow<'a>
    where
        'this: 'a;

    /// Get the [`Style`] for this Node.
    fn style(&self) -> &Style;

    /// Modify the node's output layout
    fn layout_mut(&mut self) -> &mut Layout;

    /// Get a cache entry for this Node by index
    fn cache_mut(&mut self, index: usize) -> &mut Option<Cache>;

    // Child methods

    /// Get the list of children IDs for the given node
    fn children(&self) -> Self::ChildIter<'_>;

    /// Get the number of children for the given node
    fn child_count(&self) -> usize;

    /// Get a specific child of a node, where the index represents the nth child
    fn child(&self, index: usize) -> ChildId;

    /// Get the [`Style`] for this child.
    fn child_style(&self, child_node_id: ChildId) -> &Style;

    /// Modify the child's output layout
    fn child_layout_mut(&mut self, child_node_id: ChildId) -> &mut Layout;

    /// Compute the size of the node given the specified constraints
    fn measure_child_size(
        &mut self,
        child_node_id: ChildId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> Size<f32>;

    /// Perform a full layout on the node given the specified constraints
    fn perform_child_layout(
        &mut self,
        child_node_id: ChildId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        sizing_mode: SizingMode,
    ) -> SizeAndBaselines;

    /// Perform a hidden layout (mark the node as invisible)
    fn perform_child_hidden_layout(&mut self, child_node_id: ChildId, order: u32);
}
