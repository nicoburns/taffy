//! Public API for C FFI

use super::{
    FloatResult, GridPlacement, GridPlacementResult, IntResult, ReturnCode, StyleValue, StyleValueResult,
    StyleValueUnit, TaffyAlignContent, TaffyAlignItems, TaffyDisplay, TaffyEdge, TaffyFFIResult, TaffyFlexDirection,
    TaffyFlexWrap, TaffyGridAutoFlow, TaffyOverflow, TaffyPosition, TaffyStyleConstRef, TaffyStyleMutRef,
};
use taffy::prelude as core;

/// Return [`ReturnCode::NullStylePointer`] if the passed pointer is null
macro_rules! assert_style_pointer_is_non_null {
    ($raw_style_ptr:expr) => {{
        if $raw_style_ptr.is_null() {
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
        let $style_ident = unsafe { &*($raw_style_ptr as *const core::Style) };

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
        let $style_ident = unsafe { &mut *($raw_style_ptr as *mut core::Style) };

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

// Simple enum properties

macro_rules! enum_prop_getter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleConstRef) -> IntResult {
            get_style!(raw_style, style, style.$($props).* as i32)
        }
    };
}

macro_rules! option_enum_prop_getter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleConstRef) -> IntResult {
            get_style!(raw_style, style, style.$($props).*.map(|v| v as i32).unwrap_or(0))
        }
    };
}

// Generate a function to set a style value such as margin.top or size.width
macro_rules! enum_prop_setter {
    ($func_name:ident; $($props:ident).+; $enum:ident) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleMutRef, value: $enum) -> ReturnCode {
            with_style_mut!(raw_style, style, style.$($props).* = value.into())
        }
    };
}

// Display
enum_prop_getter!(TaffyStyle_GetDisplay; display);
enum_prop_setter!(TaffyStyle_SetDisplay; display; TaffyDisplay);

// Position
enum_prop_getter!(TaffyStyle_GetPosition; position);
enum_prop_setter!(TaffyStyle_SetPosition; position; TaffyPosition);

// Overflow
enum_prop_getter!(TaffyStyle_GetOverflowX; overflow.x);
enum_prop_setter!(TaffyStyle_SetOverflowX; overflow.x; TaffyOverflow);
enum_prop_getter!(TaffyStyle_GetOverflowY; overflow.y);
enum_prop_setter!(TaffyStyle_SetOverflowY; overflow.y; TaffyOverflow);

// Alignment
option_enum_prop_getter!(TaffyStyle_GetAlignContent; align_content);
option_enum_prop_getter!(TaffyStyle_GetAlignItems; align_items);
option_enum_prop_getter!(TaffyStyle_GetAlignSelf; align_self);
option_enum_prop_getter!(TaffyStyle_GetJustifyContent; justify_content);
option_enum_prop_getter!(TaffyStyle_GetJustifyItems; justify_items);
option_enum_prop_getter!(TaffyStyle_GetJustifySelf; justify_self);
enum_prop_setter!(TaffyStyle_SetAlignContent; align_content; TaffyAlignContent);
enum_prop_setter!(TaffyStyle_SetAlignItems; align_items; TaffyAlignItems);
enum_prop_setter!(TaffyStyle_SetAlignSelf; align_self; TaffyAlignItems);
enum_prop_setter!(TaffyStyle_SetJustifyContent; justify_content; TaffyAlignContent);
enum_prop_setter!(TaffyStyle_SetJustifyItems; justify_items; TaffyAlignItems);
enum_prop_setter!(TaffyStyle_SetJustifySelf; justify_self; TaffyAlignItems);

// FlexDirection & FlexWrap
enum_prop_getter!(TaffyStyle_GetFlexDirection; flex_direction);
enum_prop_setter!(TaffyStyle_SetFlexDirection; flex_direction; TaffyFlexDirection);
enum_prop_getter!(TaffyStyle_GetFlexWrap; flex_wrap);
enum_prop_setter!(TaffyStyle_SetFlexWrap; flex_wrap; TaffyFlexWrap);

// GridAutoFlow
enum_prop_getter!(TaffyStyle_GetGridAutoFlow; grid_auto_flow);
enum_prop_setter!(TaffyStyle_SetGridAutoFlow; grid_auto_flow; TaffyGridAutoFlow);

/* API variant with single parameter that combines "value" and "unit" into a `StyleValue` struct */

// Generate a function to get a style value such as margin.top or size.width
macro_rules! style_value_prop_getter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleConstRef) -> StyleValueResult {
            get_style!(raw_style, style, style.$($props).*)
        }
    };
}

// Generate a function to set a style value such as margin.top or size.width
macro_rules! style_value_prop_setter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleMutRef, value: f32, unit: StyleValueUnit) -> ReturnCode {
            with_style_mut!(raw_style, style, style.$($props).* = try_from_raw!(unit, value))
        }
    };
}

// Size
style_value_prop_getter!(TaffyStyle_GetWidth; size.width);
style_value_prop_setter!(TaffyStyle_SetWidth; size.width);
style_value_prop_getter!(TaffyStyle_GetHeight; size.height);
style_value_prop_setter!(TaffyStyle_SetHeight; size.height);

// MinSize
style_value_prop_getter!(TaffyStyle_GetMinWidth; min_size.width);
style_value_prop_setter!(TaffyStyle_SetMinWidth; min_size.width);
style_value_prop_getter!(TaffyStyle_GetMinHeight; min_size.height);
style_value_prop_setter!(TaffyStyle_SetMinHeight; min_size.height);

// MaxSize
style_value_prop_getter!(TaffyStyle_GetMaxWidth; max_size.width);
style_value_prop_setter!(TaffyStyle_SetMaxWidth; max_size.width);
style_value_prop_getter!(TaffyStyle_GetMaxHeight; max_size.height);
style_value_prop_setter!(TaffyStyle_SetMaxHeight; max_size.height);

// Inset
style_value_prop_getter!(TaffyStyle_GetInsetTop; inset.top);
style_value_prop_setter!(TaffyStyle_SetInsetTop; inset.top);
style_value_prop_getter!(TaffyStyle_GetInsetBottom; inset.bottom);
style_value_prop_setter!(TaffyStyle_SetInsetBottom; inset.bottom);
style_value_prop_getter!(TaffyStyle_GetInsetLeft; inset.left);
style_value_prop_getter!(TaffyStyle_GetInsetRight; inset.right);
style_value_prop_setter!(TaffyStyle_SetInsetLeft; inset.left);
style_value_prop_setter!(TaffyStyle_SetInsetRight; inset.right);

// Margin
style_value_prop_getter!(TaffyStyle_GetMarginTop; margin.top);
style_value_prop_setter!(TaffyStyle_SetMarginTop; margin.top);
style_value_prop_getter!(TaffyStyle_GetMarginBottom; margin.bottom);
style_value_prop_setter!(TaffyStyle_SetMarginBottom; margin.bottom);
style_value_prop_getter!(TaffyStyle_GetMarginLeft; margin.left);
style_value_prop_getter!(TaffyStyle_GetMarginRight; margin.right);
style_value_prop_setter!(TaffyStyle_SetMarginLeft; margin.left);
style_value_prop_setter!(TaffyStyle_SetMarginRight; margin.right);

// Padding
style_value_prop_getter!(TaffyStyle_GetPaddingTop; padding.top);
style_value_prop_setter!(TaffyStyle_SetPaddingTop; padding.top);
style_value_prop_getter!(TaffyStyle_GetPaddingBottom; padding.bottom);
style_value_prop_setter!(TaffyStyle_SetPaddingBottom; padding.bottom);
style_value_prop_getter!(TaffyStyle_GetPaddingLeft; padding.left);
style_value_prop_getter!(TaffyStyle_GetPaddingRight; padding.right);
style_value_prop_setter!(TaffyStyle_SetPaddingLeft; padding.left);
style_value_prop_setter!(TaffyStyle_SetPaddingRight; padding.right);

// Border
style_value_prop_getter!(TaffyStyle_GetBorderTop; border.top);
style_value_prop_setter!(TaffyStyle_SetBorderTop; border.top);
style_value_prop_getter!(TaffyStyle_GetBorderBottom; border.bottom);
style_value_prop_setter!(TaffyStyle_SetBorderBottom; border.bottom);
style_value_prop_getter!(TaffyStyle_GetBorderLeft; border.left);
style_value_prop_getter!(TaffyStyle_GetBorderRight; border.right);
style_value_prop_setter!(TaffyStyle_SetBorderLeft; border.left);
style_value_prop_setter!(TaffyStyle_SetBorderRight; border.right);

// Gap
style_value_prop_getter!(TaffyStyle_GetColumnGap; gap.width);
style_value_prop_setter!(TaffyStyle_SetColumnGap; gap.width);
style_value_prop_getter!(TaffyStyle_GetRowGap; gap.height);
style_value_prop_setter!(TaffyStyle_SetRowGap; gap.height);

// Aspect ratio
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetAspectRatio(raw_style: TaffyStyleConstRef) -> FloatResult {
    get_style!(raw_style, style, style.aspect_ratio.unwrap_or(f32::NAN))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetAspectRatio(raw_style: TaffyStyleMutRef, value: f32) -> ReturnCode {
    with_style_mut!(raw_style, style, {
        if value.is_finite() && value > 0.0 {
            style.aspect_ratio = Some(value)
        } else {
            style.aspect_ratio = None;
        }
    })
}

// Generate a function to get a style value such as margin.top or size.width
macro_rules! float_prop_getter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleConstRef) -> FloatResult {
            get_style!(raw_style, style, style.$($props).*)
        }
    };
}

// Generate a function to set a style value such as margin.top or size.width
macro_rules! float_prop_setter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleMutRef, value: f32) -> ReturnCode {
            with_style_mut!(raw_style, style, style.$($props).* = value)
        }
    };
}

// Scrollbar width
float_prop_getter!(TaffyStyle_GetScrollbarWidth; scrollbar_width);
float_prop_setter!(TaffyStyle_SetScrollbarWidth; scrollbar_width);

// Flex
style_value_prop_getter!(TaffyStyle_GetFlexBasis; flex_basis);
style_value_prop_setter!(TaffyStyle_SetFlexBasis; flex_basis);
float_prop_getter!(TaffyStyle_GetFlexGrow; flex_grow);
float_prop_setter!(TaffyStyle_SetFlexGrow; flex_grow);
float_prop_getter!(TaffyStyle_GetFlexShrink; flex_shrink);
float_prop_setter!(TaffyStyle_SetFlexShrink; flex_shrink);

/// Function to set all the value of margin
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetMargin(
    raw_style: TaffyStyleMutRef,
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
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyleGetGridColumn(raw_style: TaffyStyleMutRef) -> GridPlacementResult {
    get_style!(raw_style, style, style.grid_column)
}

/// Set grid item's column placement
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyleSetGridColumn(raw_style: TaffyStyleMutRef, placement: GridPlacement) -> ReturnCode {
    with_style_mut!(raw_style, style, style.grid_column = placement.into())
}
