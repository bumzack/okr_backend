#![allow(unused)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use common::models::PixelModel;
use parser_generator::boxed::Parser;
use parser_generator::unboxed::serialize_pixel_vec;

pub fn test_parser_combinator(c: &mut Criterion) {
    let input = " [  {   \"r\"  :  1   ,   \"g\"  :  2   ,   \"b\"  :  3  } ,  {   \"r\"  :  11   ,   \"g\"  :  12   ,   \"b\"  :  13  } ,  {   \"r\"  :  31   ,   \"g\"  :  32   ,   \"b\"  :  33  }    ]";
    c.bench_function("parser combinators 20", |b| {
        b.iter(|| {
            let _ = serialize_pixel_vec()
                .parse(black_box(input))
                .expect("should succeed");
        })
    });
}

pub fn test_serde(c: &mut Criterion) {
    let input = " [  {   \"r\"  :  1   ,   \"g\"  :  2   ,   \"b\"  :  3  } ,  {   \"r\"  :  11   ,   \"g\"  :  12   ,   \"b\"  :  13  } ,  {   \"r\"  :  31   ,   \"g\"  :  32   ,   \"b\"  :  33  }    ]";
    c.bench_function("serde", |b| {
        b.iter(|| {
            let pixels: Vec<PixelModel> =
                serde_json::from_str(black_box(input)).expect("serde should succeed");
        })
    });
}

criterion_group!(benches, test_parser_combinator, test_serde);
criterion_main!(benches);
