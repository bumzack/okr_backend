use parser_generator::boxed::Parser;
use parser_generator::unboxed::{any_char, pred};

#[test]
fn predicate_combinator() {
    let parser = pred(any_char, |c| *c == 'o');

    assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
    assert_eq!(Err("lol"), parser.parse("lol"));
}
