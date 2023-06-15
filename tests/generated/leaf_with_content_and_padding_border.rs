#[test]
fn leaf_with_content_and_padding_border() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node = taffy
        .new_leaf_with_measure(
            taffy::style::Style {
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(8f32),
                    right: taffy::style::LengthPercentage::Points(4f32),
                    top: taffy::style::LengthPercentage::Points(2f32),
                    bottom: taffy::style::LengthPercentage::Points(6f32),
                },
                border: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(7f32),
                    right: taffy::style::LengthPercentage::Points(3f32),
                    top: taffy::style::LengthPercentage::Points(1f32),
                    bottom: taffy::style::LengthPercentage::Points(5f32),
                },
                ..Default::default()
            },
            taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &str = "HHHH";
                super::measure_standard_text(
                    known_dimensions,
                    available_space,
                    TEXT,
                    super::WritingMode::Horizontal,
                    None,
                )
            }),
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 62f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 62f32, size.width);
    assert_eq!(size.height, 24f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 24f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
}
