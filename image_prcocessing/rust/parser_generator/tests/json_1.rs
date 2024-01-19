use common::models::PixelModel;
use parser_generator::boxed::{
    parse_number_property, parse_number_property_vec_char, Parser, whitespace_wrap,
};
use parser_generator::unboxed::{
    int_value_vec, left, match_literal, ParseResult, pixel_array, right, serialize_pixel_vec,
    triple,
};

#[test]
fn parse_number() {
    let input = "23";
    let actual = int_value_vec().parse(input);

    let expected = Ok(("", vec!['2', '3']));
    assert_eq!(expected, actual);
}

#[test]
fn parse_number_property_test() {
    let parse_number_property = right(match_literal(":"), int_value_vec());
    let input_red = ":8";
    let actual = parse_number_property.parse(input_red);
    let expected = Ok(("", vec!['8']));
    println!("actual {:?}", actual);
    println!("expected {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn parse_number_property2() {
    let parse_number_property = right(match_literal(":"), int_value_vec());
    let input_red = ":8";
    let actual = parse_number_property.parse(input_red);
    let expected = Ok(("", vec!['8']));
    println!("actual {:?}", actual);
    println!("expected {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn quoted_string_test() {
    let parse_red = right(
        match_literal("\"r\""),
        right(match_literal(":"), int_value_vec()),
    );
    let input = "\"r\":8";
    let actual = parse_red.parse(input);
    let expected = Ok(("", vec!['8']));
    println!("actual {:?}", actual);
    println!("expected {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn quoted_string_test2() {
    let input = " \"r\"   :  8   ";
    let actual = parse_number_property_vec_char().parse(input);
    let expected = Ok(("", vec!['8']));
    println!("actual {:?}", actual);
    println!("expected {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn parse_number_property_test2() {
    let input = " \"r\"   :  8   ";
    let actual = parse_number_property_vec_char().parse(input);
    let expected = Ok(("", vec!['8']));
    println!("actual {:?}", actual);
    println!("expected {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn parse_number_property_test3() {
    let input = " \"r\"   :  8   ";
    let actual = parse_number_property("r").parse(input);
    let expected = Ok(("", 8));
    println!("actual {:?}", actual);
    println!("expected {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn parse_number_property_test4_multiple_colors() {
    let input = " \"r\"   :  1 ,  \"g\"   :  2 ,  \"b\"   :  3  ";

    let red = parse_number_property("r");
    let green = parse_number_property("g");
    let blue = parse_number_property("b");
    let color_properties = triple(
        left(red, whitespace_wrap(match_literal(","))),
        left(green, whitespace_wrap(match_literal(","))),
        blue,
    );

    let actual = color_properties.parse(input);
    let actual_rest = "";
    let expected = Ok((actual_rest, (1, 2, 3)));
    println!("actual {:?}", actual);
    println!("expected {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn pixel_array_test_1_element() {
    let input = " [  {   \"r\"  :  1   ,   \"g\"  :  2   ,   \"b\"  :  3  }     ]";

    let actual: ParseResult<Vec<PixelModel>> = pixel_array().parse(input);
    let actual_rest = "";
    let p = PixelModel { r: 1, g: 2, b: 3 };

    let expected = Ok((actual_rest, vec![p]));
    println!("actual {:?}", actual);
    println!("expected {:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn pixel_array_test_multiple_element() {
    let input = " [  {   \"r\"  :  1   ,   \"g\"  :  2   ,   \"b\"  :  3  } ,  {   \"r\"  :  11   ,   \"g\"  :  12   ,   \"b\"  :  13  } ,  {   \"r\"  :  31   ,   \"g\"  :  32   ,   \"b\"  :  33  }    ]";

    let actual: ParseResult<Vec<PixelModel>> = serialize_pixel_vec().parse(input);
    let actual_rest = "";
    let p = PixelModel { r: 1, g: 2, b: 3 };
    let p2 = PixelModel {
        r: 11,
        g: 12,
        b: 13,
    };
    let p3 = PixelModel {
        r: 31,
        g: 32,
        b: 33,
    };
    let expected = Ok((actual_rest, vec![p, p2, p3]));
    println!("actual {:?}", actual);
    println!("expected {:?}", expected);
    assert_eq!(actual, expected);
}
