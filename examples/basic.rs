use taffy::prelude::*;

fn main() -> Result<(), taffy::TaffyError> {
    let mut tree = Taffy::new();
    let root_id = tree.new_with_children(
        Style {
            size: Size { width: points(100.0), height: points(100.0) },
            justify_content: Some(JustifyContent::Center),
            ..default()
        },
        |node| [node.new_leaf(Style { size: Size { width: percent(0.5), height: auto() }, ..default() })],
    );

    tree.compute_layout(root_id, Size::MAX_CONTENT)?;
    taffy::util::print_tree(&tree, root_id);

    // let root_node_id = taffy.container(
    //     Style::column().width(800).height(600),
    //     |cx| {
    //         cx.new_leaf(Style::leaf().height(600));
    //         cx.new_leaf(Style::leaf().flex_grow(1.0));
    //     },
    // );

    // or just use undefined for 100 x 100
    // taffy.compute_layout(node, Size::NONE)?;

    // println!("node: {:#?}", taffy.layout(node)?);
    // println!("child: {:#?}", taffy.layout(child)?);

    Ok(())
}
