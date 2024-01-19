use parser_generator::boxed::Parser;
use parser_generator::unboxed::{match_literal, zero_or_more};

#[test]
fn one_ore_more_combinator() {
    let parser = zero_or_more(match_literal("ha"));

    assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
    assert_eq!(Ok(("ahah", vec![])), parser.parse("ahah"));
    assert_eq!(Ok(("", vec![])), parser.parse(""));
}
