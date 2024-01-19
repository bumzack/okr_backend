use parser_generator::boxed::Parser;
use parser_generator::boxed::quoted_string;

#[test]
fn predicate_combinator() {
    assert_eq!(
        Ok(("", "Hello Joe!".to_string())),
        quoted_string().parse("\"Hello Joe!\"")
    );
}
