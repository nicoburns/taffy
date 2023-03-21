//! Return types for C FFI

use super::{StyleValueUnit, StyleValue};

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

#[repr(C)]
pub struct StyleValueResult {
    pub return_code: ReturnCode,
    pub value: StyleValue,
}

impl From<ReturnCode> for StyleValueResult {
    fn from(return_code: ReturnCode) -> Self {
        Self { return_code, value: StyleValue { unit: StyleValueUnit::None, value: 0.0 } }
    }
}