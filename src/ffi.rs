//! Types for C FFI

use crate::prelude as core;
use crate::geometry::{Rect};
use std::ffi::c_void;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ReturnCode {
    /// Operation suceeded
    Ok,
    /// A None unit was specified but is not valid in this context
    InvalidNone,
    /// A Length unit was specified but is not valid in this context
    InvalidLength,
    /// A Percent unit was specified but is not valid in this context
    InvalidPercent,
    /// A MinContent unit was specified but is not valid in this context
    InvalidMinContent,
    /// A MaxContent unit was specified but is not valid in this context
    InvalidMaxContent,
    /// A FitContentPx unit was specified but is not valid in this context
    InvalidFitContentPx,
    /// A FitContentPercent unit was specified but is not valid in this context
    InvalidFitContentPercent,
    /// An Auto unit was specified but is not valid in this context
    InvalidAuto,
    /// An Fr unit was specified but is not valid in this context
    InvalidFr,
    /// A NaN value was specified but is not valid in this context
    UnexpectedNaN,
    /// A infinite value was specified but is not valid in this context
    UnexpectedInfinity,
    /// A negative value was specified but is not valid in this context
    UnexpectedNegative,
}

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
    fn from_raw(unit: StyleValueUnit, value: f32) -> Self {
        Self { unit, value }
    }
}

impl From<core::LengthPercentage> for StyleValue {
    fn from(value: core::LengthPercentage) -> Self {
        match value {
            core::LengthPercentage::Points(value) => Self { unit: StyleValueUnit::Points, value },
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
            core::LengthPercentageAuto::Points(value) => Self { unit: StyleValueUnit::Points, value },
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

#[repr(C)]
pub struct StyleValueResult {
    pub return_code: ReturnCode,
    pub value: StyleValue,
}

#[repr(transparent)]
pub struct Style {
    inner: core::Style,
}

pub fn assert_pointer_address(pointer: *const c_void, pointer_type: &str) {
    assert_ne!(
        pointer,
        std::ptr::null(),
        "Invalid {:} pointer address",
        pointer_type
    );
}

macro_rules! get_style {
    ($raw_style_ptr:expr, $rust_style_ptr:ident, $block:expr) => {{
        assert_pointer_address($raw_style_ptr, "style");
        let $rust_style_ptr = unsafe { Box::from_raw($raw_style_ptr as *mut Style) };

        let return_value = $block;

        Box::leak($rust_style_ptr);
        StyleValueResult {
            return_code: ReturnCode::Ok,
            value: return_value.into()
        }
    }};
}

macro_rules! with_style_mut {
    ($raw_style_ptr:expr, $rust_style_ptr:ident, $block:expr) => {{
        assert_pointer_address($raw_style_ptr, "style");
        let mut $rust_style_ptr = unsafe { Box::from_raw($raw_style_ptr as *mut Style) };

        $block;

        Box::leak($rust_style_ptr);
        ReturnCode::Ok
    }};
}

/// Attempt to convert a [`StyleValue`] into a type that implements `TryFrom<StyleValue>`
/// In the case of a conversion error, return a [`ReturnCode`].
macro_rules! try_from_value {
    ($value:expr) => {
        match $value.try_into() {
            Ok(val) => val,
            Err(err) => return err,
        }
    };
}

/// Attempt to convert a [`StyleValueUnit`] and a `f32` into a type that implements `TryFrom<StyleValue>`
/// In the case of a conversion error, return a [`ReturnCode`].
macro_rules! try_from_raw {
    ($unit:expr, $value:expr) => {
        try_from_value!(StyleValue::from_raw($unit, $value))
    };
}

/* API variant with single parameter that combines "value" and "unit" into a `StyleValue` struct */

/// Function to get the margin_top value
#[no_mangle]
pub extern "C" fn Taffy_get_margin_top(
    raw_style: *const c_void,
) -> StyleValueResult {
    get_style!(raw_style, style, style.inner.margin.top)
}

/// Function to set the margin_top value
#[no_mangle]
pub extern "C" fn Taffy_set_margin_top(
    raw_style: *mut c_void,
    value: StyleValue,
) -> ReturnCode {
    with_style_mut!(raw_style, style, style.inner.margin.top = try_from_value!(value))
}


/// Function to set all the value of margin
#[no_mangle]
pub extern "C" fn Taffy_set_margin_trbl(
    raw_style: *mut c_void,
    top: StyleValue,
    right: StyleValue,
    bottom: StyleValue,
    left: StyleValue,
) -> ReturnCode {
    with_style_mut!(raw_style, style, {
        style.inner.margin = Rect {
            top: try_from_value!(top),
            right: try_from_value!(right),
            bottom: try_from_value!(bottom),
            left: try_from_value!(left),
        };
    })
}

/* API variant with seperate "value" and "unit" parameters */

/// Function to get the margin_top value
#[no_mangle]
pub extern "C" fn Taffy_get_padding_top(
    raw_style: *const c_void,
) -> StyleValueResult {
    get_style!(raw_style, style, style.inner.padding.top)
}

/// Function to set the padding_top value
#[no_mangle]
pub extern "C" fn Taffy_set_padding_top(
    raw_style: *mut c_void,
    value: f32,
    unit: StyleValueUnit,
) -> ReturnCode {
    with_style_mut!(raw_style, style, style.inner.padding.top = try_from_raw!(unit, value))
}

/// Function to set all the value of padding
#[no_mangle]
pub extern "C" fn Taffy_set_padding_trbl(
    raw_style: *mut c_void,
    top_value: f32,
    top_value_unit: StyleValueUnit,
    right_value: f32,
    right_value_unit: StyleValueUnit,
    left_value: f32,
    left_value_unit: StyleValueUnit,
    bottom_value: f32,
    bottom_value_unit: StyleValueUnit,
) -> ReturnCode {
    with_style_mut!(raw_style, style, {
        style.inner.padding = Rect {
            top: try_from_raw!(top_value_unit, top_value),
            right: try_from_raw!(right_value_unit, right_value),
            bottom: try_from_raw!(bottom_value_unit, bottom_value),
            left: try_from_raw!(left_value_unit, left_value),
        };
    })
}


