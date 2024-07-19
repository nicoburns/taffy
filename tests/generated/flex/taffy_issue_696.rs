#[test]
#[allow(non_snake_case)]
fn taffy_issue_696__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Flex,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(544f32),
                height: taffy::style::Dimension::Length(251f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node100 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Flex,
            flex_shrink: 0f32,
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(394f32) },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                flex_direction: taffy::style::FlexDirection::Column,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Hidden,
                    y: taffy::style::Overflow::Hidden,
                },
                scrollbar_width: 15f32,
                flex_grow: 1f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::Length(0f32),
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(312f32),
                    height: taffy::style::Dimension::Length(251f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(32f32),
                    right: taffy::style::LengthPercentage::Length(32f32),
                    top: taffy::style::LengthPercentage::Length(32f32),
                    bottom: taffy::style::LengthPercentage::Length(32f32),
                },
                ..Default::default()
            },
            &[node100],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Length(312f32), height: auto() },
                min_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(251f32) },
                ..Default::default()
            },
            &[node10],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                position: taffy::style::Position::Absolute,
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(20f32),
                    right: taffy::style::LengthPercentage::Length(20f32),
                    top: taffy::style::LengthPercentage::Length(20f32),
                    bottom: taffy::style::LengthPercentage::Length(20f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 896f32, "width of node {:?}. Expected {}. Actual {}", node, 896f32, size.width);
    assert_eq!(size.height, 291f32, "height of node {:?}. Expected {}. Actual {}", node, 291f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 544f32, "width of node {:?}. Expected {}. Actual {}", node0, 544f32, size.width);
    assert_eq!(size.height, 251f32, "height of node {:?}. Expected {}. Actual {}", node0, 251f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node0, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node0, 20f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node0,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node0,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 312f32, "width of node {:?}. Expected {}. Actual {}", node1, 312f32, size.width);
    assert_eq!(size.height, 251f32, "height of node {:?}. Expected {}. Actual {}", node1, 251f32, size.height);
    assert_eq!(location.x, 564f32, "x of node {:?}. Expected {}. Actual {}", node1, 564f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node1, 20f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node10).unwrap();
    assert_eq!(size.width, 312f32, "width of node {:?}. Expected {}. Actual {}", node10, 312f32, size.width);
    assert_eq!(size.height, 251f32, "height of node {:?}. Expected {}. Actual {}", node10, 251f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node10,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        207f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node10,
        207f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node100).unwrap();
    assert_eq!(size.width, 248f32, "width of node {:?}. Expected {}. Actual {}", node100, 248f32, size.width);
    assert_eq!(size.height, 394f32, "height of node {:?}. Expected {}. Actual {}", node100, 394f32, size.height);
    assert_eq!(location.x, 32f32, "x of node {:?}. Expected {}. Actual {}", node100, 32f32, location.x);
    assert_eq!(location.y, 32f32, "y of node {:?}. Expected {}. Actual {}", node100, 32f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node100,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node100,
        0f32,
        layout.scroll_height()
    );
}

#[test]
#[allow(non_snake_case)]
fn taffy_issue_696__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Flex,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(544f32),
                height: taffy::style::Dimension::Length(251f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node100 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Flex,
            flex_shrink: 0f32,
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(394f32) },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                flex_direction: taffy::style::FlexDirection::Column,
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Hidden,
                    y: taffy::style::Overflow::Hidden,
                },
                scrollbar_width: 15f32,
                flex_grow: 1f32,
                flex_shrink: 0f32,
                flex_basis: taffy::style::Dimension::Length(0f32),
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(312f32),
                    height: taffy::style::Dimension::Length(251f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(32f32),
                    right: taffy::style::LengthPercentage::Length(32f32),
                    top: taffy::style::LengthPercentage::Length(32f32),
                    bottom: taffy::style::LengthPercentage::Length(32f32),
                },
                ..Default::default()
            },
            &[node100],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Length(312f32), height: auto() },
                min_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(251f32) },
                ..Default::default()
            },
            &[node10],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                position: taffy::style::Position::Absolute,
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(20f32),
                    right: taffy::style::LengthPercentage::Length(20f32),
                    top: taffy::style::LengthPercentage::Length(20f32),
                    bottom: taffy::style::LengthPercentage::Length(20f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 896f32, "width of node {:?}. Expected {}. Actual {}", node, 896f32, size.width);
    assert_eq!(size.height, 291f32, "height of node {:?}. Expected {}. Actual {}", node, 291f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 544f32, "width of node {:?}. Expected {}. Actual {}", node0, 544f32, size.width);
    assert_eq!(size.height, 251f32, "height of node {:?}. Expected {}. Actual {}", node0, 251f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node0, 20f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node0, 20f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node0,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node0,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 312f32, "width of node {:?}. Expected {}. Actual {}", node1, 312f32, size.width);
    assert_eq!(size.height, 251f32, "height of node {:?}. Expected {}. Actual {}", node1, 251f32, size.height);
    assert_eq!(location.x, 564f32, "x of node {:?}. Expected {}. Actual {}", node1, 564f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node1, 20f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node1,
        0f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node10).unwrap();
    assert_eq!(size.width, 312f32, "width of node {:?}. Expected {}. Actual {}", node10, 312f32, size.width);
    assert_eq!(size.height, 251f32, "height of node {:?}. Expected {}. Actual {}", node10, 251f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node10,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        207f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node10,
        207f32,
        layout.scroll_height()
    );
    #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
    let layout @ Layout { size, location, .. } = taffy.layout(node100).unwrap();
    assert_eq!(size.width, 248f32, "width of node {:?}. Expected {}. Actual {}", node100, 248f32, size.width);
    assert_eq!(size.height, 394f32, "height of node {:?}. Expected {}. Actual {}", node100, 394f32, size.height);
    assert_eq!(location.x, 32f32, "x of node {:?}. Expected {}. Actual {}", node100, 32f32, location.x);
    assert_eq!(location.y, 32f32, "y of node {:?}. Expected {}. Actual {}", node100, 32f32, location.y);
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_width(),
        0f32,
        "scroll_width of node {:?}. Expected {}. Actual {}",
        node100,
        0f32,
        layout.scroll_width()
    );
    #[cfg(feature = "content_size")]
    assert_eq!(
        layout.scroll_height(),
        0f32,
        "scroll_height of node {:?}. Expected {}. Actual {}",
        node100,
        0f32,
        layout.scroll_height()
    );
}
