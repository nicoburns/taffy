//! Node Data - important layout and styling data for nodes
//!
//! Used to compute layout for Taffy trees
//!
use crate::layout::{Cache, Layout, AvailableSpace};
use crate::geometry::Size;
use crate::style::FlexboxLayout;

/// Layout information for a given [`Node`](crate::node::Node)
///
/// Stored in a [`Taffy`].
pub(crate) struct NodeData {
    /// The layout strategy used by this node
    pub(crate) style: FlexboxLayout,
    /// The results of the layout computation
    pub(crate) layout: Layout,

    /// Should we try and measure this node?
    pub(crate) needs_measure: bool,

    /// A bunch of cache slots, mapping a Size<AvailableSpace> to a 
    pub(crate) intrinsic_size_cache: [Option<Cache>; 4],

    /// Does this node's layout need to be recomputed?
    pub(crate) is_dirty: bool,
}

impl NodeData {
    /// Create the data for a new node
    #[must_use]
    pub fn new(style: FlexboxLayout) -> Self {
        Self {
            style,
            intrinsic_size_cache: [Some(Cache::empty()); 4],
            layout: Layout::new(),
            is_dirty: true,
            needs_measure: false,
        }
    }

    /// Marks a node and all of its parents (recursively) as dirty
    ///
    /// This clears any cached data and signals that the data must be recomputed.
    #[inline]
    pub fn mark_dirty(&mut self) {
        self.intrinsic_size_cache = [Some(Cache::empty()); 4];
        self.is_dirty = true;
    }

    pub fn find_cache(&self, constraint: Size<AvailableSpace>) -> Option<Cache> {
        self.intrinsic_size_cache.iter().copied().find(|entry| match entry {
            Some(entry) => entry.constraint == constraint,
            None => false
        }).flatten()
    }

    pub fn set_cache(&mut self, index: usize, constraint: Size<AvailableSpace>, size: Size<f32>) {
        self.intrinsic_size_cache[index] = Some(Cache { constraint, cached_size: size })
    }
}
