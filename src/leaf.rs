use crate::geometry::Size;
use crate::layout::{AvailableSpace, Cache, ClampMode, SizingMode};
use crate::math::MaybeMath;
use crate::node::Node;
use crate::resolve::{MaybeResolve, ResolveOrDefault};
use crate::tree::LayoutTree;

// Define some general constants we will need for the remainder of the algorithm.
// let mut constants = compute_constants(tree.style(node), node_size, available_space);

fn clamp_size(size: Size<f32>, min_size: Size<Option<f32>>, max_size: Size<Option<f32>>) -> Size<f32> {
    Size {
        width: size.width.maybe_max(min_size.width).maybe_min(max_size.width),
        height: size.height.maybe_max(min_size.height).maybe_min(max_size.height),
    }
}

fn override_size(base_size : Size<f32>, override_size: Size<Option<f32>>) -> Size<f32> {
  base_size.zip_map(override_size, |base_size, override_size| override_size.unwrap_or(base_size))
}

pub(crate) fn compute(
    tree: &mut impl LayoutTree,
    node: Node,
    available_space: Size<AvailableSpace>,
    size_override: Size<Option<f32>>,
    clamp_mode: ClampMode,
    sizing_mode: SizingMode,
    cache_slot: usize,
) -> Size<f32> {
    let style = tree.style(node);

    // Resolve node's preferred/min/max sizes (width/heights) against the available space
    // (percentages resolve to pixel values)
    let node_size = style
        .size
        .maybe_resolve(available_space.as_options())
        .zip_map(size_override, |style_size, size_override| size_override.or(style_size));
    let node_min_size = style.min_size.maybe_resolve(available_space.as_options());
    let node_max_size = style.max_size.maybe_resolve(available_space.as_options());

    // Debug logs
    println!("LEAF");
    dbg!(sizing_mode, clamp_mode);
    dbg!(available_space);
    dbg!(node_size);
    dbg!(node_min_size);
    dbg!(node_max_size);

    // If we are computing inherent size, and the node has a definite width and height
    // then simply return those.
    match (sizing_mode, node_size.width, node_size.height) {
      (SizingMode::InherentSize, Some(width), Some(height)) => {
          println!("A");
          return Size {
              width: width.maybe_max(node_min_size.width).maybe_min(node_max_size.width),
              height: height.maybe_max(node_min_size.height).maybe_min(node_max_size.height),
          };
      },
      (SizingMode::InherentSize, _, _) | (SizingMode::ContentSize, _, _) => {
        // Continue with full sizing algorithm
      },
    };

    // Compute the content size under the specified available space
    let content_size = if tree.needs_measure(node) {
        println!("B");

        // Compute available space
        let available_space = Size {
            width: available_space.width.maybe_set(node_size.width),
            height: available_space.height.maybe_set(node_size.height),
        };

        // First we check if we have a cached result for the given input
        match tree.find_in_cache(node, available_space) {

            // If we do then return cached result
            Some(cached_size) => cached_size,

            // If not, then compute, cache, and then return result
            None => {
              let measured_size = tree.measure_node(node, available_space);
              *tree.cache_entry_mut(node, cache_slot) = Some(Cache { constraint: available_space, cached_size: measured_size });
              measured_size
            }
        }
    } else {
      println!("C");
      let padding = style.padding.resolve_or_default(available_space.width.as_option());
      let border = style.border.resolve_or_default(available_space.width.as_option());
      Size {
          width: 0.0 + padding.horizontal_axis_sum() + border.horizontal_axis_sum(),
          height: 0.0 + padding.vertical_axis_sum() + border.vertical_axis_sum(),
      }
    };

    dbg!(content_size);

    // Adjust size according the size, min-size, and max-size if the SizingMode and ClampMode say that we should
    return match sizing_mode {
        SizingMode::ContentSize => content_size,
        SizingMode::InherentSize => {
          let unclamped_size = override_size(content_size, node_size);
          match clamp_mode {
            ClampMode::NoClamp => unclamped_size,
            ClampMode::Clamp => clamp_size(unclamped_size, node_min_size, node_max_size),
          }
        },
    };
}
