//! Values types for C FFI

use crate::prelude as core;

use super::ReturnCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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
