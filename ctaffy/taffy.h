#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum ReturnCode {
  // Operation suceeded
  Ok,
  // The style pointer passed was null
  NullStylePointer,
  // An enum value was specified that was outside the range of valid value for this enum
  InvalidEnumValue,
  // A Points unit was specified but is not valid in this context
  InvalidNone,
  // A Points unit was specified but is not valid in this context
  InvalidPoints,
  // A Percent unit was specified but is not valid in this context
  InvalidPercent,
  // A MinContent unit was specified but is not valid in this context
  InvalidMinContent,
  // A MaxContent unit was specified but is not valid in this context
  InvalidMaxContent,
  // A FitContentPx unit was specified but is not valid in this context
  InvalidFitContentPx,
  // A FitContentPercent unit was specified but is not valid in this context
  InvalidFitContentPercent,
  // An Auto unit was specified but is not valid in this context
  InvalidAuto,
  // An Fr unit was specified but is not valid in this context
  InvalidFr,
  // A NaN value was specified but is not valid in this context
  UnexpectedNaN,
  // A infinite value was specified but is not valid in this context
  UnexpectedInfinity,
  // A negative value was specified but is not valid in this context
  UnexpectedNegative,
} ReturnCode;

typedef enum StyleValueUnit {
  // A none value (used to unset optional fields)
  None,
  // Fixed Length (pixel) value
  Length,
  // Percentage value
  Percent,
  // Min-content size
  MinContent,
  // Max-content size
  MaxContent,
  // fit-content() function with a pixel limit
  FitContentPx,
  // fit-content() function with a percentage limit
  FitContentPercent,
  // Automatic values
  Auto,
  // fr unit
  Fr,
} StyleValueUnit;

// Sets the distribution of space between and around content items
// For Flexbox it controls alignment in the cross axis
// For Grid it controls alignment in the block axis
//
// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-content)
typedef enum TaffyAlignContent {
  // Items are aligned according to their algorithm-specific default value
  // This is equivalent to not setting a value in CSS
  Normal,
  // Items are packed toward the start of the axis
  Start,
  // Items are packed toward the end of the axis
  End,
  // Items are packed towards the flex-relative start of the axis.
  //
  // For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
  // to End. In all other cases it is equivalent to Start.
  FlexStart,
  // Items are packed towards the flex-relative end of the axis.
  //
  // For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
  // to Start. In all other cases it is equivalent to End.
  FlexEnd,
  // Items are centered around the middle of the axis
  Center,
  // Items are stretched to fill the container
  Stretch,
  // The first and last items are aligned flush with the edges of the container (no gap)
  // The gap between items is distributed evenly.
  SpaceBetween,
  // The gap between the first and last items is exactly THE SAME as the gap between items.
  // The gaps are distributed evenly
  SpaceEvenly,
  // The gap between the first and last items is exactly HALF the gap between items.
  // The gaps are distributed evenly in proportion to these ratios.
  SpaceAround,
} TaffyAlignContent;

// Used to control how child nodes are aligned.
// For Flexbox it controls alignment in the cross axis
// For Grid it controls alignment in the block axis
//
// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-items)
typedef enum TaffyAlignItems {
  // Items are aligned according to their algorithm-specific default value
  // This is equivalent to not setting a value in CSS
  Normal,
  // Items are packed toward the start of the axis
  Start,
  // Items are packed toward the end of the axis
  End,
  // Items are packed towards the flex-relative start of the axis.
  //
  // For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
  // to End. In all other cases it is equivalent to Start.
  FlexStart,
  // Items are packed towards the flex-relative end of the axis.
  //
  // For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
  // to Start. In all other cases it is equivalent to End.
  FlexEnd,
  // Items are packed along the center of the cross axis
  Center,
  // Items are aligned such as their baselines align
  Baseline,
  // Stretch to fill the container
  Stretch,
} TaffyAlignItems;

// Sets the layout used for the children of this node
//
// The default values depends on on which feature flags are enabled. The order of precedence is: Flex, Grid, Block, None.
typedef enum TaffyDisplay {
  // The children will follow the block layout algorithm
  Block,
  // The children will follow the flexbox layout algorithm
  Flex,
  // The children will follow the CSS Grid layout algorithm
  Grid,
  // The children will not be laid out, and will follow absolute positioning
  None,
} TaffyDisplay;

typedef enum TaffyEdge {
  // The top edge of the box
  Top,
  // The bottom edge of the box
  Bottom,
  // The left edge of the box
  Left,
  // The right edge of the box
  Right,
  // Both the top and bottom edges of the box
  Vertical,
  // Both the left and right edges of the box
  Horizontal,
  // All four edges of the box
  All,
} TaffyEdge;

// The direction of the flexbox layout main axis.
//
// There are always two perpendicular layout axes: main (or primary) and cross (or secondary).
// Adding items will cause them to be positioned adjacent to each other along the main axis.
// By varying this value throughout your tree, you can create complex axis-aligned layouts.
//
// Items are always aligned relative to the cross axis, and justified relative to the main axis.
//
// The default behavior is [`FlexDirection::Row`].
//
// [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-direction-property)
typedef enum TaffyFlexDirection {
  // Defines +x as the main axis
  //
  // Items will be added from left to right in a row.
  Row,
  // Defines +y as the main axis
  //
  // Items will be added from top to bottom in a column.
  Column,
  // Defines -x as the main axis
  //
  // Items will be added from right to left in a row.
  RowReverse,
  // Defines -y as the main axis
  //
  // Items will be added from bottom to top in a column.
  ColumnReverse,
} TaffyFlexDirection;

// Controls whether flex items are forced onto one line or can wrap onto multiple lines.
//
// Defaults to [`FlexWrap::NoWrap`]
//
// [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-wrap-property)
typedef enum TaffyFlexWrap {
  // Items will not wrap and stay on a single line
  NoWrap,
  // Items will wrap according to this item's [`FlexDirection`]
  Wrap,
  // Items will wrap in the opposite direction to this item's [`FlexDirection`]
  WrapReverse,
} TaffyFlexWrap;

// Controls whether grid items are placed row-wise or column-wise. And whether the sparse or dense packing algorithm is used.
//
// The "dense" packing algorithm attempts to fill in holes earlier in the grid, if smaller items come up later. This may cause items to appear out-of-order, when doing so would fill in holes left by larger items.
//
// Defaults to [`GridAutoFlow::Row`]
//
// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow)
typedef enum TaffyGridAutoFlow {
  // Items are placed by filling each row in turn, adding new rows as necessary
  Row,
  // Items are placed by filling each column in turn, adding new columns as necessary.
  Column,
  // Combines `Row` with the dense packing algorithm.
  RowDense,
  // Combines `Column` with the dense packing algorithm.
  ColumnDense,
} TaffyGridAutoFlow;

// How children overflowing their container should affect layout
//
// In CSS the primary effect of this property is to control whether contents of a parent container that overflow that container should
// be displayed anyway, be clipped, or trigger the container to become a scroll container. However it also has secondary effects on layout,
// the main ones being:
//
//   - The automatic minimum size Flexbox/CSS Grid items with non-`Visible` overflow is `0` rather than being content based
//   - `Overflow::Scroll` nodes have space in the layout reserved for a scrollbar (width controlled by the `scrollbar_width` property)
//
// In Taffy, we only implement the layout related secondary effects as we are not concerned with drawing/painting. The amount of space reserved for
// a scrollbar is controlled by the `scrollbar_width` property. If this is `0` then `Scroll` behaves identically to `Hidden`.
//
// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>
typedef enum TaffyOverflow {
  // The automatic minimum size of this node as a flexbox/grid item should be based on the size of it's content.
  Visible,
  // The automatic minimum size of this node as a flexbox/grid item should be `0`.
  Hidden,
  // The automatic minimum size of this node as a flexbox/grid item should be `0`. Additionally, space should be reserved
  // for a scrollbar. The amount of space reserved is controlled by the `scrollbar_width` property.
  Scroll,
} TaffyOverflow;

// The positioning strategy for this item.
//
// This controls both how the origin is determined for the [`Style::position`] field,
// and whether or not the item will be controlled by flexbox's layout algorithm.
//
// WARNING: this enum follows the behavior of [CSS's `position` property](https://developer.mozilla.org/en-US/docs/Web/CSS/position),
// which can be unintuitive.
//
// [`Position::Relative`] is the default value, in contrast to the default behavior in CSS.
typedef enum TaffyPosition {
  // The offset is computed relative to the final position given by the layout algorithm.
  // Offsets do not affect the position of any other items; they are effectively a correction factor applied at the end.
  Relative,
  // The offset is computed relative to this item's closest positioned ancestor, if any.
  // Otherwise, it is placed relative to the origin.
  // No space is created for the item in the page layout, and its size will not be altered.
  //
  // WARNING: to opt-out of layouting entirely, you must use [`Display::None`] instead on your [`Style`] object.
  Absolute,
} TaffyPosition;

typedef struct TaffyStyle TaffyStyle;

typedef struct IntResult {
  enum ReturnCode return_code;
  int32_t value;
} IntResult;

typedef struct StyleValue {
  // The value. If the unit is variant that doesn't require a value (e.g. Auto) then the value is ignored.
  float value;
  enum StyleValueUnit unit;
} StyleValue;

typedef struct StyleValueResult {
  enum ReturnCode return_code;
  struct StyleValue value;
} StyleValueResult;

typedef struct FloatResult {
  enum ReturnCode return_code;
  float value;
} FloatResult;

// For all fields, zero represents not set
typedef struct GridPlacement {
  int16_t start;
  int16_t end;
  uint16_t span;
} GridPlacement;

typedef struct GridPlacementResult {
  enum ReturnCode return_code;
  struct GridPlacement value;
} GridPlacementResult;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct IntResult TaffyStyle_GetDisplay(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetDisplay(struct TaffyStyle *raw_style, enum TaffyDisplay value);

struct IntResult TaffyStyle_GetPosition(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetPosition(struct TaffyStyle *raw_style, enum TaffyPosition value);

struct IntResult TaffyStyle_GetOverflowX(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetOverflowX(struct TaffyStyle *raw_style, enum TaffyOverflow value);

struct IntResult TaffyStyle_GetOverflowY(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetOverflowY(struct TaffyStyle *raw_style, enum TaffyOverflow value);

struct IntResult TaffyStyle_GetAlignContent(const struct TaffyStyle *raw_style);

struct IntResult TaffyStyle_GetAlignItems(const struct TaffyStyle *raw_style);

struct IntResult TaffyStyle_GetAlignSelf(const struct TaffyStyle *raw_style);

struct IntResult TaffyStyle_GetJustifyContent(const struct TaffyStyle *raw_style);

struct IntResult TaffyStyle_GetJustifyItems(const struct TaffyStyle *raw_style);

struct IntResult TaffyStyle_GetJustifySelf(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetAlignContent(struct TaffyStyle *raw_style, enum TaffyAlignContent value);

enum ReturnCode TaffyStyle_SetAlignItems(struct TaffyStyle *raw_style, enum TaffyAlignItems value);

enum ReturnCode TaffyStyle_SetAlignSelf(struct TaffyStyle *raw_style, enum TaffyAlignItems value);

enum ReturnCode TaffyStyle_SetJustifyContent(struct TaffyStyle *raw_style, enum TaffyAlignContent value);

enum ReturnCode TaffyStyle_SetJustifyItems(struct TaffyStyle *raw_style, enum TaffyAlignItems value);

enum ReturnCode TaffyStyle_SetJustifySelf(struct TaffyStyle *raw_style, enum TaffyAlignItems value);

struct IntResult TaffyStyle_GetFlexDirection(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetFlexDirection(struct TaffyStyle *raw_style, enum TaffyFlexDirection value);

struct IntResult TaffyStyle_GetFlexWrap(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetFlexWrap(struct TaffyStyle *raw_style, enum TaffyFlexWrap value);

struct IntResult TaffyStyle_GetGridAutoFlow(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetGridAutoFlow(struct TaffyStyle *raw_style, enum TaffyGridAutoFlow value);

struct StyleValueResult TaffyStyle_GetWidth(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetWidth(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetHeight(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetHeight(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetMinWidth(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetMinWidth(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetMinHeight(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetMinHeight(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetMaxWidth(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetMaxWidth(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetMaxHeight(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetMaxHeight(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetInsetTop(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetInsetTop(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetInsetBottom(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetInsetBottom(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetInsetLeft(const struct TaffyStyle *raw_style);

struct StyleValueResult TaffyStyle_GetInsetRight(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetInsetLeft(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

enum ReturnCode TaffyStyle_SetInsetRight(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetMarginTop(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetMarginTop(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetMarginBottom(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetMarginBottom(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetMarginLeft(const struct TaffyStyle *raw_style);

struct StyleValueResult TaffyStyle_GetMarginRight(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetMarginLeft(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

enum ReturnCode TaffyStyle_SetMarginRight(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetPaddingTop(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetPaddingTop(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetPaddingBottom(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetPaddingBottom(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetPaddingLeft(const struct TaffyStyle *raw_style);

struct StyleValueResult TaffyStyle_GetPaddingRight(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetPaddingLeft(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

enum ReturnCode TaffyStyle_SetPaddingRight(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetBorderTop(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetBorderTop(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetBorderBottom(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetBorderBottom(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetBorderLeft(const struct TaffyStyle *raw_style);

struct StyleValueResult TaffyStyle_GetBorderRight(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetBorderLeft(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

enum ReturnCode TaffyStyle_SetBorderRight(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetColumnGap(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetColumnGap(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct StyleValueResult TaffyStyle_GetRowGap(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetRowGap(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct FloatResult TaffyStyle_GetAspectRatio(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetAspectRatio(struct TaffyStyle *raw_style, float value);

struct FloatResult TaffyStyle_GetScrollbarWidth(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetScrollbarWidth(struct TaffyStyle *raw_style, float value);

struct StyleValueResult TaffyStyle_GetFlexBasis(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetFlexBasis(struct TaffyStyle *raw_style, float value, enum StyleValueUnit unit);

struct FloatResult TaffyStyle_GetFlexGrow(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetFlexGrow(struct TaffyStyle *raw_style, float value);

struct FloatResult TaffyStyle_GetFlexShrink(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetFlexShrink(struct TaffyStyle *raw_style, float value);

// Function to set all the value of margin
enum ReturnCode TaffyStyle_SetMargin(struct TaffyStyle *raw_style, enum TaffyEdge edge, struct StyleValue value);

// Get grid item's column placement
struct GridPlacementResult TaffyStyleGetGridColumn(struct TaffyStyle *raw_style);

// Set grid item's column placement
enum ReturnCode TaffyStyleSetGridColumn(struct TaffyStyle *raw_style, struct GridPlacement placement);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
