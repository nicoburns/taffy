#[test]
fn leaf_overflow_scrollbars_affect_available_space_x_axis() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, Taffy};
    let mut taffy: Taffy<crate::TextMeasure> = Taffy::new();
    let node = taffy
        .new_leaf_with_context(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Scroll,
                    y: taffy::style::Overflow::Visible,
                },
                scrollbar_width: 15f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(45f32),
                    height: taffy::style::Dimension::Length(45f32),
                },
                ..Default::default()
            },
            crate::TextMeasure {
                text_content: "H\u{a0}H\u{a0}H\u{a0}H\u{a0}H\u{a0}H\u{a0}H\u{a0}H\u{a0}H\u{a0}H\u{a0}H",
                writing_mode: crate::WritingMode::Horizontal,
                _aspect_ratio: None,
            },
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 45f32, "width of node {:?}. Expected {}. Actual {}", node, 45f32, size.width);
    assert_eq!(size.height, 45f32, "height of node {:?}. Expected {}. Actual {}", node, 45f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
}
