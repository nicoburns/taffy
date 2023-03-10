#[test]
fn bevy_issue_7994_5_level() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node0000 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node000 = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(1f32),
                    height: taffy::style::Dimension::Percent(1f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(5f32),
                    right: taffy::style::LengthPercentage::Points(5f32),
                    top: taffy::style::LengthPercentage::Points(5f32),
                    bottom: taffy::style::LengthPercentage::Points(5f32),
                },
                ..Default::default()
            },
            &[node0000],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(1f32),
                    height: taffy::style::Dimension::Percent(1f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(5f32),
                    right: taffy::style::LengthPercentage::Points(5f32),
                    top: taffy::style::LengthPercentage::Points(5f32),
                    bottom: taffy::style::LengthPercentage::Points(5f32),
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(40f32),
                    height: taffy::style::Dimension::Points(40f32),
                },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Points(5f32),
                    right: taffy::style::LengthPercentageAuto::Points(5f32),
                    top: taffy::style::LengthPercentageAuto::Points(5f32),
                    bottom: taffy::style::LengthPercentageAuto::Points(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(5f32),
                    right: taffy::style::LengthPercentage::Points(5f32),
                    top: taffy::style::LengthPercentage::Points(5f32),
                    bottom: taffy::style::LengthPercentage::Points(5f32),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node100 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1f32),
                height: taffy::style::Dimension::Percent(1f32),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Points(5f32),
                right: taffy::style::LengthPercentage::Points(5f32),
                top: taffy::style::LengthPercentage::Points(5f32),
                bottom: taffy::style::LengthPercentage::Points(5f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(1f32),
                    height: taffy::style::Dimension::Percent(1f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(5f32),
                    right: taffy::style::LengthPercentage::Points(5f32),
                    top: taffy::style::LengthPercentage::Points(5f32),
                    bottom: taffy::style::LengthPercentage::Points(5f32),
                },
                ..Default::default()
            },
            &[node100],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(40f32),
                    height: taffy::style::Dimension::Points(40f32),
                },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Points(5f32),
                    right: taffy::style::LengthPercentageAuto::Points(5f32),
                    top: taffy::style::LengthPercentageAuto::Points(5f32),
                    bottom: taffy::style::LengthPercentageAuto::Points(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(5f32),
                    right: taffy::style::LengthPercentage::Points(5f32),
                    top: taffy::style::LengthPercentage::Points(5f32),
                    bottom: taffy::style::LengthPercentage::Points(5f32),
                },
                ..Default::default()
            },
            &[node10],
        )
        .unwrap();
    let node2000 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node200 = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(1f32), height: auto() },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(5f32),
                    right: taffy::style::LengthPercentage::Points(5f32),
                    top: taffy::style::LengthPercentage::Points(5f32),
                    bottom: taffy::style::LengthPercentage::Points(5f32),
                },
                ..Default::default()
            },
            &[node2000],
        )
        .unwrap();
    let node20 = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(1f32),
                    height: taffy::style::Dimension::Percent(1f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(5f32),
                    right: taffy::style::LengthPercentage::Points(5f32),
                    top: taffy::style::LengthPercentage::Points(5f32),
                    bottom: taffy::style::LengthPercentage::Points(5f32),
                },
                ..Default::default()
            },
            &[node200],
        )
        .unwrap();
    let node2 = taffy
        .new_with_children(
            taffy::style::Style {
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(40f32),
                    height: taffy::style::Dimension::Points(40f32),
                },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Points(5f32),
                    right: taffy::style::LengthPercentageAuto::Points(5f32),
                    top: taffy::style::LengthPercentageAuto::Points(5f32),
                    bottom: taffy::style::LengthPercentageAuto::Points(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(5f32),
                    right: taffy::style::LengthPercentage::Points(5f32),
                    top: taffy::style::LengthPercentage::Points(5f32),
                    bottom: taffy::style::LengthPercentage::Points(5f32),
                },
                ..Default::default()
            },
            &[node20],
        )
        .unwrap();
    let node300 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(1f32), height: auto() },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Points(5f32),
                right: taffy::style::LengthPercentage::Points(5f32),
                top: taffy::style::LengthPercentage::Points(5f32),
                bottom: taffy::style::LengthPercentage::Points(5f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node30 = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(1f32),
                    height: taffy::style::Dimension::Percent(1f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(5f32),
                    right: taffy::style::LengthPercentage::Points(5f32),
                    top: taffy::style::LengthPercentage::Points(5f32),
                    bottom: taffy::style::LengthPercentage::Points(5f32),
                },
                ..Default::default()
            },
            &[node300],
        )
        .unwrap();
    let node3 = taffy
        .new_with_children(
            taffy::style::Style {
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(40f32),
                    height: taffy::style::Dimension::Points(40f32),
                },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Points(5f32),
                    right: taffy::style::LengthPercentageAuto::Points(5f32),
                    top: taffy::style::LengthPercentageAuto::Points(5f32),
                    bottom: taffy::style::LengthPercentageAuto::Points(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(5f32),
                    right: taffy::style::LengthPercentage::Points(5f32),
                    top: taffy::style::LengthPercentage::Points(5f32),
                    bottom: taffy::style::LengthPercentage::Points(5f32),
                },
                ..Default::default()
            },
            &[node30],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                align_items: Some(taffy::style::AlignItems::FlexStart),
                justify_content: Some(taffy::style::JustifyContent::FlexStart),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(200f32),
                    height: taffy::style::Dimension::Points(200f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 40f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node00.data(), 30f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node00.data(), 20f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node00.data(), 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node00.data(), 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node000).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node000.data(), 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node000.data(), 10f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node000.data(), 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node000.data(), 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0000).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node0000.data(), 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node0000.data(), 0f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node0000.data(), 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node0000.data(), 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1.data(), 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node1.data(), 40f32, size.height);
    assert_eq!(location.x, 55f32, "x of node {:?}. Expected {}. Actual {}", node1.data(), 55f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node1.data(), 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node10.data(), 30f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node10.data(), 20f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node10.data(), 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node10.data(), 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node100).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node100.data(), 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node100.data(), 10f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node100.data(), 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node100.data(), 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node2.data(), 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node2.data(), 40f32, size.height);
    assert_eq!(location.x, 105f32, "x of node {:?}. Expected {}. Actual {}", node2.data(), 105f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node2.data(), 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node20).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node20.data(), 30f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node20.data(), 20f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node20.data(), 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node20.data(), 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node200).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node200.data(), 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node200.data(), 10f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node200.data(), 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node200.data(), 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2000).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node2000.data(), 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node2000.data(), 0f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node2000.data(), 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node2000.data(), 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3.data(), 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node3.data(), 40f32, size.height);
    assert_eq!(location.x, 155f32, "x of node {:?}. Expected {}. Actual {}", node3.data(), 155f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node3.data(), 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node30).unwrap();
    assert_eq!(size.width, 30f32, "width of node {:?}. Expected {}. Actual {}", node30.data(), 30f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node30.data(), 20f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node30.data(), 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node30.data(), 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node300).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node300.data(), 20f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node300.data(), 10f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node300.data(), 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node300.data(), 5f32, location.y);
}
