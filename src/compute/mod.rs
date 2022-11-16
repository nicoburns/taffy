pub(crate) mod flexbox;
pub(crate) mod leaf;

use crate::error::TaffyError;
use crate::geometry::{Point, Size};
use crate::layout::{Layout, LayoutMode, SizingConstraint};
use crate::math::MaybeMath;
use crate::node::Node;
use crate::resolve::MaybeResolve;
use crate::style::Display;
use crate::sys::round;
use crate::tree::LayoutTree;

/// Updates the stored layout of the provided `node` and its children
fn compute_node_layout(
    tree: &mut impl LayoutTree,
    node: Node,
    sizing_constraint: Size<SizingConstraint>,
    layout_mode: LayoutMode,
    cache_slot: usize,
) -> Size<f32> {
    // clear the dirtiness of the node now that we've computed it
    tree.mark_dirty(node, false);

    // First we check if we have a cached result for the given input
    if let Some(cached_size) = compute_from_cache(tree, node, sizing_constraint) {
        return cached_size;
    }

    // Attempt to shortcut size computation based on
    //  - KnownSize sizing constraints
    //  - The node's preferred sizes (width/heights) styles and AvailableSpace sizing constraints
    // (percentages resolve to pixel values if there is a definite AvailableSpace sizing constraint)
    let style = tree.style(node);
    let known_node_size = sizing_constraint
        .zip_map(style.size, |constraint, style_size| constraint.maybe_resolve_nominal_size(style_size))
        .known_size();
    if known_node_size.width.is_some() && known_node_size.height.is_some() {
        let node_min_size = style.min_size.maybe_resolve(sizing_constraint.available_space());
        let node_max_size = style.max_size.maybe_resolve(sizing_constraint.available_space());
        return Size {
            width: known_node_size.width.maybe_max(node_min_size.width).maybe_min(node_max_size.width).unwrap_or(0.0),
            height: known_node_size
                .height
                .maybe_max(node_min_size.height)
                .maybe_min(node_max_size.height)
                .unwrap_or(0.0),
        };
    }

    println!("COMPUTE");

    // If this is a leaf node we can skip a lot of this function in some cases
    let computed_size = if tree.children(node).is_empty() {
        // println!("leaf");
        self::leaf::compute(tree, node, sizing_constraint, layout_mode)
    } else {
        // println!("match {:?}", tree.style(node).display);
        match tree.style(node).display {
            Display::Flex => self::flexbox::compute(tree, node, sizing_constraint, layout_mode),
            Display::None => Size { width: 0.0, height: 0.0 },
        }
    };

    // Cache result
    // tree.set_cache(node, cache_slot, sizing_constraint, computed_size);

    computed_size
}

/// Try to get the computation result from the cache.
#[inline]
fn compute_from_cache(
    tree: &mut impl LayoutTree,
    node: Node,
    sizing_constraint: Size<SizingConstraint>,
) -> Option<Size<f32>> {
    for entry in (0..5).map(|idx| tree.cache_mut(node, idx)) {
        if let Some(entry) = entry {
            if entry.sizing_constraint.width.is_roughly_equal(sizing_constraint.width)
                && entry.sizing_constraint.height.is_roughly_equal(sizing_constraint.height)
            {
                return Some(entry.cached_size);
            }
        }
    }

    None
}

/// Rounds the calculated [`NodeData`] according to the spec
fn round_layout(tree: &mut impl LayoutTree, root: Node, abs_x: f32, abs_y: f32) {
    let layout = tree.layout_mut(root);
    let abs_x = abs_x + layout.location.x;
    let abs_y = abs_y + layout.location.y;

    layout.location.x = round(layout.location.x);
    layout.location.y = round(layout.location.y);

    layout.size.width = round(layout.size.width);
    layout.size.height = round(layout.size.height);

    // Satisfy the borrow checker here by re-indexing to shorten the lifetime to the loop scope
    for x in 0..tree.children(root).len() {
        let child = tree.child(root, x);
        round_layout(tree, child, abs_x, abs_y);
    }
}

/// Updates the stored layout of the provided `node` and its children
pub fn compute_layout(
    tree: &mut impl LayoutTree,
    root: Node,
    available_space: Size<SizingConstraint>,
) -> Result<(), TaffyError> {
    // Recursively compute node layout
    let size = compute_node_layout(tree, root, available_space, 0);

    let layout = Layout { order: 0, size, location: Point::ZERO };
    *tree.layout_mut(root) = layout;

    // Recursively round the layout's of this node and all children
    round_layout(tree, root, 0.0, 0.0);

    Ok(())
}
