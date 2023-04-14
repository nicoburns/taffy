use taffy::prelude::*;

// Creates three 20px x 20px children, evenly spaced 10px apart from each other
// Thus the container is 80px x 20px.

fn main() -> Result<(), taffy::TaffyError> {
    let mut tree = Taffy::new();
    let root_id = tree.new_with_children(
        Style { gap: Size { width: points(10.0), height: zero() }, ..Default::default() },
        |node| {
            let child_style = Style { size: Size { width: points(20.0), height: points(20.0) }, ..default() };
            node.new_leaf(child_style.clone());
            node.new_leaf(child_style.clone());
            node.new_leaf(child_style.clone());
        },
    );

    // Compute layout and print result
    tree.compute_layout(root_id, Size::MAX_CONTENT)?;
    taffy::util::print_tree(&tree, root_id);

    Ok(())
}
