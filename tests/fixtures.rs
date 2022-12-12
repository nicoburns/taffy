// this declaration is necessary to "mount" the generated code where cargo can see it
// this allows us to both keep code generation scoped to a singe directory for fs events
// and to keep each test in a separate file
mod generated;

use std::path::Component;

use html_parser::{Dom, Element, Node};
use swc_common::input::StringInput;
use swc_common::source_map::BytePos;
use swc_css_ast::{self as ast, ComponentValue, Declaration, DeclarationName, Ident, StyleBlock};
use swc_css_parser::parse_string_input;
use swc_css_parser::parser::ParserConfig;
use taffy::prelude::*;

const EMPTY_STRING: String = String::new();

const HTML: &str = r###"
  <div id="test-root" style="flex-direction: row; flex-wrap: wrap; width: 100px; height: 200px; column-gap: 10px; row-gap: 20px; align-items:flex-end;">
    <div style="width: 20px; "></div>
    <div style="width: 20px;"></div>
    <div style="width: 20px;"></div>
    <div style="width: 20px;"></div>
    <div style="width: 20px;"></div>
    <div style="width: 20px;"></div>
  </div>
"###;

const HTML2: &str = r###"
  <div id="test-root" style="width: 140px; display: grid; grid-template-columns: 40px minmax(20px, 40px) 40px;grid-template-rows: 40px 40px 40px;">
    <div></div>
  </div>
"###;

#[test]
fn parse_html() {
    let tree = Dom::parse(HTML2).unwrap();
    let root = &tree.children[0];

    parse_node(root);

    assert_eq!(true, false);
}

fn parse_node(node: &Node) {
    match node {
        Node::Element(element) => parse_element(element),
        Node::Text(text) => println!("{text}"),
        Node::Comment(text) => println!("{text}"),
    }
}

fn parse_element(element: &Element) {
    // dbg!(&element);

    println!("{}", element.name);
    let styles = element.attributes.get("style");
    if let Some(Some(styles)) = styles {
        println!("  {styles}");
        parse_style(styles);
    }

    for child in &element.children {
        parse_node(child);
    }
}

fn parse_style(style_text: &str) -> Style {
    let input = StringInput::new(style_text, BytePos(0), BytePos(style_text.len() as u32));
    let mut errors = vec![];
    let parsed_styles = parse_string_input::<Vec<StyleBlock>>(input, ParserConfig::default(), &mut errors).unwrap();

    // dbg!(errors);
    // dbg!(parsed_styles);

    let declarations = parsed_styles.iter().filter_map(|style_block| match style_block {
        StyleBlock::Declaration(decl) => Some(decl),
        _ => None,
    });

    let mut style = Style::default();
    for decl in declarations {
        let name = parse_declaration_name(&decl.name);
        match name {
            "display" => {
                let ComponentValue::Ident(ident) = &decl.value[0] else { continue; };
                style.display = match ident_as_str(ident) {
                    "grid" => Display::Grid,
                    "none" => Display::None,
                    "flex" | _ => Display::Flex,
                };
            }
            "width" => {
                let Some(width) = parse_dimension(&decl.value[0]) else { continue; };
                style.size.width = width;
            }
            "height" => {
                let Some(height) = parse_dimension(&decl.value[0]) else { continue; };
                style.size.height = height;
            }
            "min-width" => {
                let Some(width) = parse_dimension(&decl.value[0]) else { continue; };
                style.min_size.width = width;
            }
            "min-height" => {
                let Some(height) = parse_dimension(&decl.value[0]) else { continue; };
                style.min_size.height = height;
            }
            "max-width" => {
                let Some(width) = parse_dimension(&decl.value[0]) else { continue; };
                style.max_size.width = width;
            }
            "max-height" => {
                let Some(height) = parse_dimension(&decl.value[0]) else { continue; };
                style.max_size.height = height;
            }
            _ => {}
        };
        // dbg!(decl);
    }

    dbg!(style.size.width);

    style
}

fn ident_as_str(ident: &Ident) -> &str {
    ident.raw.as_ref().unwrap()
}

fn parse_declaration_name(name: &DeclarationName) -> &str {
    let DeclarationName::Ident(ident) = name else { return "" };
    ident_as_str(ident)
}

fn parse_dimension(raw_value: &ComponentValue) -> Option<Dimension> {
    match raw_value {
        ComponentValue::Percentage(percentage) => Some(Dimension::Percent(percentage.value.value as f32)),
        ComponentValue::Dimension(ast::Dimension::Length(length)) if ident_as_str(&length.unit) == "px" => {
            Some(points(length.value.value as f32))
        }
        ComponentValue::Ident(ident) => match ident_as_str(ident) {
            "auto" => auto(),
            _ => None,
        },
        _ => {
            dbg!(raw_value);
            None
        }
    }
}

fn parse_length_percentage_auto(raw_value: &ComponentValue) -> Option<LengthPercentageAuto> {
    parse_dimension(raw_value).and_then(|dim| dim.try_into().ok())
}

fn parse_length_percentage(raw_value: &ComponentValue) -> Option<LengthPercentageAuto> {
    parse_dimension(raw_value).and_then(|dim| dim.try_into().ok())
}
