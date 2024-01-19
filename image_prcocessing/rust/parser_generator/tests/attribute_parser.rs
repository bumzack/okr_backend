#![type_length_limit = "1239834"]

use parser_generator::boxed::Parser;
use parser_generator::unboxed::attributes;

#[test]
fn attribute_parser() {
    let expected = vec![
        ("one".to_string(), "1".to_string()),
        ("two".to_string(), "2".to_string()),
    ];

    let s = "   one=\"1\"     two=\"2\"";

    assert_eq!(Ok(("", expected)), attributes().parse(s));
}
