#[test]
fn grid_auto_flex_row_child() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 0f32,
            flex_shrink: 0f32,
            flex_basis: taffy::style::Dimension::Auto,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(60f32),
                height: taffy::style::Dimension::Points(40f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            flex_shrink: 1f32,
            flex_basis: taffy::style::Dimension::Points(0f32),
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style { display: taffy::style::Display::Flex, ..Default::default() },
            &[node00, node01],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![auto(), auto()],
                grid_template_columns: vec![auto(), auto()],
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100f32), height: auto() },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    assert_eq!(taffy.layout(node).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 80f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().size.width, 60f32);
    assert_eq!(taffy.layout(node00).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node01).unwrap().size.width, 20f32);
    assert_eq!(taffy.layout(node01).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node01).unwrap().location.x, 60f32);
    assert_eq!(taffy.layout(node01).unwrap().location.y, 0f32);
}
