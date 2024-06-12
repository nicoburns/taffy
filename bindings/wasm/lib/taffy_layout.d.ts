/* tslint:disable */
/* eslint-disable */
/**
 * Controls whether flex items are forced onto one line or can wrap onto multiple lines.
 *
 * Defaults to [`FlexWrap::NoWrap`]
 *
 * [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-wrap-property)
 */
export enum FlexWrap {
  /**
   * Items will not wrap and stay on a single line
   */
  NoWrap = 0,
  /**
   * Items will wrap according to this item's [`FlexDirection`]
   */
  Wrap = 1,
  /**
   * Items will wrap in the opposite direction to this item's [`FlexDirection`]
   */
  WrapReverse = 2,
}
/**
 * Used to control how child nodes are aligned.
 * For Flexbox it controls alignment in the cross axis
 * For Grid it controls alignment in the block axis
 *
 * [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-items)
 */
export enum AlignItems {
  /**
   * Items are packed toward the start of the axis
   */
  Start = 0,
  /**
   * Items are packed toward the end of the axis
   */
  End = 1,
  /**
   * Items are packed towards the flex-relative start of the axis.
   *
   * For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
   * to End. In all other cases it is equivalent to Start.
   */
  FlexStart = 2,
  /**
   * Items are packed towards the flex-relative end of the axis.
   *
   * For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
   * to Start. In all other cases it is equivalent to End.
   */
  FlexEnd = 3,
  /**
   * Items are packed along the center of the cross axis
   */
  Center = 4,
  /**
   * Items are aligned such as their baselines align
   */
  Baseline = 5,
  /**
   * Stretch to fill the container
   */
  Stretch = 6,
}
/**
 * Sets the layout used for the children of this node
 *
 * The default values depends on on which feature flags are enabled. The order of precedence is: Flex, Grid, Block, None.
 */
export enum Display {
  /**
   * The children will follow the block layout algorithm
   */
  Block = 0,
  /**
   * The children will follow the flexbox layout algorithm
   */
  Flex = 1,
  /**
   * The children will follow the CSS Grid layout algorithm
   */
  Grid = 2,
  /**
   * The element and it's children will not be laid out and will behave as if they
   * did not exist.
   */
  None = 3,
}
/** */
export enum StyleUnit {
  Px = 0,
  Percent = 1,
  Auto = 2,
  MinContent = 3,
  MaxContent = 4,
  FitContentPx = 5,
  FitContentPercent = 6,
  Fr = 7,
}
/**
 * The positioning strategy for this item.
 *
 * This controls both how the origin is determined for the [`Style::position`] field,
 * and whether or not the item will be controlled by flexbox's layout algorithm.
 *
 * WARNING: this enum follows the behavior of [CSS's `position` property](https://developer.mozilla.org/en-US/docs/Web/CSS/position),
 * which can be unintuitive.
 *
 * [`Position::Relative`] is the default value, in contrast to the default behavior in CSS.
 */
export enum Position {
  /**
   * The offset is computed relative to the final position given by the layout algorithm.
   * Offsets do not affect the position of any other items; they are effectively a correction factor applied at the end.
   */
  Relative = 0,
  /**
   * The offset is computed relative to this item's closest positioned ancestor, if any.
   * Otherwise, it is placed relative to the origin.
   * No space is created for the item in the page layout, and its size will not be altered.
   *
   * WARNING: to opt-out of layouting entirely, you must use [`Display::None`] instead on your [`Style`] object.
   */
  Absolute = 1,
}
/**
 * How children overflowing their container should affect layout
 *
 * In CSS the primary effect of this property is to control whether contents of a parent container that overflow that container should
 * be displayed anyway, be clipped, or trigger the container to become a scroll container. However it also has secondary effects on layout,
 * the main ones being:
 *
 *   - The automatic minimum size Flexbox/CSS Grid items with non-`Visible` overflow is `0` rather than being content based
 *   - `Overflow::Scroll` nodes have space in the layout reserved for a scrollbar (width controlled by the `scrollbar_width` property)
 *
 * In Taffy, we only implement the layout related secondary effects as we are not concerned with drawing/painting. The amount of space reserved for
 * a scrollbar is controlled by the `scrollbar_width` property. If this is `0` then `Scroll` behaves identically to `Hidden`.
 *
 * <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>
 */
export enum Overflow {
  /**
   * The automatic minimum size of this node as a flexbox/grid item should be based on the size of its content.
   * Content that overflows this node *should* contribute to the scroll region of its parent.
   */
  Visible = 0,
  /**
   * The automatic minimum size of this node as a flexbox/grid item should be based on the size of its content.
   * Content that overflows this node should *not* contribute to the scroll region of its parent.
   */
  Clip = 1,
  /**
   * The automatic minimum size of this node as a flexbox/grid item should be `0`.
   * Content that overflows this node should *not* contribute to the scroll region of its parent.
   */
  Hidden = 2,
  /**
   * The automatic minimum size of this node as a flexbox/grid item should be `0`. Additionally, space should be reserved
   * for a scrollbar. The amount of space reserved is controlled by the `scrollbar_width` property.
   * Content that overflows this node should *not* contribute to the scroll region of its parent.
   */
  Scroll = 3,
}
/**
 * Sets the distribution of space between and around content items
 * For Flexbox it controls alignment in the cross axis
 * For Grid it controls alignment in the block axis
 *
 * [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-content)
 */
export enum AlignContent {
  /**
   * Items are packed toward the start of the axis
   */
  Start = 0,
  /**
   * Items are packed toward the end of the axis
   */
  End = 1,
  /**
   * Items are packed towards the flex-relative start of the axis.
   *
   * For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
   * to End. In all other cases it is equivalent to Start.
   */
  FlexStart = 2,
  /**
   * Items are packed towards the flex-relative end of the axis.
   *
   * For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
   * to Start. In all other cases it is equivalent to End.
   */
  FlexEnd = 3,
  /**
   * Items are centered around the middle of the axis
   */
  Center = 4,
  /**
   * Items are stretched to fill the container
   */
  Stretch = 5,
  /**
   * The first and last items are aligned flush with the edges of the container (no gap)
   * The gap between items is distributed evenly.
   */
  SpaceBetween = 6,
  /**
   * The gap between the first and last items is exactly THE SAME as the gap between items.
   * The gaps are distributed evenly
   */
  SpaceEvenly = 7,
  /**
   * The gap between the first and last items is exactly HALF the gap between items.
   * The gaps are distributed evenly in proportion to these ratios.
   */
  SpaceAround = 8,
}
/**
 * Controls whether grid items are placed row-wise or column-wise. And whether the sparse or dense packing algorithm is used.
 *
 * The "dense" packing algorithm attempts to fill in holes earlier in the grid, if smaller items come up later. This may cause items to appear out-of-order, when doing so would fill in holes left by larger items.
 *
 * Defaults to [`GridAutoFlow::Row`]
 *
 * [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow)
 */
export enum GridAutoFlow {
  /**
   * Items are placed by filling each row in turn, adding new rows as necessary
   */
  Row = 0,
  /**
   * Items are placed by filling each column in turn, adding new columns as necessary.
   */
  Column = 1,
  /**
   * Combines `Row` with the dense packing algorithm.
   */
  RowDense = 2,
  /**
   * Combines `Column` with the dense packing algorithm.
   */
  ColumnDense = 3,
}
/**
 * The direction of the flexbox layout main axis.
 *
 * There are always two perpendicular layout axes: main (or primary) and cross (or secondary).
 * Adding items will cause them to be positioned adjacent to each other along the main axis.
 * By varying this value throughout your tree, you can create complex axis-aligned layouts.
 *
 * Items are always aligned relative to the cross axis, and justified relative to the main axis.
 *
 * The default behavior is [`FlexDirection::Row`].
 *
 * [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-direction-property)
 */
export enum FlexDirection {
  /**
   * Defines +x as the main axis
   *
   * Items will be added from left to right in a row.
   */
  Row = 0,
  /**
   * Defines +y as the main axis
   *
   * Items will be added from top to bottom in a column.
   */
  Column = 1,
  /**
   * Defines -x as the main axis
   *
   * Items will be added from right to left in a row.
   */
  RowReverse = 2,
  /**
   * Defines -y as the main axis
   *
   * Items will be added from bottom to top in a column.
   */
  ColumnReverse = 3,
}
/** */
export class Layout {
  free(): void;
  /**
   * @param {number} at
   * @returns {Layout}
   */
  child(at: number): Layout;
  /** */
  readonly childCount: number;
  /** */
  readonly height: number;
  /** */
  readonly width: number;
  /** */
  readonly x: number;
  /** */
  readonly y: number;
}
/** */
export class Node {
  free(): void;
  /**
   * @param {TaffyTree} tree
   */
  constructor(tree: TaffyTree);
  /**
   * @param {any} measure
   */
  setMeasure(measure: any): void;
  /**
   * @param {Node} child
   */
  addChild(child: Node): void;
  /**
   * @param {Node} child
   */
  removeChild(child: Node): void;
  /**
   * @param {number} index
   * @param {Node} child
   */
  replaceChildAtIndex(index: number, child: Node): void;
  /**
   * @param {number} index
   */
  removeChildAtIndex(index: number): void;
  /** */
  markDirty(): void;
  /**
   * @returns {boolean}
   */
  isDirty(): boolean;
  /**
   * @returns {number}
   */
  childCount(): number;
  /**
   * @param {any} size
   * @returns {Layout}
   */
  computeLayout(size: any): Layout;
  /**
   * @returns {Display}
   */
  getDisplay(): Display;
  /**
   * @param {Display} value
   */
  setDisplay(value: Display): void;
  /**
   * @returns {Position}
   */
  getPosition(): Position;
  /**
   * @param {Position} value
   */
  setPosition(value: Position): void;
  /**
   * @returns {Overflow}
   */
  getOverflowX(): Overflow;
  /**
   * @param {Overflow} value
   */
  setOverflowX(value: Overflow): void;
  /**
   * @returns {Overflow}
   */
  getOverflowY(): Overflow;
  /**
   * @param {Overflow} value
   */
  setOverflowY(value: Overflow): void;
  /**
   * @param {Overflow} value
   */
  setOverflow(value: Overflow): void;
  /**
   * @param {number} value
   */
  setScrollbarWidth(value: number): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setInsetTop(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setInsetBottom(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setInsetLeft(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setInsetRight(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setInsetHorizontal(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setInsetVertical(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setInsetAll(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setWidth(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setHeight(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setMinWidth(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setMinHeight(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setMaxWidth(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setMaxHeight(value: number, unit: StyleUnit): void;
  /**
   * @param {number | undefined} [value]
   */
  setAspectRatio(value?: number): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setPaddingTop(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setPaddingBottom(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setPaddingLeft(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setPaddingRight(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setPaddingHorizontal(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setPaddingVertical(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setPaddingAll(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setMarginTop(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setMarginBottom(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setMarginLeft(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setMarginRight(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setMarginHorizontal(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setMarginVertical(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setMarginAll(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setBorderWidthTop(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setBorderWidthBottom(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setBorderWidthLeft(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setBorderWidthRight(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setBorderWidthHorizontal(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setBorderWidthVertical(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setBorderWidthAll(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setRowGap(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setColumnGap(value: number, unit: StyleUnit): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setGap(value: number, unit: StyleUnit): void;
  /**
   * @param {AlignContent | undefined} [value]
   */
  setAlignContent(value?: AlignContent): void;
  /**
   * @param {AlignContent | undefined} [value]
   */
  setJustifyContent(value?: AlignContent): void;
  /**
   * @param {AlignItems | undefined} [value]
   */
  setAlignItems(value?: AlignItems): void;
  /**
   * @param {AlignItems | undefined} [value]
   */
  setJustifyItems(value?: AlignItems): void;
  /**
   * @param {AlignItems | undefined} [value]
   */
  setAlignSelf(value?: AlignItems): void;
  /**
   * @param {AlignItems | undefined} [value]
   */
  setJustifySelf(value?: AlignItems): void;
  /**
   * @param {FlexDirection} value
   */
  setFlexDirection(value: FlexDirection): void;
  /**
   * @param {FlexWrap} value
   */
  setFlexWrap(value: FlexWrap): void;
  /**
   * @param {number} value
   */
  setFlexGrow(value: number): void;
  /**
   * @param {number} value
   */
  setFlexShrink(value: number): void;
  /**
   * @param {number} value
   * @param {StyleUnit} unit
   */
  setFlexBasis(value: number, unit: StyleUnit): void;
  /**
   * @param {GridAutoFlow} value
   */
  setGridAutoFlow(value: GridAutoFlow): void;
}
/** */
export class TaffyTree {
  free(): void;
  /** */
  constructor();
}

export function instantiate(): Promise<{ TaffyTree: typeof TaffyTree; Node: typeof Node }>;
