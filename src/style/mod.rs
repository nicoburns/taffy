//! A representation of [CSS layout properties](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) in Rust, used for flexbox layout

mod alignment;
mod dimension;
mod dimension_helpers;
mod flex;

pub use alignment::*;
pub use dimension::*;
pub use dimension_helpers::*;
pub use flex::*;

use crate::geometry::{Rect, Size};

/// Sets the layout used for the children of this node
///
/// [`Display::Flex`] is the default value.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Display {
    /// The children will follow the flexbox layout algorithm
    Flex,
    /// The children will not be laid out, and will follow absolute positioning
    None,
}

impl Default for Display {
    fn default() -> Self {
        Self::Flex
    }
}

/// The positioning strategy for this item.
///
/// This controls both how the origin is determined for the [`Style::position`] field,
/// and whether or not the item will be controlled by flexbox's layout algorithm.
///
/// WARNING: this enum follows the behavior of [CSS's `position` property](https://developer.mozilla.org/en-US/docs/Web/CSS/position),
/// which can be unintuitive.
///
/// [`PositionType::Relative`] is the default value, in contrast to the default behavior in CSS.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PositionType {
    /// The offset is computed relative to the final position given by the layout algorithm.
    /// Offsets do not affect the position of any other items; they are effectively a correction factor applied at the end.
    Relative,
    /// The offset is computed relative to this item's closest positioned ancestor, if any.
    /// Otherwise, it is placed relative to the origin.
    /// No space is created for the item in the page layout, and its size will not be altered.
    ///
    /// WARNING: to opt-out of layouting entirely, you must use [`Display::None`] instead on your [`Style`] object.
    Absolute,
}

impl Default for PositionType {
    fn default() -> Self {
        Self::Relative
    }
}

/// The flexbox layout information for a single [`Node`](crate::node::Node).
///
/// The most important idea in flexbox is the notion of a "main" and "cross" axis, which are always perpendicular to each other.
/// The orientation of these axes are controlled via the [`FlexDirection`] field of this struct.
///
/// This struct follows the [CSS equivalent](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Flexible_Box_Layout/Basic_Concepts_of_Flexbox) directly;
/// information about the behavior on the web should transfer directly.
///
/// Detailed information about the exact behavior of each of these fields
/// can be found on [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS) by searching for the field name.
/// The distinction between margin, padding and border is explained well in
/// this [introduction to the box model](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Box_Model/Introduction_to_the_CSS_box_model).
///
/// If the behavior does not match the flexbox layout algorithm on the web, please file a bug!
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Style {
    /// What layout strategy should be used?
    pub display: Display,
    /// What should the `position` value of this struct use as a base offset?
    pub position_type: PositionType,
    /// Which direction does the main axis flow in?
    pub flex_direction: FlexDirection,
    /// Should elements wrap, or stay in a single line?
    pub flex_wrap: FlexWrap,
    /// How should items be aligned relative to the cross axis?
    pub align_items: AlignItems,
    /// Should this item violate the cross axis alignment specified by its parent's [`AlignItems`]?
    pub align_self: AlignSelf,
    /// How should content contained within this item be aligned relative to the cross axis?
    pub align_content: AlignContent,
    /// How should items be aligned relative to the main axis?
    pub justify_content: JustifyContent,
    /// How should the position of this element be tweaked relative to the layout defined?
    pub position: Rect<LengthPercentageAuto>,
    /// How large should the margin be on each side?
    pub margin: Rect<LengthPercentageAuto>,
    /// How large should the padding be on each side?
    pub padding: Rect<LengthPercentage>,
    /// How large should the border be on each side?
    pub border: Rect<LengthPercentage>,
    // Gap
    /// How large should the gaps between items in a grid or flex container be?
    pub gap: Size<LengthPercentage>,
    /// The relative rate at which this item grows when it is expanding to fill space
    ///
    /// 0.0 is the default value, and this value must not be negative.
    pub flex_grow: f32,
    /// The relative rate at which this item shrinks when it is contracting to fit into space
    ///
    /// 1.0 is the default value, and this value must not be negative.
    pub flex_shrink: f32,
    /// Sets the initial main axis size of the item
    pub flex_basis: Dimension,
    /// Sets the initial size of the item
    // TODO: why does this exist as distinct from flex_basis? How do they interact?
    pub size: Size<Dimension>,
    /// Controls the minimum size of the item
    pub min_size: Size<Dimension>,
    /// Controls the maximum size of the item
    pub max_size: Size<Dimension>,
    /// Sets the preferred aspect ratio for the item
    ///
    /// The ratio is calculated as width divided by height.
    pub aspect_ratio: Option<f32>,
}

impl Default for Style {
    fn default() -> Self {
        Style::DEFAULT
    }
}

impl Style {
    /// The [`Default`] layout, in a form that can be used in const functions
    pub const DEFAULT: Style = Style {
        display: Display::Flex,
        position_type: PositionType::Relative,
        flex_direction: FlexDirection::Row,
        flex_wrap: FlexWrap::NoWrap,
        align_items: AlignItems::Stretch,
        align_self: AlignSelf::Auto,
        align_content: AlignContent::Stretch,
        justify_content: JustifyContent::FlexStart,
        position: Rect::auto(),
        margin: Rect::zero(),
        padding: Rect::zero(),
        border: Rect::zero(),
        gap: Size::zero(),
        flex_grow: 0.0,
        flex_shrink: 1.0,
        flex_basis: Dimension::Auto,
        size: Size::auto(),
        min_size: Size::auto(),
        max_size: Size::auto(),
        aspect_ratio: None,
    };

    /// Computes the final alignment of this item based on the parent's [`AlignItems`] and this item's [`AlignSelf`]
    pub(crate) fn align_self(&self, parent: &Style) -> AlignSelf {
        // FUTURE WARNING: This function should never return AlignSelf::Auto
        // See #169 https://github.com/DioxusLabs/taffy/pull/169#issuecomment-1157698840

        if self.align_self == AlignSelf::Auto {
            match parent.align_items {
                AlignItems::FlexStart => AlignSelf::FlexStart,
                AlignItems::FlexEnd => AlignSelf::FlexEnd,
                AlignItems::Center => AlignSelf::Center,
                AlignItems::Baseline => AlignSelf::Baseline,
                AlignItems::Stretch => AlignSelf::Stretch,
            }
        } else {
            self.align_self
        }
    }
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {
    use super::Style;
    use crate::geometry::{Rect, Size};

    #[test]
    fn defaults_match() {
        let old_defaults = Style {
            display: Default::default(),
            position_type: Default::default(),
            flex_direction: Default::default(),
            flex_wrap: Default::default(),
            align_items: Default::default(),
            align_self: Default::default(),
            align_content: Default::default(),
            justify_content: Default::default(),
            position: Rect::auto(),
            margin: Rect::zero(),
            padding: Rect::zero(),
            border: Rect::zero(),
            gap: Size::zero(),
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: super::Dimension::Auto,
            size: Size::auto(),
            min_size: Size::auto(),
            max_size: Size::auto(),
            aspect_ratio: Default::default(),
        };

        assert_eq!(Style::DEFAULT, Style::default());
        assert_eq!(Style::DEFAULT, old_defaults);
    }

    mod test_flex_direction {
        use crate::style::*;

        #[test]
        fn flex_direction_is_row() {
            assert_eq!(FlexDirection::Row.is_row(), true);
            assert_eq!(FlexDirection::RowReverse.is_row(), true);
            assert_eq!(FlexDirection::Column.is_row(), false);
            assert_eq!(FlexDirection::ColumnReverse.is_row(), false);
        }

        #[test]
        fn flex_direction_is_column() {
            assert_eq!(FlexDirection::Row.is_column(), false);
            assert_eq!(FlexDirection::RowReverse.is_column(), false);
            assert_eq!(FlexDirection::Column.is_column(), true);
            assert_eq!(FlexDirection::ColumnReverse.is_column(), true);
        }

        #[test]
        fn flex_direction_is_reverse() {
            assert_eq!(FlexDirection::Row.is_reverse(), false);
            assert_eq!(FlexDirection::RowReverse.is_reverse(), true);
            assert_eq!(FlexDirection::Column.is_reverse(), false);
            assert_eq!(FlexDirection::ColumnReverse.is_reverse(), true);
        }
    }

    mod test_flexbox_layout {
        use crate::style::*;

        fn layout_from_align_items(align: AlignItems) -> Style {
            Style { align_items: align, ..Default::default() }
        }

        fn layout_from_align_self(align: AlignSelf) -> Style {
            Style { align_self: align, ..Default::default() }
        }

        #[test]
        fn flexbox_layout_align_self_auto() {
            let parent = layout_from_align_items(AlignItems::FlexStart);
            let layout = layout_from_align_self(AlignSelf::Auto);
            assert_eq!(layout.align_self(&parent), AlignSelf::FlexStart);

            let parent = layout_from_align_items(AlignItems::FlexEnd);
            let layout = layout_from_align_self(AlignSelf::Auto);
            assert_eq!(layout.align_self(&parent), AlignSelf::FlexEnd);

            let parent = layout_from_align_items(AlignItems::Center);
            let layout = layout_from_align_self(AlignSelf::Auto);
            assert_eq!(layout.align_self(&parent), AlignSelf::Center);

            let parent = layout_from_align_items(AlignItems::Baseline);
            let layout = layout_from_align_self(AlignSelf::Auto);
            assert_eq!(layout.align_self(&parent), AlignSelf::Baseline);

            let parent = layout_from_align_items(AlignItems::Stretch);
            let layout = layout_from_align_self(AlignSelf::Auto);
            assert_eq!(layout.align_self(&parent), AlignSelf::Stretch);
        }

        #[test]
        fn align_self() {
            let parent = layout_from_align_items(AlignItems::FlexEnd);
            let layout = layout_from_align_self(AlignSelf::FlexStart);
            assert_eq!(layout.align_self(&parent), AlignSelf::FlexStart);

            let parent = layout_from_align_items(AlignItems::FlexStart);
            let layout = layout_from_align_self(AlignSelf::FlexEnd);
            assert_eq!(layout.align_self(&parent), AlignSelf::FlexEnd);

            let parent = layout_from_align_items(AlignItems::FlexStart);
            let layout = layout_from_align_self(AlignSelf::Center);
            assert_eq!(layout.align_self(&parent), AlignSelf::Center);

            let parent = layout_from_align_items(AlignItems::FlexStart);
            let layout = layout_from_align_self(AlignSelf::Baseline);
            assert_eq!(layout.align_self(&parent), AlignSelf::Baseline);

            let parent = layout_from_align_items(AlignItems::FlexStart);
            let layout = layout_from_align_self(AlignSelf::Stretch);
            assert_eq!(layout.align_self(&parent), AlignSelf::Stretch);
        }
    }
}
