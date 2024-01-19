use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};

use image::GenericImageView;
use serde::{Deserialize, Serialize};
use serde_json::json;

use common::models::PixelModel;

fn main() {
    let image = load_image();

    println!("image w {}, h {}", image.w, image.h);
}

#[derive(Debug, Serialize, Deserialize)]
struct Img {
    w: usize,
    h: usize,
    pixels: Vec<PixelModel>,
}

impl Img {
    fn get(&self, x: usize, y: usize) -> &PixelModel {
        let idx = y * self.w + x;
        &self.pixels[idx]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut PixelModel {
        let idx = y * self.w + x;
        &mut self.pixels[idx]
    }

    fn set(&mut self, x: usize, y: usize, p: PixelModel) {
        let idx = y * self.w + x;
        self.pixels[idx] = p;
    }
}

//
// impl Index<usize> for Img {
//     type Output = [Pix];
//     fn index(&self, index: usize) -> &Self::Output {
//
//         &self.pixels[index * self.h .. (index+1) * self.h]
//     }
// }

fn load_image() -> Img {
    let path = env!("CARGO_MANIFEST_DIR");
    let filename = "shrug";

    let png_filename = format!("{}/{}.png", path, &filename);

    let f = File::open(png_filename).expect("open");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader
        .read_to_end(&mut buffer)
        .expect("read file into buffer");

    let i = image::load_from_memory_with_format(&buffer, image::ImageFormat::Png);

    let i = i.expect("should be an image");

    let pixels: Vec<PixelModel> = i
        .pixels()
        //    .take(100)
        .map(|p| {
            // if p.2.0[0] != 255 {
            //     println!("pixel  {:?}", p);
            // }
            PixelModel {
                r: p.2.0[0],
                g: p.2.0[1],
                b: p.2.0[2],
            }
        })
        .collect();

    let mut img = Img {
        w: i.width() as usize,
        h: i.height() as usize,
        pixels,
    };

    let json = json!(&img).to_string();
    let json_filename = format!("{}/{}.json", path, &filename);
    fs::write(json_filename, &json).expect("should write a json file");

    let p = img.get(378, 501);
    println!("pixel {:?}", p);

    let p = PixelModel {
        r: 127,
        g: 127,
        b: 127,
    };
    img.set(378, 501, p);

    let p = img.get(378, 501);
    println!("pixel {:?}", p);

    let p = img.get_mut(378, 501);
    p.r = 28;
    p.g = 29;
    p.b = 30;
    println!("pixel {:?}", p);

    let p = img.get(378, 501);
    println!("pixel after get mut  {:?}", p);
    img
}
