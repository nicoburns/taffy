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

typedef enum TaffyDisplay {
  Block,
  Flex,
  Grid,
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

typedef enum TaffyOverflow {
  Visible,
  Hidden,
  Scroll,
} TaffyOverflow;

typedef enum TaffyPosition {
  Relative,
  Absolute,
} TaffyPosition;

typedef struct TaffyStyle TaffyStyle;

typedef struct FloatResult {
  enum ReturnCode return_code;
  float value;
} FloatResult;

typedef struct StyleValue {
  // The value. If the unit is variant that doesn't require a value (e.g. Auto) then the value is ignored.
  float value;
  enum StyleValueUnit unit;
} StyleValue;

typedef struct StyleValueResult {
  enum ReturnCode return_code;
  struct StyleValue value;
} StyleValueResult;

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

struct FloatResult TaffyStyle_GetDisplay(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetDisplay(struct TaffyStyle *raw_style, enum TaffyDisplay value);

struct FloatResult TaffyStyle_GetPosition(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetPosition(struct TaffyStyle *raw_style, enum TaffyPosition value);

struct FloatResult TaffyStyle_GetOverflowX(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetOverflowX(struct TaffyStyle *raw_style, enum TaffyOverflow value);

struct FloatResult TaffyStyle_GetOverflowY(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetOverflowY(struct TaffyStyle *raw_style, enum TaffyOverflow value);

struct StyleValueResult TaffyStyle_GetWidth(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetWidth(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetHeight(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetHeight(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetMinWidth(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetMinWidth(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetMinHeight(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetMinHeight(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetMaxWidth(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetMaxWidth(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetMaxHeight(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetMaxHeight(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetInsetTop(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetInsetTop(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetInsetBottom(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetInsetBottom(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetInsetLeft(const struct TaffyStyle *raw_style);

struct StyleValueResult TaffyStyle_GetInsetRight(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetInsetLeft(struct TaffyStyle *raw_style, struct StyleValue value);

enum ReturnCode TaffyStyle_SetInsetRight(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetMarginTop(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetMarginTop(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetMarginBottom(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetMarginBottom(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetMarginLeft(const struct TaffyStyle *raw_style);

struct StyleValueResult TaffyStyle_GetMarginRight(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetMarginLeft(struct TaffyStyle *raw_style, struct StyleValue value);

enum ReturnCode TaffyStyle_SetMarginRight(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetPaddingTop(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetPaddingTop(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetPaddingBottom(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetPaddingBottom(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetPaddingLeft(const struct TaffyStyle *raw_style);

struct StyleValueResult TaffyStyle_GetPaddingRight(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetPaddingLeft(struct TaffyStyle *raw_style, struct StyleValue value);

enum ReturnCode TaffyStyle_SetPaddingRight(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetBorderTop(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetBorderTop(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetBorderBottom(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetBorderBottom(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetBorderLeft(const struct TaffyStyle *raw_style);

struct StyleValueResult TaffyStyle_GetBorderRight(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetBorderLeft(struct TaffyStyle *raw_style, struct StyleValue value);

enum ReturnCode TaffyStyle_SetBorderRight(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetColumnGap(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetColumnGap(struct TaffyStyle *raw_style, struct StyleValue value);

struct StyleValueResult TaffyStyle_GetRowGap(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetRowGap(struct TaffyStyle *raw_style, struct StyleValue value);

struct FloatResult TaffyStyle_GetAspectRatio(const struct TaffyStyle *raw_style);

enum ReturnCode TaffyStyle_SetAspectRatio(struct TaffyStyle *raw_style, float value);

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
