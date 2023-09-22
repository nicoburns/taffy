//! Public API for C FFI
pub use taffy::style::Style as TaffyStyle;

use super::{
    FloatResult, GridPlacement, GridPlacementResult, ReturnCode, StyleValue, StyleValueResult, StyleValueUnit,
    TaffyEdge, TaffyFFIResult,
};
use std::ffi::c_void;
use taffy::geometry::Rect;

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
        let $style_ident = unsafe { &mut *($raw_style_ptr as *mut TaffyStyle) };

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

// Generate a function to get a style value such as margin.top or size.width
macro_rules! numeric_prop_getter {
    ($func_name:ident, $property:ident, $edge:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $func_name(raw_style: *const TaffyStyle) -> StyleValueResult {
            get_style!(raw_style, style, style.$property.$edge)
        }
    };
}

// Generate a function to set a style value such as margin.top or size.width
macro_rules! numeric_prop_setter {
    ($func_name:ident, $property:ident, $edge:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $func_name(raw_style: *mut TaffyStyle, value: StyleValue) -> ReturnCode {
            with_style_mut!(raw_style, style, style.$property.$edge = try_from_value!(value))
        }
    };
}

// Size
numeric_prop_getter!(TaffyStyle_GetWidth, size, width);
numeric_prop_setter!(TaffyStyle_SetWidth, size, width);
numeric_prop_getter!(TaffyStyle_GetHeight, size, height);
numeric_prop_setter!(TaffyStyle_SetHeight, size, height);

// MinSize
numeric_prop_getter!(TaffyStyle_GetMinWidth, min_size, width);
numeric_prop_setter!(TaffyStyle_SetMinWidth, min_size, width);
numeric_prop_getter!(TaffyStyle_GetMinHeight, min_size, height);
numeric_prop_setter!(TaffyStyle_SetMinHeight, min_size, height);

// MaxSize
numeric_prop_getter!(TaffyStyle_GetMaxWidth, max_size, width);
numeric_prop_setter!(TaffyStyle_SetMaxWidth, max_size, width);
numeric_prop_getter!(TaffyStyle_GetMaxHeight, max_size, height);
numeric_prop_setter!(TaffyStyle_SetMaxHeight, max_size, height);

// Inset
numeric_prop_getter!(TaffyStyle_GetInsetTop, inset, top);
numeric_prop_setter!(TaffyStyle_SetInsetTop, inset, top);
numeric_prop_getter!(TaffyStyle_GetInsetBottom, inset, bottom);
numeric_prop_setter!(TaffyStyle_SetInsetBottom, inset, bottom);
numeric_prop_getter!(TaffyStyle_GetInsetLeft, inset, left);
numeric_prop_getter!(TaffyStyle_GetInsetRight, inset, right);
numeric_prop_setter!(TaffyStyle_SetInsetLeft, inset, left);
numeric_prop_setter!(TaffyStyle_SetInsetRight, inset, right);

// Margin
numeric_prop_getter!(TaffyStyle_GetMarginTop, margin, top);
numeric_prop_setter!(TaffyStyle_SetMarginTop, margin, top);
numeric_prop_getter!(TaffyStyle_GetMarginBottom, margin, bottom);
numeric_prop_setter!(TaffyStyle_SetMarginBottom, margin, bottom);
numeric_prop_getter!(TaffyStyle_GetMarginLeft, margin, left);
numeric_prop_getter!(TaffyStyle_GetMarginRight, margin, right);
numeric_prop_setter!(TaffyStyle_SetMarginLeft, margin, left);
numeric_prop_setter!(TaffyStyle_SetMarginRight, margin, right);

// Padding
numeric_prop_getter!(TaffyStyle_GetPaddingTop, padding, top);
numeric_prop_setter!(TaffyStyle_SetPaddingTop, padding, top);
numeric_prop_getter!(TaffyStyle_GetPaddingBottom, padding, bottom);
numeric_prop_setter!(TaffyStyle_SetPaddingBottom, padding, bottom);
numeric_prop_getter!(TaffyStyle_GetPaddingLeft, padding, left);
numeric_prop_getter!(TaffyStyle_GetPaddingRight, padding, right);
numeric_prop_setter!(TaffyStyle_SetPaddingLeft, padding, left);
numeric_prop_setter!(TaffyStyle_SetPaddingRight, padding, right);

// Border
numeric_prop_getter!(TaffyStyle_GetBorderTop, border, top);
numeric_prop_setter!(TaffyStyle_SetBorderTop, border, top);
numeric_prop_getter!(TaffyStyle_GetBorderBottom, border, bottom);
numeric_prop_setter!(TaffyStyle_SetBorderBottom, border, bottom);
numeric_prop_getter!(TaffyStyle_GetBorderLeft, border, left);
numeric_prop_getter!(TaffyStyle_GetBorderRight, border, right);
numeric_prop_setter!(TaffyStyle_SetBorderLeft, border, left);
numeric_prop_setter!(TaffyStyle_SetBorderRight, border, right);

// Gap
numeric_prop_getter!(TaffyStyle_GetColumnGap, gap, width);
numeric_prop_setter!(TaffyStyle_SetColumnGap, gap, width);
numeric_prop_getter!(TaffyStyle_GetRowGap, gap, height);
numeric_prop_setter!(TaffyStyle_SetRowGap, gap, height);

// Aspect ratio
#[no_mangle]
pub unsafe extern "C" fn TaffyStyle_GetAspectRatio(raw_style: *const TaffyStyle) -> FloatResult {
    get_style!(raw_style, style, style.aspect_ratio.unwrap_or(f32::NAN))
}
#[no_mangle]
pub unsafe extern "C" fn TaffyStyle_SetAspectRatio(raw_style: *mut TaffyStyle, value: f32) -> ReturnCode {
    with_style_mut!(raw_style, style, {
        if value.is_finite() && value > 0.0 {
            style.aspect_ratio = Some(value)
        } else {
            style.aspect_ratio = None;
        }
    })
}

// Flex
#[no_mangle]
pub unsafe extern "C" fn TaffyStyle_GetFlexBasis(raw_style: *const TaffyStyle) -> StyleValueResult {
    get_style!(raw_style, style, style.flex_basis)
}
#[no_mangle]
pub unsafe extern "C" fn TaffyStyle_SetFlexBasis(
    raw_style: *mut TaffyStyle,
    value: f32,
    unit: StyleValueUnit,
) -> ReturnCode {
    with_style_mut!(raw_style, style, style.flex_basis = try_from_raw!(unit, value))
}
#[no_mangle]
pub unsafe extern "C" fn TaffyStyle_GetFlexGrow(raw_style: *const TaffyStyle) -> FloatResult {
    get_style!(raw_style, style, style.flex_grow)
}
#[no_mangle]
pub unsafe extern "C" fn TaffyStyle_SetFlexGrow(raw_style: *mut TaffyStyle, value: f32) -> ReturnCode {
    with_style_mut!(raw_style, style, style.flex_grow = value)
}
#[no_mangle]
pub unsafe extern "C" fn TaffyStyle_GetFlexShrink(raw_style: *const TaffyStyle) -> FloatResult {
    get_style!(raw_style, style, style.flex_shrink)
}
#[no_mangle]
pub unsafe extern "C" fn TaffyStyle_SetFlexShrink(raw_style: *mut TaffyStyle, value: f32) -> ReturnCode {
    with_style_mut!(raw_style, style, style.flex_shrink = value)
}

// /// Function to get the margin_top value
// #[no_mangle]
// pub unsafe extern "C" fn TaffyStyle_GetMarginTop(raw_style: *const TaffyStyle) -> StyleValueResult {
//     get_style!(raw_style, style, style.margin.top)
// }

// /// Function to set the margin_top value
// #[no_mangle]
// pub unsafe extern "C" fn TaffyStyle_SetMarginTop(raw_style: *mut TaffyStyle, value: StyleValue) -> ReturnCode {
//     with_style_mut!(raw_style, style, style.margin.top = try_from_value!(value))
// }

/// Function to set all the value of margin
#[no_mangle]
pub unsafe extern "C" fn TaffyStyle_SetMargin(
    raw_style: *mut TaffyStyle,
    edge: TaffyEdge,
    value: StyleValue,
) -> ReturnCode {
    let value = try_from_value!(value);
    with_style_mut!(raw_style, style, {
        match edge {
            TaffyEdge::Top => style.margin.top = value,
            TaffyEdge::Bottom => style.margin.bottom = value,
            TaffyEdge::Left => style.margin.left = value,
            TaffyEdge::Right => style.margin.right = value,
            TaffyEdge::Vertical => {
                style.margin.top = value;
                style.margin.bottom = value;
            }
            TaffyEdge::Horizontal => {
                style.margin.left = value;
                style.margin.right = value;
            }
            TaffyEdge::All => {
                style.margin.top = value;
                style.margin.bottom = value;
                style.margin.left = value;
                style.margin.right = value;
            }
        };
    })
}

/* Grid APIs */

/// Get grid item's column placement
#[no_mangle]
pub unsafe extern "C" fn TaffyStyleGetGridColumn(raw_style: *mut TaffyStyle) -> GridPlacementResult {
    get_style!(raw_style, style, style.grid_column)
}

/// Set grid item's column placement
#[no_mangle]
pub unsafe extern "C" fn TaffyStyleSetGridColumn(raw_style: *mut TaffyStyle, placement: GridPlacement) -> ReturnCode {
    with_style_mut!(raw_style, style, style.grid_column = placement.into())
}
