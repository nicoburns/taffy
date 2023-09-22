//! Return types for C FFI

use super::{StyleValue, StyleValueUnit, GridPlacement};

pub (crate) trait TaffyFFIResult {
    type Value;
    fn from_value(value: Self::Value) -> Self;
    fn from_return_code(return_code: ReturnCode) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ReturnCode {
    /// Operation suceeded
    Ok,
    /// The style pointer passed was null
    NullStylePointer,
    /// A None unit was specified but is not valid in this context
    InvalidNone,
    /// A Points unit was specified but is not valid in this context
    InvalidPoints,
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

impl TaffyFFIResult for ReturnCode {
    type Value = ReturnCode;
    fn from_value(value: Self::Value) -> Self {
       value
    }
    fn from_return_code(return_code: ReturnCode) -> Self {
        return_code
    }
}

#[repr(C)]
pub struct StyleValueResult {
    pub return_code: ReturnCode,
    pub value: StyleValue,
}

impl TaffyFFIResult for StyleValueResult {
    type Value = StyleValue;
    fn from_value(value: Self::Value) -> Self {
        Self { return_code: ReturnCode::Ok, value }
    }
    fn from_return_code(return_code: ReturnCode) -> Self {
        Self { return_code, value: StyleValue { unit: StyleValueUnit::None, value: 0.0 } }
    }
}


#[repr(C)]
pub struct GridPlacementResult {
    pub return_code: ReturnCode,
    pub value: GridPlacement,
}

impl TaffyFFIResult for GridPlacementResult {
    type Value = GridPlacement;
    fn from_value(value: Self::Value) -> Self {
        Self { return_code: ReturnCode::Ok, value }
    }
    fn from_return_code(return_code: ReturnCode) -> Self {
        Self { return_code, value: GridPlacement { start: 0, end: 0, span: 0 } }
    }
}
