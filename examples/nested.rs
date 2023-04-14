use taffy::prelude::*;

fn main() -> Result<(), taffy::TaffyError> {
    let mut tree = Taffy::new();

    let root_id = tree.new_with_children(
        Style { size: Size { width: percent(1.0), height: percent(1.0) }, ..default() },
        |node| {
            let left = node.new_with_children(
                Style { size: Size { width: percent(0.5), height: percent(1.0) }, ..default() },
                |node| [node.new_leaf(Style { size: Size { width: points(5.0), height: points(5.0) }, ..default() })],
            );
            let right = node.new_with_children(
                Style { size: Size { width: percent(0.5), height: percent(1.0) }, ..default() },
                |node| [node.new_leaf(Style { size: Size { width: points(5.0), height: points(5.0) }, ..default() })],
            );
            [left, right]
        },
    );

    tree.compute_layout(root_id, Size { height: points(100.0), width: points(100.0) })?;
    taffy::util::print_tree(&tree, root_id);
    println!("node: {:#?}", tree.layout(root_id)?);

    Ok(())
}
