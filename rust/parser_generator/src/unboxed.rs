use common::models::PixelModel;

use crate::boxed::{either, parse_number_property, Parser, quoted_string, whitespace_wrap};

pub fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Element {
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Element>,
}

pub fn identifier(input: &str) -> ParseResult<String> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '-' {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}

pub type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

impl<'a, F, Output> Parser<'a, Output> for F
    where
        F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
    where
        P: Parser<'a, A>,
        F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

pub fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
    where
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}

pub fn triple<'a, P1, P2, P3, R1, R2, R3>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
) -> impl Parser<'a, (R1, R2, R3)>
    where
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>,
        P3: Parser<'a, R3>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2.parse(next_input).and_then(|(next_input, result2)| {
                parser3
                    .parse(next_input)
                    .map(|(last_input, result3)| (last_input, (result1, result2, result3)))
            })
        })
    }
}

pub fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
    where
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

pub fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
    where
        P1: Parser<'a, R1>,
        P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}

pub fn one_or_more1<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
    where
        P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_time)) = parser.parse(input) {
            input = next_input;
            result.push(first_time);
        } else {
            return Err(input);
        }

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

pub fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
    where
        P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

pub fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
    where
        P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_item)) = parser.parse(input) {
            input = next_input;
            result.push(first_item);
        } else {
            return Err(input);
        }

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

// pub fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
//     where
//         P: Parser<'a, A>  {
//     map(pair(parser, zero_or_more(parser)), |(head, mut tail)| {
//         tail.insert(0, head);
//         tail
//     })
// }

pub fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        _ => Err(input),
    }
}

pub fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
    where
        P: Parser<'a, A>,
        F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, value)) = parser.parse(input) {
            if predicate(&value) {
                return Ok((next_input, value));
            }
        }
        Err(input)
    }
}

pub fn whitespace_char<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_whitespace())
}

pub fn number<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_numeric())
}

pub fn int_value_vec<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(number())
}

pub fn int_value<'a>() -> impl Parser<'a, u8> {
    move |input| {
        let p = one_or_more(number());

        if let Ok((next_input, arr)) = p.parse(input) {
            let s: String = arr.into_iter().collect();
            let v = s.parse::<u8>();
            match v {
                Ok(i) => Ok((next_input, i)),
                Err(_) => Err(input),
            }
        } else {
            Err(input)
        }
    }
}

pub fn space1<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

pub fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}

pub fn attribute_pair<'a>() -> impl Parser<'a, (String, String)> {
    pair(identifier, right(match_literal("="), quoted_string()))
}

pub fn attributes<'a>() -> impl Parser<'a, Vec<(String, String)>> {
    zero_or_more(right(space1(), attribute_pair()))
}

pub fn element_start<'a>() -> impl Parser<'a, (String, Vec<(String, String)>)> {
    right(match_literal("<"), pair(identifier, attributes()))
}

pub fn single_element<'a>() -> impl Parser<'a, Element> {
    map(
        left(element_start(), match_literal("/>")),
        |(name, attributes)| Element {
            name,
            attributes,
            children: vec![],
        },
    )
}

pub fn pixel_obj<'a>() -> impl Parser<'a, PixelModel> {
    let red = parse_number_property("r");
    let green = parse_number_property("g");
    let blue = parse_number_property("b");

    right(
        whitespace_wrap(match_literal("{")),
        whitespace_wrap(left(
            triple(
                left(red, whitespace_wrap(match_literal(","))),
                left(green, whitespace_wrap(match_literal(","))),
                blue,
            ),
            whitespace_wrap(match_literal("}")),
        )),
    )
        .map(|(r, g, b)| PixelModel { r, g, b })
}

pub fn pixel_array<'a>() -> impl Parser<'a, Vec<PixelModel>> {
    right(
        whitespace_wrap(match_literal("[")),
        whitespace_wrap(left(
            one_or_more(pixel_obj()),
            whitespace_wrap(match_literal("]")),
        )),
    )
}

pub fn serialize_pixel_vec<'a>() -> impl Parser<'a, Vec<PixelModel>> {
    whitespace_wrap(either(list_with_one_pixel(), list_with_multiple_pixel()))
}

pub fn list_with_one_pixel<'a>() -> impl Parser<'a, Vec<PixelModel>> {
    right(
        whitespace_wrap(match_literal("[")),
        whitespace_wrap(left(
            one_or_more(pixel_obj()),
            whitespace_wrap(match_literal("]")),
        )),
    )
}

pub fn list_with_multiple_pixel<'a>() -> impl Parser<'a, Vec<PixelModel>> {
    right(
        whitespace_wrap(match_literal("[")),
        left(
            whitespace_wrap(blupp()),
            whitespace_wrap(match_literal("]")),
        ),
    )
}

fn blupp<'a>() -> impl Parser<'a, Vec<PixelModel>> {
    move |input| {
        let p = pair(
            whitespace_wrap(pixel_obj()),
            zero_or_more(right(
                whitespace_wrap(match_literal(",")),
                whitespace_wrap(pixel_obj()),
            )),
        );

        if let Ok((next_input, (p, mut arr))) = p.parse(input) {
            arr.insert(0, p);
            Ok((next_input, arr))
        } else {
            Err(input)
        }
    }
}
