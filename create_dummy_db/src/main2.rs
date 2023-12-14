use std::sync::Once;

use magick_rust::{magick_wand_genesis, MagickWand};
use rand::prelude::*;
use svg::Document;

static START: Once = Once::new();

const IMAGE_WIDTH: usize = 64;
const IMAGE_HEIGHT: usize = 64;

fn main() {
    let mut rng = thread_rng();

    for i in 0..100 {
        rectangle(&mut rng, i);
        circle(&mut rng, i);
        //  ellipse(&mut rng);
        x(&mut rng, i);
    }
}

fn x(rnd: &mut ThreadRng, i: usize) {
    let cx = rnd.gen_range(10..IMAGE_WIDTH as i32 / 2);
    let cy = rnd.gen_range(10..IMAGE_HEIGHT as i32 / 2);
    let a = rnd.gen_range(10..IMAGE_WIDTH as i32 / 2) / 2;

    let line1 = svg::node::element::Line::new()
        .set("x1", cx - a)
        .set("y1", cy - a)
        .set("x2", cx + a)
        .set("y2", cy + a)
        .set("stroke", "black")
        .set("stroke-width", 2)
        .set("fill", "none");

    let line2 = svg::node::element::Line::new()
        .set("x1", cx - a)
        .set("y1", cy + a)
        .set("x2", cx + a)
        .set("y2", cy - a)
        .set("stroke", "black")
        .set("stroke-width", 2)
        .set("fill", "none");

    let document = Document::new()
        .set("viewBox", (0, 0, IMAGE_WIDTH, IMAGE_HEIGHT))
        .add(line1)
        .add(line2);

    let filename = format!("x_{}", i);
    let full_path = format!("./images/svg/{}.svg", &filename);
    svg::save(full_path, &document).unwrap();
    convert_to_png(&filename);
}

fn circle(rnd: &mut ThreadRng, i: usize) {
    let min_radius = 5;
    let x = rnd.gen_range((2 * min_radius)..IMAGE_WIDTH as i32 / 2);
    let y = rnd.gen_range((2 * min_radius)..IMAGE_HEIGHT as i32 / 2);
    let r = rnd.gen_range(min_radius..IMAGE_WIDTH as i32 / 2);

    let c = svg::node::element::Circle::new()
        .set("cx", x)
        .set("cy", y)
        .set("r", r)
        .set("stroke", "black")
        .set("stroke-width", 2)
        .set("fill", "none");

    let document = Document::new()
        .set("viewBox", (0, 0, IMAGE_WIDTH, IMAGE_HEIGHT))
        .add(c);

    let filename = format!("circle_{}", i);
    let full_path = format!("./images/svg/{}.svg", &filename);
    svg::save(full_path, &document).unwrap();
    convert_to_png(&filename);
}

fn rectangle(rnd: &mut ThreadRng, i: usize) {
    let x1 = rnd.gen_range(0..IMAGE_WIDTH as i32 / 2);
    let y1 = rnd.gen_range(0..IMAGE_HEIGHT as i32 / 2);

    let width = rnd.gen_range(5..IMAGE_WIDTH as i32 / 2);
    let height = rnd.gen_range(5..IMAGE_HEIGHT as i32 / 2);

    let c = svg::node::element::Rectangle::new()
        .set("x", x1)
        .set("y", y1)
        .set("width", width)
        .set("height", height)
        .set("stroke", "black")
        .set("stroke-width", 2)
        .set("fill", "none");

    let document = Document::new()
        .set("viewBox", (0, 0, IMAGE_WIDTH, IMAGE_HEIGHT))
        .add(c);

    let filename = format!("rectangle_{}", i);
    let full_path = format!("./images/svg/{}.svg", &filename);
    svg::save(full_path, &document).unwrap();
    convert_to_png(&filename);
}

fn convert_to_png(filename: &str) {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let input = format!("./images/svg/{}.svg", &filename);
    let wand = MagickWand::new();
    wand.read_image(&input).expect("should find it");
    wand.fit(IMAGE_WIDTH, IMAGE_HEIGHT);
    let output = format!("./images/{}.png", filename);
    let x = wand.write_image(&output);

    match x {
        Ok(()) => println!("file save ok "),
        Err(e) => println!("file save crashed  {}", e),
    }
}
