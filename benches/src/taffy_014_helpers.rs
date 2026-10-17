use rand::distr::uniform::SampleRange;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use taffy::style::Style as TaffyStyle;

use super::{BuildTree, BuildTreeExt, GenStyle};

pub struct Taffy014TreeBuilder<R: Rng, G: GenStyle<TaffyStyle>> {
    rng: R,
    style_generator: G,
    tree: taffy_014::TaffyTree,
    root: taffy_014::NodeId,
    nodes: Vec<taffy_014::NodeId>,
}

// Implement the BuildTree trait
impl<R: Rng, G: GenStyle<TaffyStyle>> BuildTree<R, G> for Taffy014TreeBuilder<R, G> {
    const NAME: &'static str = "Taffy 0.14";
    type Tree = taffy_014::TaffyTree;
    type Node = taffy_014::NodeId;

    fn with_rng(mut rng: R, mut style_generator: G) -> Self {
        let mut tree = taffy_014::TaffyTree::new();
        let root = tree.new_leaf(convert_style(style_generator.create_root_style(&mut rng))).unwrap();
        Taffy014TreeBuilder { rng, style_generator, tree, root, nodes: vec![root] }
    }

    fn compute_layout_inner(&mut self, available_width: Option<f32>, available_height: Option<f32>) {
        let available_space =
            taffy_014::geometry::Size { width: available_width.into(), height: available_height.into() };
        self.tree.compute_layout(self.root, available_space).unwrap();
    }

    fn mark_all_dirty(&mut self) {
        for &node in &self.nodes {
            self.tree.mark_dirty(node).unwrap();
        }
    }

    fn random_usize(&mut self, range: impl SampleRange<usize>) -> usize {
        self.rng.random_range(range)
    }

    fn create_leaf_node(&mut self) -> Self::Node {
        let style = self.style_generator.create_leaf_style(&mut self.rng);
        let node = self.tree.new_leaf(convert_style(style)).unwrap();
        self.nodes.push(node);
        node
    }

    fn create_container_node(&mut self, children: &[Self::Node]) -> Self::Node {
        let style = self.style_generator.create_container_style(&mut self.rng);
        let node = self.tree.new_with_children(convert_style(style), children).unwrap();
        self.nodes.push(node);
        node
    }

    fn create_node_with_style(&mut self, style: TaffyStyle, children: &[Self::Node]) -> Self::Node {
        let node = self.tree.new_with_children(convert_style(style), children).unwrap();
        self.nodes.push(node);
        node
    }

    fn total_node_count(&mut self) -> usize {
        self.tree.total_node_count()
    }

    fn set_root_children(&mut self, children: &[Self::Node]) {
        self.tree.set_children(self.root, children).unwrap();
    }

    fn into_tree_and_root(self) -> (Self::Tree, Self::Node) {
        (self.tree, self.root)
    }
}

impl<G: GenStyle<TaffyStyle>> BuildTreeExt<G> for Taffy014TreeBuilder<ChaCha8Rng, G> {}

/// A Taffy 0.14 copy of a tree built with the current version of Taffy, along with what is needed to lay it out
/// from scratch repeatedly
pub struct Taffy014LayoutTree<NodeContext = ()> {
    pub tree: taffy_014::TaffyTree<NodeContext>,
    pub root: taffy_014::NodeId,
    nodes: Vec<taffy_014::NodeId>,
}

impl Taffy014LayoutTree {
    pub fn new(tree_and_root: (taffy::TaffyTree, taffy::NodeId)) -> Self {
        Self::convert(tree_and_root, |&()| ())
    }

    pub fn compute_layout(&mut self, available_space: taffy::Size<taffy::AvailableSpace>) {
        self.tree.compute_layout(self.root, available_space_to_014(available_space)).unwrap();
    }
}

impl<NodeContext> Taffy014LayoutTree<NodeContext> {
    /// Copy a tree, using `convert_context` to copy the context of each node that has one
    pub fn convert<C>(
        (tree, root): (taffy::TaffyTree<C>, taffy::NodeId),
        mut convert_context: impl FnMut(&C) -> NodeContext,
    ) -> Self {
        let mut new_tree = taffy_014::TaffyTree::new();
        let mut nodes = Vec::new();
        let root = convert_node(&tree, root, &mut new_tree, &mut nodes, &mut convert_context);
        Self { tree: new_tree, root, nodes }
    }

    /// Clear the cached layout of every node in the tree
    pub fn mark_all_dirty(&mut self) {
        for &node in &self.nodes {
            self.tree.mark_dirty(node).unwrap();
        }
    }
}

fn convert_node<C, NodeContext>(
    tree: &taffy::TaffyTree<C>,
    node: taffy::NodeId,
    new_tree: &mut taffy_014::TaffyTree<NodeContext>,
    nodes: &mut Vec<taffy_014::NodeId>,
    convert_context: &mut impl FnMut(&C) -> NodeContext,
) -> taffy_014::NodeId {
    let children: Vec<_> = tree
        .children(node)
        .unwrap()
        .into_iter()
        .map(|child| convert_node(tree, child, new_tree, nodes, convert_context))
        .collect();
    let style = convert_style(tree.style(node).unwrap().clone());
    let new_node = match tree.get_node_context(node) {
        Some(context) => new_tree.new_leaf_with_context(style, convert_context(context)).unwrap(),
        None => new_tree.new_leaf(style).unwrap(),
    };
    new_tree.set_children(new_node, &children).unwrap();
    nodes.push(new_node);
    new_node
}

pub fn available_space_to_014(input: taffy::Size<taffy::AvailableSpace>) -> taffy_014::Size<taffy_014::AvailableSpace> {
    let map = |space| match space {
        taffy::AvailableSpace::Definite(value) => taffy_014::AvailableSpace::Definite(value),
        taffy::AvailableSpace::MinContent => taffy_014::AvailableSpace::MinContent,
        taffy::AvailableSpace::MaxContent => taffy_014::AvailableSpace::MaxContent,
    };
    taffy_014::Size { width: map(input.width), height: map(input.height) }
}

pub fn available_space_from_014(
    input: taffy_014::Size<taffy_014::AvailableSpace>,
) -> taffy::Size<taffy::AvailableSpace> {
    let map = |space| match space {
        taffy_014::AvailableSpace::Definite(value) => taffy::AvailableSpace::Definite(value),
        taffy_014::AvailableSpace::MinContent => taffy::AvailableSpace::MinContent,
        taffy_014::AvailableSpace::MaxContent => taffy::AvailableSpace::MaxContent,
    };
    taffy::Size { width: map(input.width), height: map(input.height) }
}

/// Converts every style property, following CSS semantics for values that Taffy 0.14 can't represent: they leave
/// the property at its default (as a browser drops a declaration that it can't parse). The input is destructured
/// exhaustively and the output is built without `..Default::default()`, so a property that is added to Taffy is a
/// compile error here rather than being dropped unnoticed.
fn convert_style(style: taffy::style::Style) -> taffy_014::style::Style {
    let taffy::style::Style {
        dummy: _,
        display,
        item_is_table,
        item_is_replaced,
        // Taffy 0.14 has no equivalent
        item_is_compressible_replaced: _,
        box_sizing,
        direction,
        overflow,
        scrollbar_width,
        contain,
        float,
        clear,
        position,
        inset,
        size,
        min_size,
        max_size,
        aspect_ratio,
        margin,
        padding,
        border,
        align_items,
        align_self,
        justify_items,
        justify_self,
        align_content,
        justify_content,
        gap,
        text_align,
        flex_direction,
        flex_wrap,
        flex_line_count,
        flex_basis,
        flex_grow,
        flex_shrink,
        grid_template_rows,
        grid_template_columns,
        grid_auto_rows,
        grid_auto_columns,
        grid_auto_flow,
        grid_template_areas,
        grid_template_column_names,
        grid_template_row_names,
        grid_row,
        grid_column,
    } = style;

    let default = taffy_014::style::Style::<String>::DEFAULT;
    taffy_014::style::Style {
        dummy: core::marker::PhantomData,
        display: convert_display(display),
        item_is_table,
        item_is_replaced,
        box_sizing: convert_box_sizing(box_sizing),
        direction: convert_direction(direction),
        overflow: convert_point(overflow, convert_overflow),
        scrollbar_width,
        contain: convert_contain(contain),
        float: convert_float(float),
        clear: convert_clear(clear),
        position: convert_position(position).unwrap_or(default.position),
        inset: convert_rect(inset, convert_length_percentage_auto, default.inset),
        size: convert_size(size, convert_dimension, default.size),
        min_size: convert_size(min_size, convert_length_percentage_auto, default.min_size),
        max_size: convert_size(max_size, convert_length_percentage_auto, default.max_size),
        aspect_ratio,
        margin: convert_rect(margin, convert_length_percentage_auto, default.margin),
        padding: convert_rect(padding, convert_length_percentage, default.padding),
        border: convert_rect(border, convert_length_percentage, default.border),
        gap: convert_size(gap, convert_length_percentage, default.gap),
        // Alignment
        align_items: convert_align_items(align_items),
        align_self: align_self.and_then(convert_align_items),
        justify_items: convert_align_items(justify_items),
        justify_self: justify_self.and_then(convert_align_items),
        align_content: convert_align_content(align_content),
        justify_content: convert_align_content(justify_content),
        // Block
        text_align: convert_text_align(text_align),
        // Flexbox
        flex_direction: convert_flex_direction(flex_direction),
        flex_wrap: convert_flex_wrap(flex_wrap),
        flex_line_count,
        flex_basis: convert_dimension(flex_basis).unwrap_or(default.flex_basis),
        flex_grow,
        flex_shrink,
        // Grid
        grid_template_rows: convert_grid_template(grid_template_rows).unwrap_or(default.grid_template_rows),
        grid_template_columns: convert_grid_template(grid_template_columns).unwrap_or(default.grid_template_columns),
        grid_auto_rows: convert_grid_auto_tracks(grid_auto_rows).unwrap_or(default.grid_auto_rows),
        grid_auto_columns: convert_grid_auto_tracks(grid_auto_columns).unwrap_or(default.grid_auto_columns),
        grid_auto_flow: convert_grid_auto_flow(grid_auto_flow),
        grid_template_areas: grid_template_areas.map(convert_grid_template_areas),
        grid_template_row_names,
        grid_template_column_names,
        grid_row: convert_line(grid_row, convert_grid_placement),
        grid_column: convert_line(grid_column, convert_grid_placement),
    }
}

fn convert_rect<T, U>(
    input: taffy::geometry::Rect<T>,
    map: impl Fn(T) -> Option<U>,
    default: taffy_014::geometry::Rect<U>,
) -> taffy_014::geometry::Rect<U> {
    taffy_014::geometry::Rect {
        left: map(input.left).unwrap_or(default.left),
        right: map(input.right).unwrap_or(default.right),
        top: map(input.top).unwrap_or(default.top),
        bottom: map(input.bottom).unwrap_or(default.bottom),
    }
}

fn convert_size<T, U>(
    input: taffy::geometry::Size<T>,
    map: impl Fn(T) -> Option<U>,
    default: taffy_014::geometry::Size<U>,
) -> taffy_014::geometry::Size<U> {
    taffy_014::geometry::Size {
        width: map(input.width).unwrap_or(default.width),
        height: map(input.height).unwrap_or(default.height),
    }
}

fn convert_point<T, U, F: Fn(T) -> U>(input: taffy::geometry::Point<T>, map: F) -> taffy_014::geometry::Point<U> {
    taffy_014::geometry::Point { x: map(input.x), y: map(input.y) }
}

fn convert_line<T, U, F: Fn(T) -> U>(input: taffy::geometry::Line<T>, map: F) -> taffy_014::geometry::Line<U> {
    taffy_014::geometry::Line { start: map(input.start), end: map(input.end) }
}

// `calc()` values are opaque pointers that are resolved by the tree that owns them, so they can't be converted
fn convert_dimension(input: taffy::style::Dimension) -> Option<taffy_014::style::Dimension> {
    use taffy::style::ExpandedDimension as E;
    use taffy_014::style::Dimension;
    match input.expand() {
        E::Length(value) => Some(Dimension::length(value)),
        E::Percent(value) => Some(Dimension::percent(value)),
        E::Auto => Some(Dimension::auto()),
        E::MinContent => Some(Dimension::min_content()),
        E::MaxContent => Some(Dimension::max_content()),
        E::FitContent => Some(Dimension::fit_content()),
        E::FitContentPx(limit) => Some(Dimension::fit_content_px(limit)),
        E::FitContentPercent(limit) => Some(Dimension::fit_content_percent(limit)),
        E::Stretch => Some(Dimension::stretch()),
        E::Content => Some(Dimension::content()),
        E::FitContentCalc(_) | E::Calc(_) => None,
    }
}

fn convert_length_percentage_auto(
    input: taffy::style::LengthPercentageAuto,
) -> Option<taffy_014::style::LengthPercentageAuto> {
    use taffy::style::ExpandedLengthPercentageAuto as E;
    match input.expand() {
        E::Length(value) => Some(taffy_014::style::LengthPercentageAuto::length(value)),
        E::Percent(value) => Some(taffy_014::style::LengthPercentageAuto::percent(value)),
        E::Auto => Some(taffy_014::style::LengthPercentageAuto::auto()),
        E::Calc(_) => None,
    }
}

fn convert_length_percentage(input: taffy::style::LengthPercentage) -> Option<taffy_014::style::LengthPercentage> {
    use taffy::style::ExpandedLengthPercentage as E;
    match input.expand() {
        E::Length(value) => Some(taffy_014::style::LengthPercentage::length(value)),
        E::Percent(value) => Some(taffy_014::style::LengthPercentage::percent(value)),
        E::Calc(_) => None,
    }
}

fn convert_min_track_sizing_function(
    input: taffy::style::MinTrackSizingFunction,
) -> Option<taffy_014::style::MinTrackSizingFunction> {
    use taffy::style::ExpandedMinTrackSizingFunction as E;
    use taffy_014::style::MinTrackSizingFunction;
    match input.expand() {
        E::Length(value) => Some(MinTrackSizingFunction::length(value)),
        E::Percent(value) => Some(MinTrackSizingFunction::percent(value)),
        E::Auto => Some(MinTrackSizingFunction::auto()),
        E::MinContent => Some(MinTrackSizingFunction::min_content()),
        E::MaxContent => Some(MinTrackSizingFunction::max_content()),
        E::Calc(_) => None,
    }
}

fn convert_max_track_sizing_function(
    input: taffy::style::MaxTrackSizingFunction,
) -> Option<taffy_014::style::MaxTrackSizingFunction> {
    use taffy::style::ExpandedMaxTrackSizingFunction as E;
    use taffy_014::style::MaxTrackSizingFunction;
    match input.expand() {
        E::Length(value) => Some(MaxTrackSizingFunction::length(value)),
        E::Percent(value) => Some(MaxTrackSizingFunction::percent(value)),
        E::Auto => Some(MaxTrackSizingFunction::auto()),
        E::MinContent => Some(MaxTrackSizingFunction::min_content()),
        E::MaxContent => Some(MaxTrackSizingFunction::max_content()),
        E::FitContentPx(limit) => Some(MaxTrackSizingFunction::fit_content_px(limit)),
        E::FitContentPercent(limit) => Some(MaxTrackSizingFunction::fit_content_percent(limit)),
        E::Fr(fraction) => Some(MaxTrackSizingFunction::fr(fraction)),
        E::FitContentCalc(_) | E::Calc(_) => None,
    }
}

fn convert_track_sizing_function(
    input: taffy::style::TrackSizingFunction,
) -> Option<taffy_014::style::TrackSizingFunction> {
    Some(taffy_014::geometry::MinMax {
        min: convert_min_track_sizing_function(input.min)?,
        max: convert_max_track_sizing_function(input.max)?,
    })
}

fn convert_grid_auto_tracks(
    input: Vec<taffy::style::TrackSizingFunction>,
) -> Option<Vec<taffy_014::style::TrackSizingFunction>> {
    input.into_iter().map(convert_track_sizing_function).collect()
}

fn convert_grid_template(
    input: Vec<taffy::style::GridTemplateComponent<String>>,
) -> Option<Vec<taffy_014::style::GridTemplateComponent<String>>> {
    input.into_iter().map(convert_grid_template_component).collect()
}

fn convert_grid_template_component(
    input: taffy::style::GridTemplateComponent<String>,
) -> Option<taffy_014::style::GridTemplateComponent<String>> {
    Some(match input {
        taffy::style::GridTemplateComponent::Single(track) => {
            taffy_014::style::GridTemplateComponent::Single(convert_track_sizing_function(track)?)
        }
        taffy::style::GridTemplateComponent::Repeat(repetition) => {
            taffy_014::style::GridTemplateComponent::Repeat(taffy_014::style::GridTemplateRepetition {
                count: match repetition.count {
                    taffy::style::RepetitionCount::AutoFill => taffy_014::style::RepetitionCount::AutoFill,
                    taffy::style::RepetitionCount::AutoFit => taffy_014::style::RepetitionCount::AutoFit,
                    taffy::style::RepetitionCount::Count(count) => taffy_014::style::RepetitionCount::Count(count),
                },
                tracks: repetition.tracks.into_iter().map(convert_track_sizing_function).collect::<Option<_>>()?,
                line_names: repetition.line_names,
            })
        }
    })
}

fn convert_grid_auto_flow(input: taffy::style::GridAutoFlow) -> taffy_014::style::GridAutoFlow {
    match input {
        taffy::style::GridAutoFlow::Row => taffy_014::style::GridAutoFlow::Row,
        taffy::style::GridAutoFlow::Column => taffy_014::style::GridAutoFlow::Column,
        taffy::style::GridAutoFlow::RowDense => taffy_014::style::GridAutoFlow::RowDense,
        taffy::style::GridAutoFlow::ColumnDense => taffy_014::style::GridAutoFlow::ColumnDense,
    }
}

fn convert_grid_template_areas(
    input: taffy::style::GridTemplateAreas<String>,
) -> taffy_014::style::GridTemplateAreas<String> {
    taffy_014::style::GridTemplateAreas {
        areas: input
            .areas
            .into_iter()
            .map(|area| taffy_014::style::GridTemplateArea {
                name: area.name,
                row_start: area.row_start,
                row_end: area.row_end,
                column_start: area.column_start,
                column_end: area.column_end,
            })
            .collect(),
        row_count: input.row_count,
        column_count: input.column_count,
    }
}

fn convert_grid_placement(input: taffy::style::GridPlacement<String>) -> taffy_014::style::GridPlacement<String> {
    match input {
        taffy::style::GridPlacement::Auto => taffy_014::style::GridPlacement::Auto,
        taffy::style::GridPlacement::Line(line) => taffy_014::style_helpers::line(line.as_i16()),
        taffy::style::GridPlacement::NamedLine(name, index) => taffy_014::style::GridPlacement::NamedLine(name, index),
        taffy::style::GridPlacement::Span(span) => taffy_014::style::GridPlacement::Span(span),
        taffy::style::GridPlacement::NamedSpan(name, span) => taffy_014::style::GridPlacement::NamedSpan(name, span),
    }
}

fn convert_alignment_safety(input: taffy::style::AlignmentSafety) -> taffy_014::style::AlignmentSafety {
    match input {
        taffy::style::AlignmentSafety::Default | taffy::style::AlignmentSafety::Unsafe => {
            taffy_014::style::AlignmentSafety::Unsafe
        }
        taffy::style::AlignmentSafety::Safe => taffy_014::style::AlignmentSafety::Safe,
    }
}

// Taffy 0.14 has no `normal` keyword: the layout mode's default alignment is represented by `None`
fn convert_align_items(input: taffy::style::AlignItems) -> Option<taffy_014::style::AlignItems> {
    use taffy::style::AlignItemsKeyword as K;
    use taffy_014::style::AlignItemsKeyword as K014;
    let keyword = match input.keyword {
        K::Normal => return None,
        K::Start => K014::Start,
        K::End => K014::End,
        K::FlexStart => K014::FlexStart,
        K::FlexEnd => K014::FlexEnd,
        K::SelfStart => K014::SelfStart,
        K::SelfEnd => K014::SelfEnd,
        K::Center => K014::Center,
        K::Baseline => K014::Baseline,
        K::Stretch => K014::Stretch,
    };
    Some(taffy_014::style::AlignItems { keyword, safety: convert_alignment_safety(input.safety) })
}

fn convert_align_content(input: taffy::style::AlignContent) -> Option<taffy_014::style::AlignContent> {
    use taffy::style::AlignContentKeyword as K;
    use taffy_014::style::AlignContentKeyword as K014;
    let keyword = match input.keyword {
        K::Normal => return None,
        K::Start => K014::Start,
        K::End => K014::End,
        K::FlexStart => K014::FlexStart,
        K::FlexEnd => K014::FlexEnd,
        K::Center => K014::Center,
        K::Stretch => K014::Stretch,
        K::SpaceBetween => K014::SpaceBetween,
        K::SpaceEvenly => K014::SpaceEvenly,
        K::SpaceAround => K014::SpaceAround,
    };
    Some(taffy_014::style::AlignContent { keyword, safety: convert_alignment_safety(input.safety) })
}

fn convert_display(input: taffy::style::Display) -> taffy_014::style::Display {
    match input {
        taffy::style::Display::None => taffy_014::style::Display::None,
        taffy::style::Display::Flex => taffy_014::style::Display::Flex,
        taffy::style::Display::Grid => taffy_014::style::Display::Grid,
        taffy::style::Display::Block => taffy_014::style::Display::Block,
        taffy::style::Display::FlowRoot => taffy_014::style::Display::FlowRoot,
    }
}

fn convert_contain(input: taffy::style::Contain) -> taffy_014::style::Contain {
    let mut contain = taffy_014::style::Contain::NONE;
    if input.contains(taffy::style::Contain::LAYOUT) {
        contain |= taffy_014::style::Contain::LAYOUT;
    }
    if input.contains(taffy::style::Contain::PAINT) {
        contain |= taffy_014::style::Contain::PAINT;
    }
    contain
}

fn convert_float(input: taffy::style::Float) -> taffy_014::style::Float {
    match input {
        taffy::style::Float::Left => taffy_014::style::Float::Left,
        taffy::style::Float::Right => taffy_014::style::Float::Right,
        taffy::style::Float::None => taffy_014::style::Float::None,
    }
}

fn convert_clear(input: taffy::style::Clear) -> taffy_014::style::Clear {
    match input {
        taffy::style::Clear::Left => taffy_014::style::Clear::Left,
        taffy::style::Clear::Right => taffy_014::style::Clear::Right,
        taffy::style::Clear::Both => taffy_014::style::Clear::Both,
        taffy::style::Clear::None => taffy_014::style::Clear::None,
    }
}

fn convert_text_align(input: taffy::style::TextAlign) -> taffy_014::style::TextAlign {
    match input {
        taffy::style::TextAlign::Auto => taffy_014::style::TextAlign::Auto,
        taffy::style::TextAlign::LegacyLeft => taffy_014::style::TextAlign::LegacyLeft,
        taffy::style::TextAlign::LegacyRight => taffy_014::style::TextAlign::LegacyRight,
        taffy::style::TextAlign::LegacyCenter => taffy_014::style::TextAlign::LegacyCenter,
    }
}

fn convert_box_sizing(input: taffy::style::BoxSizing) -> taffy_014::style::BoxSizing {
    match input {
        taffy::style::BoxSizing::BorderBox => taffy_014::style::BoxSizing::BorderBox,
        taffy::style::BoxSizing::ContentBox => taffy_014::style::BoxSizing::ContentBox,
    }
}

fn convert_direction(input: taffy::style::Direction) -> taffy_014::style::Direction {
    match input {
        taffy::style::Direction::Ltr => taffy_014::style::Direction::Ltr,
        taffy::style::Direction::Rtl => taffy_014::style::Direction::Rtl,
    }
}

fn convert_overflow(input: taffy::style::Overflow) -> taffy_014::style::Overflow {
    match input {
        taffy::style::Overflow::Visible => taffy_014::style::Overflow::Visible,
        taffy::style::Overflow::Clip => taffy_014::style::Overflow::Clip,
        taffy::style::Overflow::Hidden => taffy_014::style::Overflow::Hidden,
        taffy::style::Overflow::Scroll => taffy_014::style::Overflow::Scroll,
    }
}

// Taffy 0.14 has no static, fixed or sticky positioning
fn convert_position(input: taffy::style::Position) -> Option<taffy_014::style::Position> {
    match input {
        taffy::style::Position::Relative => Some(taffy_014::style::Position::Relative),
        taffy::style::Position::Absolute => Some(taffy_014::style::Position::Absolute),
        taffy::style::Position::Static | taffy::style::Position::Fixed | taffy::style::Position::Sticky => None,
    }
}

fn convert_flex_direction(input: taffy::style::FlexDirection) -> taffy_014::style::FlexDirection {
    match input {
        taffy::style::FlexDirection::Row => taffy_014::style::FlexDirection::Row,
        taffy::style::FlexDirection::Column => taffy_014::style::FlexDirection::Column,
        taffy::style::FlexDirection::RowReverse => taffy_014::style::FlexDirection::RowReverse,
        taffy::style::FlexDirection::ColumnReverse => taffy_014::style::FlexDirection::ColumnReverse,
    }
}

fn convert_flex_wrap(input: taffy::style::FlexWrap) -> taffy_014::style::FlexWrap {
    match input {
        taffy::style::FlexWrap::NoWrap => taffy_014::style::FlexWrap::NoWrap,
        taffy::style::FlexWrap::Wrap => taffy_014::style::FlexWrap::Wrap,
        taffy::style::FlexWrap::WrapReverse => taffy_014::style::FlexWrap::WrapReverse,
        taffy::style::FlexWrap::Balance => taffy_014::style::FlexWrap::Balance,
        taffy::style::FlexWrap::BalanceReverse => taffy_014::style::FlexWrap::BalanceReverse,
    }
}
