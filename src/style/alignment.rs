//! Style definitions for alignment

/// Sets the distribution of space between and around content items along the cross-axis
///
/// The default value is [`AlignContent::Stretch`].
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#align-content-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AlignContent {
    /// Items are packed toward the start of the axis
    FlexStart,
    /// Items are packed toward the end of the axis
    FlexEnd,
    /// Items are centered around the middle of the axis
    Center,
    /// Items are stretched to fill the container
    Stretch,
    /// The first and last items are aligned flush with the edges of the container (no gap)
    /// The gap between items is distributed evenly.
    SpaceBetween,
    /// The gap between the first and last items is exactly THE SAME as the gap between items.
    /// The gaps are distributed evenly
    SpaceEvenly,
    /// The gap between the first and last items is exactly HALF the gap between items.
    /// The gaps are distributed evenly in proportion to these ratios.
    SpaceAround,
}

impl Default for AlignContent {
    fn default() -> Self {
        Self::Stretch
    }
}

/// Sets the distribution of space between and around content items along the main-axis
///
/// The default value is [`JustifyContent::FlexStart`].
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#justify-content-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JustifyContent {
    /// Items are packed toward the start of the main axis
    FlexStart,
    /// Items are packed toward the end of the main axis
    FlexEnd,
    /// Items are packed along the center of the main axis
    Center,
    /// The first and last items are aligned flush with the edges of the container (no gap)
    /// The gaps between items are distributed evenly.
    SpaceBetween,
    /// The gap between the first and last items is exactly THE SAME as the gap between items.
    /// The gaps are distributed evenly
    SpaceEvenly,
    /// The gap between the first and last items is exactly HALF the gap between items.
    /// The gaps are distributed evenly in proportion to these ratios.
    SpaceAround,
}

impl Default for JustifyContent {
    fn default() -> Self {
        Self::FlexStart
    }
}

/// How [`Nodes`](crate::node::Node) are aligned relative to the cross axis
///
/// The default behavior is [`AlignItems::Stretch`].
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#align-items-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AlignItems {
    /// Items are packed toward the start of the cross axis
    FlexStart,
    /// Items are packed toward the end of the cross axis
    FlexEnd,
    /// Items are packed along the center of the cross axis
    Center,
    /// Items are aligned such as their baselines align
    Baseline,
    /// Stretch to fill the container
    Stretch,
}

impl Default for AlignItems {
    fn default() -> Self {
        Self::Stretch
    }
}

/// Overrides the inherited [`AlignItems`] behavior for this node.
///
/// The behavior of any child nodes will be controlled by this node's [`AlignItems`] value.
///
/// The default behavior is [`AlignSelf::Auto`].
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#align-items-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AlignSelf {
    /// Inherits the [`AlignItems`] behavior of the parent
    Auto,
    /// Items are packed toward the start of the cross axis
    FlexStart,
    /// Items are packed toward the end of the cross axis
    FlexEnd,
    /// Items are packed along the center of the cross axis
    Center,
    /// Items are aligned such as their baselines align
    Baseline,
    /// Distribute items evenly, but stretch them to fill the container
    Stretch,
}

impl Default for AlignSelf {
    fn default() -> Self {
        Self::Auto
    }
}
