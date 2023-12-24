#![type_length_limit = "144635175"]

use parser_generator::boxed::Parser;
use parser_generator::unboxed::{Element, single_element};

#[test]
fn attribute_parser() {
    let expected = Element {
        name: "div".to_string(),
        attributes: vec![("class".to_string(), "float".to_string())],
        children: vec![],
    };

    let s = "<div class=\"float\"/>";

    assert_eq!(Ok(("", expected)), single_element().parse(s));
}
