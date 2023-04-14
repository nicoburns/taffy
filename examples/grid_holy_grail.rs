// This creates a so-called "holy grail" layout using the CSS Grid layout algorithm
// See: https://en.wikipedia.org/wiki/Holy_grail_(web_design)

fn main() -> Result<(), taffy::TaffyError> {
    use taffy::prelude::*;

    let mut tree = Taffy::new();
    let root = tree.new_with_children(
        Style {
            display: Display::Grid,
            size: Size { width: points(800.0), height: points(600.0) },
            grid_template_columns: vec![points(250.0), fr(1.0), points(250.0)],
            grid_template_rows: vec![points(150.0), fr(1.0), points(150.0)],
            ..default()
        },
        |node| {
            let _header = node.new_leaf(Style { grid_row: line(1), grid_column: span(3), ..default() });
            let _left_sidebar = node.new_leaf(Style { grid_row: line(2), grid_column: line(1), ..default() });
            let _content_area = node.new_leaf(Style { grid_row: line(2), grid_column: line(2), ..default() });
            let _right_sidebar = node.new_leaf(Style { grid_row: line(2), grid_column: line(3), ..default() });
            let _footer = node.new_leaf(Style { grid_row: line(3), grid_column: span(3), ..default() });
        },
    );

    // Compute layout and print result
    tree.compute_layout(root, Size { width: points(800.0), height: points(600.0) })?;
    taffy::util::print_tree(&tree, root);

    Ok(())
}
