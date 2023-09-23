//! Values types for C FFI

use taffy::prelude as core;

use super::ReturnCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum TaffyEdge {
    /// The top edge of the box
    Top,
    /// The bottom edge of the box
    Bottom,
    /// The left edge of the box
    Left,
    /// The right edge of the box
    Right,
    /// Both the top and bottom edges of the box
    Vertical,
    /// Both the left and right edges of the box
    Horizontal,
    /// All four edges of the box
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum StyleValueUnit {
    /// A none value (used to unset optional fields)
    None,
    /// Fixed Length (pixel) value
    Length,
    /// Percentage value
    Percent,
    /// Min-content size
    MinContent,
    /// Max-content size
    MaxContent,
    /// fit-content() function with a pixel limit
    FitContentPx,
    /// fit-content() function with a percentage limit
    FitContentPercent,
    /// Automatic values
    Auto,
    /// fr unit
    Fr,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct StyleValue {
    /// The value. If the unit is variant that doesn't require a value (e.g. Auto) then the value is ignored.
    pub value: f32,
    pub unit: StyleValueUnit,
}
impl Default for StyleValue {
    fn default() -> Self {
        Self { unit: StyleValueUnit::None, value: 0.0 }
    }   
}

impl StyleValue {
    #[inline(always)]
    pub fn from_raw(unit: StyleValueUnit, value: f32) -> Self {
        Self { unit, value }
    }
}

impl From<core::LengthPercentage> for StyleValue {
    fn from(value: core::LengthPercentage) -> Self {
        match value {
            core::LengthPercentage::Length(value) => Self { unit: StyleValueUnit::Length, value },
            core::LengthPercentage::Percent(value) => Self { unit: StyleValueUnit::Percent, value },
        }
    }
}

impl TryFrom<StyleValue> for core::LengthPercentage {
    type Error = ReturnCode;

    fn try_from(value: StyleValue) -> Result<Self, Self::Error> {
        match value.unit {
            StyleValueUnit::Length => Ok(core::LengthPercentage::Length(value.value)),
            StyleValueUnit::Percent => Ok(core::LengthPercentage::Percent(value.value)),
            StyleValueUnit::None => Err(ReturnCode::InvalidNone),
            StyleValueUnit::Auto => Err(ReturnCode::InvalidAuto),
            StyleValueUnit::MinContent => Err(ReturnCode::InvalidMinContent),
            StyleValueUnit::MaxContent => Err(ReturnCode::InvalidMaxContent),
            StyleValueUnit::FitContentPx => Err(ReturnCode::InvalidFitContentPx),
            StyleValueUnit::FitContentPercent => Err(ReturnCode::InvalidFitContentPercent),
            StyleValueUnit::Fr => Err(ReturnCode::InvalidFr),
        }
    }
}

impl From<core::LengthPercentageAuto> for StyleValue {
    fn from(value: core::LengthPercentageAuto) -> Self {
        match value {
            core::LengthPercentageAuto::Length(value) => Self { unit: StyleValueUnit::Length, value },
            core::LengthPercentageAuto::Percent(value) => Self { unit: StyleValueUnit::Percent, value },
            core::LengthPercentageAuto::Auto => Self { unit: StyleValueUnit::Auto, value: 0.0 },
        }
    }
}

impl TryFrom<StyleValue> for core::LengthPercentageAuto {
    type Error = ReturnCode;

    fn try_from(value: StyleValue) -> Result<Self, Self::Error> {
        match value.unit {
            StyleValueUnit::Auto => Ok(core::LengthPercentageAuto::Auto),
            StyleValueUnit::Length => Ok(core::LengthPercentageAuto::Length(value.value)),
            StyleValueUnit::Percent => Ok(core::LengthPercentageAuto::Percent(value.value)),
            StyleValueUnit::None => Err(ReturnCode::InvalidNone),
            StyleValueUnit::MinContent => Err(ReturnCode::InvalidMinContent),
            StyleValueUnit::MaxContent => Err(ReturnCode::InvalidMaxContent),
            StyleValueUnit::FitContentPx => Err(ReturnCode::InvalidFitContentPx),
            StyleValueUnit::FitContentPercent => Err(ReturnCode::InvalidFitContentPercent),
            StyleValueUnit::Fr => Err(ReturnCode::InvalidFr),
        }
    }
}

impl From<core::Dimension> for StyleValue {
    fn from(value: core::Dimension) -> Self {
        match value {
            core::Dimension::Length(value) => Self { unit: StyleValueUnit::Length, value },
            core::Dimension::Percent(value) => Self { unit: StyleValueUnit::Percent, value },
            core::Dimension::Auto => Self { unit: StyleValueUnit::Auto, value: 0.0 },
        }
    }
}

impl TryFrom<StyleValue> for core::Dimension {
    type Error = ReturnCode;

    fn try_from(value: StyleValue) -> Result<Self, Self::Error> {
        match value.unit {
            StyleValueUnit::Auto => Ok(core::Dimension::Auto),
            StyleValueUnit::Length => Ok(core::Dimension::Length(value.value)),
            StyleValueUnit::Percent => Ok(core::Dimension::Percent(value.value)),
            StyleValueUnit::None => Err(ReturnCode::InvalidNone),
            StyleValueUnit::MinContent => Err(ReturnCode::InvalidMinContent),
            StyleValueUnit::MaxContent => Err(ReturnCode::InvalidMaxContent),
            StyleValueUnit::FitContentPx => Err(ReturnCode::InvalidFitContentPx),
            StyleValueUnit::FitContentPercent => Err(ReturnCode::InvalidFitContentPercent),
            StyleValueUnit::Fr => Err(ReturnCode::InvalidFr),
        }
    }
}

/// For all fields, zero represents not set
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct GridPlacement {
    pub start: i16,
    pub end: i16,
    pub span: u16,
}

impl Default for GridPlacement {
    fn default() -> Self {
        Self { start: 0, end: 0, span: 0 }
    }   
}

impl From<GridPlacement> for core::Line<core::GridPlacement> {
    fn from(placement: GridPlacement) -> Self {
        Self::from_raw_parts(placement.start, placement.span, placement.end)
    }
}

impl From<core::Line<core::GridPlacement>> for GridPlacement {
    fn from(placement: core::Line<core::GridPlacement>) -> Self {
        let (start, span, end) = placement.into_raw_parts();
        Self { start, span, end }
    }
}
