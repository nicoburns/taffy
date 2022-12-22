//! Alignment of tracks and final positioning of items
use super::types::GridTrack;
use crate::axis::InBothAbsAxis;
use crate::compute::common::alignment::compute_alignment_offset;
use crate::compute::compute_node_layout;
use crate::geometry::{Line, Point, Rect, Size};
use crate::layout::{Layout, RunMode, SizingMode};
use crate::node::Node;
use crate::resolve::MaybeResolve;
use crate::style::{AlignContent, AlignItems, AlignSelf, AvailableSpace, PositionType};
use crate::sys::{f32_max, f32_min};
use crate::tree::LayoutTree;

/// Align the grid tracks within the grid according to the align-content (rows) or
/// justify-content (columns) property. This only does anything if the size of the
/// grid is not equal to the size of the grid container in the axis being aligned.
pub(super) fn align_tracks(
    grid_container_content_box_size: f32,
    padding: Line<f32>,
    border: Line<f32>,
    tracks: &mut [GridTrack],
    track_alignment_style: AlignContent,
) {
    let used_size: f32 = tracks.iter().map(|track| track.base_size).sum();
    let size_diff = grid_container_content_box_size - used_size;
    let free_space = f32_max(size_diff, 0.0);
    let overflow = f32_min(size_diff, 0.0);

    // If the used_size > grid_container_size then the tracks must overflow their container
    // The direction in which they do so is determined by the alignment style
    let origin = padding.start
        + border.start
        + match track_alignment_style {
            AlignContent::Start => 0.0,
            AlignContent::End => overflow,
            AlignContent::Center => overflow / 2.0,
            AlignContent::Stretch => 0.0,
            AlignContent::SpaceBetween => 0.0,
            AlignContent::SpaceEvenly => 0.0,
            AlignContent::SpaceAround => 0.0,
        };

    // Count the number of non-collapsed tracks (not counting gutters)
    let num_tracks = tracks.iter().skip(1).step_by(2).filter(|track| !track.is_collapsed).count();

    // Grid layout treats gaps as full tracks rather than applying them at alignment so we
    // simply pass zero here. Grid layout is never reversed.
    let gap = 0.0;
    let layout_is_reversed = false;

    // Compute offsets
    let mut total_offset = origin;
    tracks.iter_mut().enumerate().for_each(|(i, track)| {
        // Odd tracks are gutters (but slices are zero-indexed, so odd tracks have even indicies)
        let is_gutter = i % 2 == 0;

        // The first non-gutter track is index 1
        let is_first = i == 1;

        let offset = if is_gutter {
            0.0
        } else {
            compute_alignment_offset(free_space, num_tracks, gap, track_alignment_style, layout_is_reversed, is_first)
        };

        track.offset = total_offset + offset;
        total_offset = total_offset + offset + track.base_size;
    });
}

/// Align and size a grid item into it's final position
pub(super) fn align_and_position_item(
    tree: &mut impl LayoutTree,
    node: Node,
    order: u32,
    grid_area: Rect<f32>,
    container_content_box: Size<f32>,
    alignment_styles: InBothAbsAxis<Option<AlignItems>>,
) {
    let grid_area_size = Size { width: grid_area.right - grid_area.left, height: grid_area.bottom - grid_area.top };

    let style = tree.style(node);
    let aspect_ratio = style.aspect_ratio;
    let justify_self = style.justify_self;
    let align_self = style.align_self;

    let position_type = style.position_type;
    let inset_horizontal =
        style.position.horizontal_components().map(|size| size.resolve_to_option(container_content_box.width));
    let inset_vertical =
        style.position.vertical_components().map(|size| size.resolve_to_option(container_content_box.height));
    let inherent_size = style.size.maybe_resolve(container_content_box);

    // Note: This is not a bug. It is part of the CSS spec that both horizontal and vertical margins
    // resolve against the WIDTH of the grid area.
    let margin = style.margin.map(|margin| margin.resolve_to_option(grid_area_size.width));
    let grid_area_minus_item_margins_size = Size {
        width: grid_area_size.width - margin.left.unwrap_or(0.0) - margin.right.unwrap_or(0.0),
        height: grid_area_size.height - margin.top.unwrap_or(0.0) - margin.bottom.unwrap_or(0.0),
    };

    // If node is absolutely positioned and width is not set explicitly, then deduce it
    // from left, right and container_content_box if both are set.
    let width = inherent_size.width.or_else(|| {
        if position_type == PositionType::Absolute {
            if let (Some(left), Some(right)) = (inset_horizontal.start, inset_horizontal.end) {
                return Some(f32_max(grid_area_size.width - left - right, 0.0));
            }
        }
        None
    });
    let height = inherent_size.height.or_else(|| {
        if position_type == PositionType::Absolute {
            if let (Some(top), Some(bottom)) = (inset_vertical.start, inset_vertical.end) {
                return Some(f32_max(grid_area_size.height - top - bottom, 0.0));
            }
        }
        None
    });

    // Layout node
    let measured_size = compute_node_layout(
        tree,
        node,
        Size { width, height },
        grid_area_minus_item_margins_size.map(|size| AvailableSpace::Definite(size)),
        RunMode::PeformLayout,
        SizingMode::InherentSize,
    );

    let (x, width) = align_and_size_item_within_area(
        Line { start: grid_area.left, end: grid_area.right },
        justify_self.or(alignment_styles.horizontal),
        width,
        measured_size.width,
        position_type,
        inset_horizontal,
        margin.horizontal_components(),
        aspect_ratio,
    );
    let (y, height) = align_and_size_item_within_area(
        Line { start: grid_area.top, end: grid_area.bottom },
        align_self.or(alignment_styles.vertical),
        height,
        measured_size.height,
        position_type,
        inset_vertical,
        margin.vertical_components(),
        aspect_ratio,
    );

    *tree.layout_mut(node) = Layout { order, size: Size { width, height }, location: Point { x, y } };
}

/// Align and size a grid item along a single axis
pub(super) fn align_and_size_item_within_area(
    grid_area: Line<f32>,
    alignment_style: Option<AlignSelf>,
    style_size: Option<f32>,
    measured_size: f32,
    position_type: PositionType,
    inset: Line<Option<f32>>,
    margin: Line<Option<f32>>,
    aspect_ratio: Option<f32>,
) -> (f32, f32) {
    // Calculate grid area dimension in the axis
    let non_auto_margin = Line { start: margin.start.unwrap_or(0.0), end: margin.end.unwrap_or(0.0) };
    let grid_area_size = f32_max(grid_area.end - grid_area.start, 0.0);
    let free_space = f32_max(grid_area_size - style_size.unwrap_or(measured_size) - non_auto_margin.sum(), 0.0);

    // Expand auto margins to fill available space
    let auto_margin_count = margin.start.is_none() as u8 + margin.end.is_none() as u8;
    let auto_margin_size = if auto_margin_count > 0 { free_space / auto_margin_count as f32 } else { 0.0 };
    let resolved_margin =
        Line { start: margin.start.unwrap_or(auto_margin_size), end: margin.end.unwrap_or(auto_margin_size) };

    // Compute default alignment style if it set on neither the parent or the node itself
    let alignment_style = alignment_style.unwrap_or_else(|| {
        if style_size.is_some() || aspect_ratio.is_some() {
            AlignSelf::Start
        } else {
            AlignSelf::Stretch
        }
    });

    // Compute size in the axis
    let size = style_size.unwrap_or_else(|| {
        if alignment_style == AlignItems::Stretch && position_type != PositionType::Absolute {
            f32_max(grid_area_size - resolved_margin.sum(), measured_size)
        } else {
            measured_size
        }
    });

    // Compute offset in the axis
    let alignment_based_offset = match alignment_style {
        AlignSelf::Start => resolved_margin.start,
        AlignSelf::End => grid_area_size - size - resolved_margin.end,
        AlignSelf::Center => (grid_area_size - size + resolved_margin.start - resolved_margin.end) / 2.0,
        // TODO: Add support for baseline alignment. For now we treat it as "start".
        AlignSelf::Baseline => resolved_margin.start,
        AlignSelf::Stretch => resolved_margin.start,
    };

    let offset_within_area = if position_type == PositionType::Absolute {
        if let Some(start) = inset.start {
            start + non_auto_margin.start
        } else if let Some(end) = inset.end {
            grid_area_size - end - size - non_auto_margin.end
        } else {
            alignment_based_offset
        }
    } else {
        alignment_based_offset
    };

    let mut start = grid_area.start + offset_within_area;
    if position_type == PositionType::Relative {
        start += inset.start.or(inset.end.map(|pos| -pos)).unwrap_or(0.0);
    }

    (start, size)
}
