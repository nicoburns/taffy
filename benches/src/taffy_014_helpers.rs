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
    convert_size(input, |space| match space {
        taffy::AvailableSpace::Definite(value) => taffy_014::AvailableSpace::Definite(value),
        taffy::AvailableSpace::MinContent => taffy_014::AvailableSpace::MinContent,
        taffy::AvailableSpace::MaxContent => taffy_014::AvailableSpace::MaxContent,
    })
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

/// Converts the subset of style properties used by the benchmarks. Alignment properties and grid template areas
/// are left at their defaults.
fn convert_style(style: taffy::style::Style) -> taffy_014::style::Style {
    taffy_014::style::Style {
        display: convert_display(style.display),
        item_is_table: style.item_is_table,
        item_is_replaced: style.item_is_replaced,
        box_sizing: convert_box_sizing(style.box_sizing),
        direction: convert_direction(style.direction),
        overflow: convert_point(style.overflow, convert_overflow),
        scrollbar_width: style.scrollbar_width,
        position: convert_position(style.position),
        inset: convert_rect(style.inset, convert_length_percentage_auto),
        size: convert_size(style.size, convert_dimension),
        min_size: convert_size(style.min_size, convert_length_percentage_auto),
        max_size: convert_size(style.max_size, convert_length_percentage_auto),
        aspect_ratio: style.aspect_ratio,
        margin: convert_rect(style.margin, convert_length_percentage_auto),
        padding: convert_rect(style.padding, convert_length_percentage),
        border: convert_rect(style.border, convert_length_percentage),
        gap: convert_size(style.gap, convert_length_percentage),
        // Flexbox
        flex_direction: convert_flex_direction(style.flex_direction),
        flex_wrap: convert_flex_wrap(style.flex_wrap),
        flex_line_count: style.flex_line_count,
        flex_basis: convert_dimension(style.flex_basis),
        flex_grow: style.flex_grow,
        flex_shrink: style.flex_shrink,
        // Grid
        grid_template_rows: style.grid_template_rows.into_iter().map(convert_grid_template_component).collect(),
        grid_template_columns: style.grid_template_columns.into_iter().map(convert_grid_template_component).collect(),
        grid_auto_rows: style.grid_auto_rows.into_iter().map(convert_track_sizing_function).collect(),
        grid_auto_columns: style.grid_auto_columns.into_iter().map(convert_track_sizing_function).collect(),
        grid_auto_flow: convert_grid_auto_flow(style.grid_auto_flow),
        grid_template_row_names: style.grid_template_row_names,
        grid_template_column_names: style.grid_template_column_names,
        grid_row: convert_line(style.grid_row, convert_grid_placement),
        grid_column: convert_line(style.grid_column, convert_grid_placement),
        ..Default::default()
    }
}

fn convert_rect<T, U, F: Fn(T) -> U>(input: taffy::geometry::Rect<T>, map: F) -> taffy_014::geometry::Rect<U> {
    taffy_014::geometry::Rect {
        left: map(input.left),
        right: map(input.right),
        top: map(input.top),
        bottom: map(input.bottom),
    }
}

fn convert_size<T, U, F: Fn(T) -> U>(input: taffy::geometry::Size<T>, map: F) -> taffy_014::geometry::Size<U> {
    taffy_014::geometry::Size { width: map(input.width), height: map(input.height) }
}

fn convert_point<T, U, F: Fn(T) -> U>(input: taffy::geometry::Point<T>, map: F) -> taffy_014::geometry::Point<U> {
    taffy_014::geometry::Point { x: map(input.x), y: map(input.y) }
}

fn convert_line<T, U, F: Fn(T) -> U>(input: taffy::geometry::Line<T>, map: F) -> taffy_014::geometry::Line<U> {
    taffy_014::geometry::Line { start: map(input.start), end: map(input.end) }
}

// The `CompactLength` tag values differ between versions, so values are rebuilt from their tag rather than
// copied across in their raw form.
fn convert_dimension(input: taffy::style::Dimension) -> taffy_014::style::Dimension {
    let raw = input.into_raw();
    match raw.tag() {
        taffy::style::CompactLength::LENGTH_TAG => taffy_014::style::Dimension::length(raw.value()),
        taffy::style::CompactLength::PERCENT_TAG => taffy_014::style::Dimension::percent(raw.value()),
        taffy::style::CompactLength::AUTO_TAG => taffy_014::style::Dimension::auto(),
        _ => panic!("unsupported Dimension variant"),
    }
}

fn convert_length_percentage_auto(input: taffy::style::LengthPercentageAuto) -> taffy_014::style::LengthPercentageAuto {
    let raw = input.into_raw();
    match raw.tag() {
        taffy::style::CompactLength::LENGTH_TAG => taffy_014::style::LengthPercentageAuto::length(raw.value()),
        taffy::style::CompactLength::PERCENT_TAG => taffy_014::style::LengthPercentageAuto::percent(raw.value()),
        taffy::style::CompactLength::AUTO_TAG => taffy_014::style::LengthPercentageAuto::auto(),
        _ => panic!("unsupported LengthPercentageAuto variant"),
    }
}

fn convert_length_percentage(input: taffy::style::LengthPercentage) -> taffy_014::style::LengthPercentage {
    let raw = input.into_raw();
    match raw.tag() {
        taffy::style::CompactLength::LENGTH_TAG => taffy_014::style::LengthPercentage::length(raw.value()),
        taffy::style::CompactLength::PERCENT_TAG => taffy_014::style::LengthPercentage::percent(raw.value()),
        _ => panic!("unsupported LengthPercentage variant"),
    }
}

fn convert_min_track_sizing_function(
    input: taffy::style::MinTrackSizingFunction,
) -> taffy_014::style::MinTrackSizingFunction {
    let raw = input.into_raw();
    match raw.tag() {
        taffy::style::CompactLength::LENGTH_TAG => taffy_014::style::MinTrackSizingFunction::length(raw.value()),
        taffy::style::CompactLength::PERCENT_TAG => taffy_014::style::MinTrackSizingFunction::percent(raw.value()),
        taffy::style::CompactLength::AUTO_TAG => taffy_014::style::MinTrackSizingFunction::auto(),
        taffy::style::CompactLength::MIN_CONTENT_TAG => taffy_014::style::MinTrackSizingFunction::min_content(),
        taffy::style::CompactLength::MAX_CONTENT_TAG => taffy_014::style::MinTrackSizingFunction::max_content(),
        _ => panic!("unsupported MinTrackSizingFunction variant"),
    }
}

fn convert_max_track_sizing_function(
    input: taffy::style::MaxTrackSizingFunction,
) -> taffy_014::style::MaxTrackSizingFunction {
    let raw = input.into_raw();
    match raw.tag() {
        taffy::style::CompactLength::LENGTH_TAG => taffy_014::style::MaxTrackSizingFunction::length(raw.value()),
        taffy::style::CompactLength::PERCENT_TAG => taffy_014::style::MaxTrackSizingFunction::percent(raw.value()),
        taffy::style::CompactLength::AUTO_TAG => taffy_014::style::MaxTrackSizingFunction::auto(),
        taffy::style::CompactLength::MIN_CONTENT_TAG => taffy_014::style::MaxTrackSizingFunction::min_content(),
        taffy::style::CompactLength::MAX_CONTENT_TAG => taffy_014::style::MaxTrackSizingFunction::max_content(),
        taffy::style::CompactLength::FIT_CONTENT_PX_TAG => {
            taffy_014::style::MaxTrackSizingFunction::fit_content_px(raw.value())
        }
        taffy::style::CompactLength::FIT_CONTENT_PERCENT_TAG => {
            taffy_014::style::MaxTrackSizingFunction::fit_content_percent(raw.value())
        }
        taffy::style::CompactLength::FR_TAG => taffy_014::style::MaxTrackSizingFunction::fr(raw.value()),
        _ => panic!("unsupported MaxTrackSizingFunction variant"),
    }
}

fn convert_track_sizing_function(input: taffy::style::TrackSizingFunction) -> taffy_014::style::TrackSizingFunction {
    taffy_014::geometry::MinMax {
        min: convert_min_track_sizing_function(input.min),
        max: convert_max_track_sizing_function(input.max),
    }
}

fn convert_grid_template_component(
    input: taffy::style::GridTemplateComponent<String>,
) -> taffy_014::style::GridTemplateComponent<String> {
    match input {
        taffy::style::GridTemplateComponent::Single(track) => {
            taffy_014::style::GridTemplateComponent::Single(convert_track_sizing_function(track))
        }
        taffy::style::GridTemplateComponent::Repeat(repetition) => {
            taffy_014::style::GridTemplateComponent::Repeat(taffy_014::style::GridTemplateRepetition {
                count: match repetition.count {
                    taffy::style::RepetitionCount::AutoFill => taffy_014::style::RepetitionCount::AutoFill,
                    taffy::style::RepetitionCount::AutoFit => taffy_014::style::RepetitionCount::AutoFit,
                    taffy::style::RepetitionCount::Count(count) => taffy_014::style::RepetitionCount::Count(count),
                },
                tracks: repetition.tracks.into_iter().map(convert_track_sizing_function).collect(),
                line_names: repetition.line_names,
            })
        }
    }
}

fn convert_grid_auto_flow(input: taffy::style::GridAutoFlow) -> taffy_014::style::GridAutoFlow {
    match input {
        taffy::style::GridAutoFlow::Row => taffy_014::style::GridAutoFlow::Row,
        taffy::style::GridAutoFlow::Column => taffy_014::style::GridAutoFlow::Column,
        taffy::style::GridAutoFlow::RowDense => taffy_014::style::GridAutoFlow::RowDense,
        taffy::style::GridAutoFlow::ColumnDense => taffy_014::style::GridAutoFlow::ColumnDense,
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

fn convert_display(input: taffy::style::Display) -> taffy_014::style::Display {
    match input {
        taffy::style::Display::None => taffy_014::style::Display::None,
        taffy::style::Display::Flex => taffy_014::style::Display::Flex,
        taffy::style::Display::Grid => taffy_014::style::Display::Grid,
        taffy::style::Display::Block => taffy_014::style::Display::Block,
        taffy::style::Display::FlowRoot => taffy_014::style::Display::FlowRoot,
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

// Taffy 0.14 has no `Static`, `Fixed` or `Sticky` positioning, so these map to their closest equivalent
fn convert_position(input: taffy::style::Position) -> taffy_014::style::Position {
    match input {
        taffy::style::Position::Static => taffy_014::style::Position::Relative,
        taffy::style::Position::Relative => taffy_014::style::Position::Relative,
        taffy::style::Position::Absolute => taffy_014::style::Position::Absolute,
        taffy::style::Position::Fixed => taffy_014::style::Position::Absolute,
        taffy::style::Position::Sticky => taffy_014::style::Position::Relative,
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
