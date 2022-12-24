#[test]
fn grid_percent_container_definite_available_space() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node01 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node02 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node03 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![flex(1f32), flex(1f32)],
                grid_template_columns: vec![flex(1f32), flex(1f32)],
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(0.6f32),
                    height: taffy::style::Dimension::Percent(0.4f32),
                },
                ..Default::default()
            },
            &[node00, node01, node02, node03],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![points(100f32)],
                grid_template_columns: vec![points(100f32)],
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
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 60f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node00).unwrap().size.height, 20f32);
    assert_eq!(taffy.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node01).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node01).unwrap().size.height, 20f32);
    assert_eq!(taffy.layout(node01).unwrap().location.x, 30f32);
    assert_eq!(taffy.layout(node01).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node02).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node02).unwrap().size.height, 20f32);
    assert_eq!(taffy.layout(node02).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node02).unwrap().location.y, 20f32);
    assert_eq!(taffy.layout(node03).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node03).unwrap().size.height, 20f32);
    assert_eq!(taffy.layout(node03).unwrap().location.x, 30f32);
    assert_eq!(taffy.layout(node03).unwrap().location.y, 20f32);
}
