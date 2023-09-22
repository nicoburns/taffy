//! Public API for C FFI
pub use taffy::style::Style as TaffyStyle;

use taffy::geometry::Rect;
use std::ffi::c_void;
use super::{GridPlacement, ReturnCode, StyleValue, StyleValueResult, StyleValueUnit, GridPlacementResult, TaffyFFIResult};


/// Return [`ReturnCode::NullStylePointer`] if the passed pointer is null
macro_rules! assert_style_pointer_is_non_null {
    ($raw_style_ptr:expr) => {{
        if ($raw_style_ptr as *const c_void) == std::ptr::null() {
            return TaffyFFIResult::from_return_code(ReturnCode::NullStylePointer);
        }
    }};
}

/// Assert that the passed raw style pointer is non-null
/// Then give the passed expression access to the value of the inner [`core::Style`] struct pointed to by the raw style pointer
/// Return whatever the expression evaluates to wrapped in a [`StyleValueResult`] if the expression does not interally return.
macro_rules! get_style {
    ($raw_style_ptr:expr, $style_ident:ident, $block:expr) => {{
        assert_style_pointer_is_non_null!($raw_style_ptr);
        let $style_ident = unsafe { &*($raw_style_ptr as *const TaffyStyle) };

        let return_value = $block;

        TaffyFFIResult::from_value(return_value.into())
    }};
}

/// Assert that the passed raw style pointer is non-null
/// Then give the passed expression mutable access to the value of the inner [`core::Style`] struct pointed to by the raw style pointer
/// Return [`ReturnCode::Ok`] if the expression does not internally return.
macro_rules! with_style_mut {
    ($raw_style_ptr:expr, $style_ident:ident, $block:expr) => {{
        assert_style_pointer_is_non_null!($raw_style_ptr);
        let $style_ident = unsafe { &mut*($raw_style_ptr as *mut TaffyStyle) };

        $block;

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
pub unsafe extern "C" fn TaffyStyle_get_margin_top(raw_style: *const TaffyStyle) -> StyleValueResult {
    get_style!(raw_style, style, style.margin.top)
}

/// Function to set the margin_top value
#[no_mangle]
pub unsafe extern "C" fn TaffyStyle_set_margin_top(raw_style: *mut TaffyStyle, value: StyleValue) -> ReturnCode {
    with_style_mut!(raw_style, style, style.margin.top = try_from_value!(value))
}

/// Function to set all the value of margin
#[no_mangle]
pub unsafe extern "C" fn TaffyStyle_set_margin_trbl(
    raw_style: *mut TaffyStyle,
    top: StyleValue,
    right: StyleValue,
    bottom: StyleValue,
    left: StyleValue,
) -> ReturnCode {
    with_style_mut!(raw_style, style, {
        style.margin = Rect {
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
pub unsafe extern "C" fn TaffyStyle_get_padding_top(raw_style: *const TaffyStyle) -> StyleValueResult {
    get_style!(raw_style, style, style.padding.top)
}

/// Function to set the padding_top value
#[no_mangle]
pub unsafe extern "C" fn TaffyStyle_set_padding_top(raw_style: *mut TaffyStyle, value: f32, unit: StyleValueUnit) -> ReturnCode {
    with_style_mut!(raw_style, style, style.padding.top = try_from_raw!(unit, value))
}

/// Function to set all the value of padding
#[no_mangle]
pub unsafe extern "C" fn TaffyStyle_set_padding_trbl(
    raw_style: *mut TaffyStyle,
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
        style.padding = Rect {
            top: try_from_raw!(top_value_unit, top_value),
            right: try_from_raw!(right_value_unit, right_value),
            bottom: try_from_raw!(bottom_value_unit, bottom_value),
            left: try_from_raw!(left_value_unit, left_value),
        };
    })
}

/* Grid APIs */

/// Get grid item's column placement
#[no_mangle]
pub unsafe extern "C" fn TaffyStyle_get_grid_column(raw_style: *mut TaffyStyle) -> GridPlacementResult {
    get_style!(raw_style, style, style.grid_column)
}

/// Set grid item's column placement
#[no_mangle]
pub unsafe extern "C" fn TaffyStyle_set_grid_column(raw_style: *mut TaffyStyle, placement: GridPlacement) -> ReturnCode {
    with_style_mut!(raw_style, style, style.grid_column = placement.into())
}
