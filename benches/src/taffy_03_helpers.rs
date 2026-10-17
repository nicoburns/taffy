use rand::distr::uniform::SampleRange;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use taffy::style::Style as TaffyStyle;
use taffy_03::tree::LayoutTree;

use super::{BuildTree, BuildTreeExt, GenStyle};

pub struct Taffy03TreeBuilder<R: Rng, G: GenStyle<TaffyStyle>> {
    rng: R,
    style_generator: G,
    tree: taffy_03::Taffy,
    root: taffy_03::prelude::Node,
    nodes: Vec<taffy_03::prelude::Node>,
}

// Implement the BuildTree trait
impl<R: Rng, G: GenStyle<TaffyStyle>> BuildTree<R, G> for Taffy03TreeBuilder<R, G> {
    const NAME: &'static str = "Taffy 0.3";
    type Tree = taffy_03::Taffy;
    type Node = taffy_03::prelude::Node;

    fn with_rng(mut rng: R, mut style_generator: G) -> Self {
        let mut tree = taffy_03::Taffy::new();
        let root = tree.new_leaf(convert_style(style_generator.create_root_style(&mut rng))).unwrap();
        Taffy03TreeBuilder { rng, style_generator, tree, root, nodes: vec![root] }
    }

    fn compute_layout_inner(&mut self, available_width: Option<f32>, available_height: Option<f32>) {
        let available_space =
            taffy_03::geometry::Size { width: available_width.into(), height: available_height.into() };
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

impl<G: GenStyle<TaffyStyle>> BuildTreeExt<G> for Taffy03TreeBuilder<ChaCha8Rng, G> {}

/// A Taffy 0.3 copy of a tree built with the current version of Taffy, along with what is needed to lay it out
/// from scratch repeatedly
pub struct Taffy03LayoutTree {
    pub tree: taffy_03::Taffy,
    pub root: taffy_03::prelude::Node,
    nodes: Vec<taffy_03::prelude::Node>,
}

impl Taffy03LayoutTree {
    pub fn new(tree_and_root: (taffy::TaffyTree, taffy::NodeId)) -> Self {
        Self::convert(tree_and_root, |&()| taffy_03::node::MeasureFunc::Raw(|_, _| taffy_03::geometry::Size::ZERO))
    }

    /// Copy a tree, using `create_measure_func` to create the measure function of each node that has a context
    /// (Taffy 0.3 has no node contexts)
    pub fn convert<C>(
        (tree, root): (taffy::TaffyTree<C>, taffy::NodeId),
        mut create_measure_func: impl FnMut(&C) -> taffy_03::node::MeasureFunc,
    ) -> Self {
        let mut new_tree = taffy_03::Taffy::new();
        let mut nodes = Vec::new();
        let root = convert_node(&tree, root, &mut new_tree, &mut nodes, &mut create_measure_func);
        Self { tree: new_tree, root, nodes }
    }

    /// Clear the cached layout of every node in the tree
    pub fn mark_all_dirty(&mut self) {
        for &node in &self.nodes {
            self.tree.mark_dirty(node).unwrap();
        }
    }

    pub fn compute_layout(&mut self, available_space: taffy::Size<taffy::AvailableSpace>) {
        self.tree.compute_layout(self.root, available_space_to_03(available_space)).unwrap();
    }
}

fn convert_node<C>(
    tree: &taffy::TaffyTree<C>,
    node: taffy::NodeId,
    new_tree: &mut taffy_03::Taffy,
    nodes: &mut Vec<taffy_03::prelude::Node>,
    create_measure_func: &mut impl FnMut(&C) -> taffy_03::node::MeasureFunc,
) -> taffy_03::prelude::Node {
    let children: Vec<_> = tree
        .children(node)
        .unwrap()
        .into_iter()
        .map(|child| convert_node(tree, child, new_tree, nodes, create_measure_func))
        .collect();
    let style = convert_style(tree.style(node).unwrap().clone());
    let new_node = match tree.get_node_context(node) {
        Some(context) => new_tree.new_leaf_with_measure(style, create_measure_func(context)).unwrap(),
        None => new_tree.new_leaf(style).unwrap(),
    };
    new_tree.set_children(new_node, &children).unwrap();
    nodes.push(new_node);
    new_node
}

/// Creates a measure function with mutable access to `context`, as the measure functions of later versions of
/// Taffy have.
///
/// Taffy 0.3 measure functions must be `Fn + Send + Sync`. Rather than putting the context behind a lock (which
/// would add overhead that later versions don't have), it is mutated through an `UnsafeCell`. That is only sound
/// while a measure function is never called concurrently, which holds because the benchmarks never share a tree
/// between threads (and Taffy 0.3 never calls a measure function re-entrantly).
pub fn measure_func_with_context<C: Send + 'static>(
    context: C,
    measure: impl Fn(
            &mut C,
            taffy_03::geometry::Size<Option<f32>>,
            taffy_03::geometry::Size<taffy_03::style::AvailableSpace>,
        ) -> taffy_03::geometry::Size<f32>
        + Send
        + Sync
        + 'static,
) -> taffy_03::node::MeasureFunc {
    struct UnsyncCell<C>(std::cell::UnsafeCell<C>);
    // SAFETY: see above
    unsafe impl<C: Send> Sync for UnsyncCell<C> {}
    impl<C> UnsyncCell<C> {
        // A method (rather than accessing the field in the closure) so that the closure captures the whole cell
        fn get(&self) -> *mut C {
            self.0.get()
        }
    }

    let context = UnsyncCell(std::cell::UnsafeCell::new(context));
    taffy_03::node::MeasureFunc::Boxed(Box::new(move |known_dimensions, available_space| {
        // SAFETY: the measure function is never called concurrently or re-entrantly (see above), so this is the
        // only reference to the context
        measure(unsafe { &mut *context.get() }, known_dimensions, available_space)
    }))
}

pub fn available_space_to_03(
    input: taffy::Size<taffy::AvailableSpace>,
) -> taffy_03::geometry::Size<taffy_03::style::AvailableSpace> {
    let map = |space| match space {
        taffy::AvailableSpace::Definite(value) => taffy_03::style::AvailableSpace::Definite(value),
        taffy::AvailableSpace::MinContent => taffy_03::style::AvailableSpace::MinContent,
        taffy::AvailableSpace::MaxContent => taffy_03::style::AvailableSpace::MaxContent,
    };
    taffy_03::geometry::Size { width: map(input.width), height: map(input.height) }
}

pub fn available_space_from_03(
    input: taffy_03::geometry::Size<taffy_03::style::AvailableSpace>,
) -> taffy::Size<taffy::AvailableSpace> {
    let map = |space| match space {
        taffy_03::style::AvailableSpace::Definite(value) => taffy::AvailableSpace::Definite(value),
        taffy_03::style::AvailableSpace::MinContent => taffy::AvailableSpace::MinContent,
        taffy_03::style::AvailableSpace::MaxContent => taffy::AvailableSpace::MaxContent,
    };
    taffy::Size { width: map(input.width), height: map(input.height) }
}

/// Converts a style, following CSS semantics for what Taffy 0.3 doesn't support: properties that it doesn't have
/// are ignored, and a value that it can't represent leaves the property at its default (as a browser drops a
/// declaration that it can't parse). The input is destructured exhaustively, so a property that is added to Taffy
/// is a compile error here rather than being dropped unnoticed.
fn convert_style(style: taffy::style::Style) -> taffy_03::style::Style {
    let taffy::style::Style {
        dummy: _,
        display,
        // Properties that Taffy 0.3 doesn't have
        item_is_table: _,
        item_is_replaced: _,
        item_is_compressible_replaced: _,
        box_sizing: _,
        direction: _,
        overflow: _,
        scrollbar_width: _,
        contain: _,
        float: _,
        clear: _,
        text_align: _,
        flex_line_count: _,
        grid_template_areas: _,
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
        flex_direction,
        flex_wrap,
        flex_basis,
        flex_grow,
        flex_shrink,
        grid_template_rows,
        grid_template_columns,
        grid_auto_rows,
        grid_auto_columns,
        grid_auto_flow,
        grid_template_column_names,
        grid_template_row_names,
        grid_row,
        grid_column,
    } = style;

    let default = taffy_03::style::Style::DEFAULT;
    taffy_03::style::Style {
        display: convert_display(display).unwrap_or(default.display),
        position: convert_position(position).unwrap_or(default.position),
        inset: convert_rect(inset, convert_length_percentage_auto, default.inset),
        margin: convert_rect(margin, convert_length_percentage_auto, default.margin),
        padding: convert_rect(padding, convert_length_percentage, default.padding),
        border: convert_rect(border, convert_length_percentage, default.border),
        size: convert_size(size, convert_dimension, default.size),
        min_size: convert_size(min_size, convert_length_percentage_auto_to_dimension, default.min_size),
        max_size: convert_size(max_size, convert_length_percentage_auto_to_dimension, default.max_size),
        aspect_ratio,
        gap: convert_size(gap, convert_length_percentage, default.gap),
        // Alignment
        align_items: convert_align_items(align_items),
        align_self: align_self.and_then(convert_align_items),
        justify_items: convert_align_items(justify_items),
        justify_self: justify_self.and_then(convert_align_items),
        align_content: convert_align_content(align_content),
        justify_content: convert_align_content(justify_content),
        // Flexbox
        flex_direction: convert_flex_direction(flex_direction),
        flex_wrap: convert_flex_wrap(flex_wrap).unwrap_or(default.flex_wrap),
        flex_grow,
        flex_shrink,
        flex_basis: convert_dimension(flex_basis).unwrap_or(default.flex_basis),
        // Grid
        grid_template_rows: convert_grid_template(grid_template_rows, &grid_template_row_names)
            .unwrap_or(default.grid_template_rows),
        grid_template_columns: convert_grid_template(grid_template_columns, &grid_template_column_names)
            .unwrap_or(default.grid_template_columns),
        grid_auto_rows: convert_grid_auto_tracks(grid_auto_rows).unwrap_or(default.grid_auto_rows),
        grid_auto_columns: convert_grid_auto_tracks(grid_auto_columns).unwrap_or(default.grid_auto_columns),
        grid_auto_flow: convert_grid_auto_flow(grid_auto_flow),
        grid_row: convert_line(grid_row, convert_grid_placement, default.grid_row),
        grid_column: convert_line(grid_column, convert_grid_placement, default.grid_column),
    }
}

fn convert_line<T, U>(
    input: taffy::geometry::Line<T>,
    map: impl Fn(T) -> Option<U>,
    default: taffy_03::geometry::Line<U>,
) -> taffy_03::geometry::Line<U> {
    taffy_03::geometry::Line {
        start: map(input.start).unwrap_or(default.start),
        end: map(input.end).unwrap_or(default.end),
    }
}

fn convert_min_track_sizing_function(
    input: taffy::style::MinTrackSizingFunction,
) -> Option<taffy_03::style::MinTrackSizingFunction> {
    use taffy::style::ExpandedMinTrackSizingFunction as E;
    use taffy_03::style::{LengthPercentage, MinTrackSizingFunction};
    match input.expand() {
        E::Length(value) => Some(MinTrackSizingFunction::Fixed(LengthPercentage::Points(value))),
        E::Percent(value) => Some(MinTrackSizingFunction::Fixed(LengthPercentage::Percent(value))),
        E::Auto => Some(MinTrackSizingFunction::Auto),
        E::MinContent => Some(MinTrackSizingFunction::MinContent),
        E::MaxContent => Some(MinTrackSizingFunction::MaxContent),
        E::Calc(_) => None,
    }
}

fn convert_max_track_sizing_function(
    input: taffy::style::MaxTrackSizingFunction,
) -> Option<taffy_03::style::MaxTrackSizingFunction> {
    use taffy::style::ExpandedMaxTrackSizingFunction as E;
    use taffy_03::style::{LengthPercentage, MaxTrackSizingFunction};
    match input.expand() {
        E::Length(value) => Some(MaxTrackSizingFunction::Fixed(LengthPercentage::Points(value))),
        E::Percent(value) => Some(MaxTrackSizingFunction::Fixed(LengthPercentage::Percent(value))),
        E::Auto => Some(MaxTrackSizingFunction::Auto),
        E::MinContent => Some(MaxTrackSizingFunction::MinContent),
        E::MaxContent => Some(MaxTrackSizingFunction::MaxContent),
        E::FitContentPx(limit) => Some(MaxTrackSizingFunction::FitContent(LengthPercentage::Points(limit))),
        E::FitContentPercent(limit) => Some(MaxTrackSizingFunction::FitContent(LengthPercentage::Percent(limit))),
        E::Fr(fraction) => Some(MaxTrackSizingFunction::Fraction(fraction)),
        E::FitContentCalc(_) | E::Calc(_) => None,
    }
}

fn convert_track_sizing_function(
    input: taffy::style::TrackSizingFunction,
) -> Option<taffy_03::style::NonRepeatedTrackSizingFunction> {
    Some(taffy_03::geometry::MinMax {
        min: convert_min_track_sizing_function(input.min)?,
        max: convert_max_track_sizing_function(input.max)?,
    })
}

fn convert_grid_auto_tracks(
    input: Vec<taffy::style::TrackSizingFunction>,
) -> Option<Vec<taffy_03::style::NonRepeatedTrackSizingFunction>> {
    input.into_iter().map(convert_track_sizing_function).collect()
}

/// Taffy 0.3 doesn't support named grid lines, so a template that names any lines is not converted
fn convert_grid_template(
    tracks: Vec<taffy::style::GridTemplateComponent<String>>,
    line_names: &[Vec<String>],
) -> Option<Vec<taffy_03::style::TrackSizingFunction>> {
    if line_names.iter().any(|names| !names.is_empty()) {
        return None;
    }
    tracks.into_iter().map(convert_grid_template_component).collect()
}

fn convert_grid_template_component(
    input: taffy::style::GridTemplateComponent<String>,
) -> Option<taffy_03::style::TrackSizingFunction> {
    match input {
        taffy::style::GridTemplateComponent::Single(track) => {
            Some(taffy_03::style::TrackSizingFunction::Single(convert_track_sizing_function(track)?))
        }
        taffy::style::GridTemplateComponent::Repeat(repetition) => {
            if repetition.line_names.iter().any(|names| !names.is_empty()) {
                return None;
            }
            let count = match repetition.count {
                taffy::style::RepetitionCount::AutoFill => taffy_03::style::GridTrackRepetition::AutoFill,
                taffy::style::RepetitionCount::AutoFit => taffy_03::style::GridTrackRepetition::AutoFit,
                taffy::style::RepetitionCount::Count(count) => taffy_03::style::GridTrackRepetition::Count(count),
            };
            let tracks = repetition.tracks.into_iter().map(convert_track_sizing_function).collect::<Option<_>>()?;
            Some(taffy_03::style::TrackSizingFunction::Repeat(count, tracks))
        }
    }
}

fn convert_grid_auto_flow(input: taffy::style::GridAutoFlow) -> taffy_03::style::GridAutoFlow {
    match input {
        taffy::style::GridAutoFlow::Row => taffy_03::style::GridAutoFlow::Row,
        taffy::style::GridAutoFlow::Column => taffy_03::style::GridAutoFlow::Column,
        taffy::style::GridAutoFlow::RowDense => taffy_03::style::GridAutoFlow::RowDense,
        taffy::style::GridAutoFlow::ColumnDense => taffy_03::style::GridAutoFlow::ColumnDense,
    }
}

// Taffy 0.3 doesn't support named grid lines
fn convert_grid_placement(input: taffy::style::GridPlacement<String>) -> Option<taffy_03::style::GridPlacement> {
    match input {
        taffy::style::GridPlacement::Auto => Some(taffy_03::style::GridPlacement::Auto),
        taffy::style::GridPlacement::Line(line) => Some(taffy_03::style_helpers::line(line.as_i16())),
        taffy::style::GridPlacement::Span(span) => Some(taffy_03::style::GridPlacement::Span(span)),
        taffy::style::GridPlacement::NamedLine(..) | taffy::style::GridPlacement::NamedSpan(..) => None,
    }
}

// Taffy 0.3 represents the default alignment with `None` (it has no `normal` keyword), which is also what values
// that it doesn't support (`self-start`, `self-end` and `safe` alignment) fall back to
fn convert_align_items(input: taffy::style::AlignItems) -> Option<taffy_03::style::AlignItems> {
    use taffy::style::AlignItemsKeyword as K;
    use taffy_03::style::AlignItems as A;
    if input.safety == taffy::style::AlignmentSafety::Safe {
        return None;
    }
    Some(match input.keyword {
        K::Normal | K::SelfStart | K::SelfEnd => return None,
        K::Start => A::Start,
        K::End => A::End,
        K::FlexStart => A::FlexStart,
        K::FlexEnd => A::FlexEnd,
        K::Center => A::Center,
        K::Baseline => A::Baseline,
        K::Stretch => A::Stretch,
    })
}

fn convert_align_content(input: taffy::style::AlignContent) -> Option<taffy_03::style::AlignContent> {
    use taffy::style::AlignContentKeyword as K;
    use taffy_03::style::AlignContent as A;
    if input.safety == taffy::style::AlignmentSafety::Safe {
        return None;
    }
    Some(match input.keyword {
        K::Normal => return None,
        K::Start => A::Start,
        K::End => A::End,
        K::FlexStart => A::FlexStart,
        K::FlexEnd => A::FlexEnd,
        K::Center => A::Center,
        K::Stretch => A::Stretch,
        K::SpaceBetween => A::SpaceBetween,
        K::SpaceEvenly => A::SpaceEvenly,
        K::SpaceAround => A::SpaceAround,
    })
}

fn convert_rect<T, U>(
    input: taffy::geometry::Rect<T>,
    map: impl Fn(T) -> Option<U>,
    default: taffy_03::geometry::Rect<U>,
) -> taffy_03::geometry::Rect<U> {
    taffy_03::geometry::Rect {
        left: map(input.left).unwrap_or(default.left),
        right: map(input.right).unwrap_or(default.right),
        top: map(input.top).unwrap_or(default.top),
        bottom: map(input.bottom).unwrap_or(default.bottom),
    }
}

fn convert_size<T, U>(
    input: taffy::geometry::Size<T>,
    map: impl Fn(T) -> Option<U>,
    default: taffy_03::geometry::Size<U>,
) -> taffy_03::geometry::Size<U> {
    taffy_03::geometry::Size {
        width: map(input.width).unwrap_or(default.width),
        height: map(input.height).unwrap_or(default.height),
    }
}

fn convert_dimension(input: taffy::style::Dimension) -> Option<taffy_03::style::Dimension> {
    use taffy::style::ExpandedDimension as E;
    match input.expand() {
        E::Length(value) => Some(taffy_03::style::Dimension::Points(value)),
        E::Percent(value) => Some(taffy_03::style::Dimension::Percent(value)),
        E::Auto => Some(taffy_03::style::Dimension::Auto),
        E::MinContent
        | E::MaxContent
        | E::FitContent
        | E::FitContentPx(_)
        | E::FitContentPercent(_)
        | E::FitContentCalc(_)
        | E::Stretch
        | E::Content
        | E::Calc(_) => None,
    }
}

fn convert_length_percentage_auto_to_dimension(
    input: taffy::style::LengthPercentageAuto,
) -> Option<taffy_03::style::Dimension> {
    use taffy::style::ExpandedLengthPercentageAuto as E;
    match input.expand() {
        E::Length(value) => Some(taffy_03::style::Dimension::Points(value)),
        E::Percent(value) => Some(taffy_03::style::Dimension::Percent(value)),
        E::Auto => Some(taffy_03::style::Dimension::Auto),
        E::Calc(_) => None,
    }
}

fn convert_length_percentage_auto(
    input: taffy::style::LengthPercentageAuto,
) -> Option<taffy_03::style::LengthPercentageAuto> {
    use taffy::style::ExpandedLengthPercentageAuto as E;
    match input.expand() {
        E::Length(value) => Some(taffy_03::style::LengthPercentageAuto::Points(value)),
        E::Percent(value) => Some(taffy_03::style::LengthPercentageAuto::Percent(value)),
        E::Auto => Some(taffy_03::style::LengthPercentageAuto::Auto),
        E::Calc(_) => None,
    }
}

fn convert_length_percentage(input: taffy::style::LengthPercentage) -> Option<taffy_03::style::LengthPercentage> {
    use taffy::style::ExpandedLengthPercentage as E;
    match input.expand() {
        E::Length(value) => Some(taffy_03::style::LengthPercentage::Points(value)),
        E::Percent(value) => Some(taffy_03::style::LengthPercentage::Percent(value)),
        E::Calc(_) => None,
    }
}

// Taffy 0.3 has no block layout
fn convert_display(input: taffy::style::Display) -> Option<taffy_03::style::Display> {
    match input {
        taffy::style::Display::None => Some(taffy_03::style::Display::None),
        taffy::style::Display::Flex => Some(taffy_03::style::Display::Flex),
        taffy::style::Display::Grid => Some(taffy_03::style::Display::Grid),
        taffy::style::Display::Block | taffy::style::Display::FlowRoot => None,
    }
}

// Taffy 0.3 has no static, fixed or sticky positioning
fn convert_position(input: taffy::style::Position) -> Option<taffy_03::style::Position> {
    match input {
        taffy::style::Position::Relative => Some(taffy_03::style::Position::Relative),
        taffy::style::Position::Absolute => Some(taffy_03::style::Position::Absolute),
        taffy::style::Position::Static | taffy::style::Position::Fixed | taffy::style::Position::Sticky => None,
    }
}

fn convert_flex_direction(input: taffy::style::FlexDirection) -> taffy_03::style::FlexDirection {
    match input {
        taffy::style::FlexDirection::Row => taffy_03::style::FlexDirection::Row,
        taffy::style::FlexDirection::Column => taffy_03::style::FlexDirection::Column,
        taffy::style::FlexDirection::RowReverse => taffy_03::style::FlexDirection::RowReverse,
        taffy::style::FlexDirection::ColumnReverse => taffy_03::style::FlexDirection::ColumnReverse,
    }
}

// Taffy 0.3 has no `flex-wrap: balance`
fn convert_flex_wrap(input: taffy::style::FlexWrap) -> Option<taffy_03::style::FlexWrap> {
    match input {
        taffy::style::FlexWrap::NoWrap => Some(taffy_03::style::FlexWrap::NoWrap),
        taffy::style::FlexWrap::Wrap => Some(taffy_03::style::FlexWrap::Wrap),
        taffy::style::FlexWrap::WrapReverse => Some(taffy_03::style::FlexWrap::WrapReverse),
        taffy::style::FlexWrap::Balance | taffy::style::FlexWrap::BalanceReverse => None,
    }
}
