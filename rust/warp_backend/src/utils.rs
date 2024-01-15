use common::models::{PixelModel, ResolutionModel};
use commonbefe::models::Resolution;

pub fn to_resolution(r: &ResolutionModel) -> Resolution {
    if r.resolution.eq("original") {
        Resolution {
            name: r.resolution.clone(),
            width: -1,
            height: -1,
            original: true,
        }
    } else {
        let width_height: Vec<&str> = r.resolution.split('x').collect();
        let w = width_height[0]
            .parse::<i32>()
            .expect("width should be a number ");
        let h = width_height[1]
            .parse::<i32>()
            .expect("height should be a number ");

        Resolution {
            name: r.resolution.clone(),
            width: w,
            height: h,
            original: false,
        }
    }
}

pub fn get_sorted_resolutions(resoultions: Vec<ResolutionModel>) -> Vec<Resolution> {
    let mut originals: Vec<Resolution> = resoultions
        .iter()
        .filter(|r| r.resolution.eq("original"))
        .map(|r| to_resolution(r))
        .collect();

    let mut others: Vec<Resolution> = resoultions
        .iter()
        .filter(|r| !r.resolution.eq("original"))
        .map(|r| to_resolution(r))
        .collect();

    others.sort_by(|a, b| b.width.cmp(&a.width));

    originals.append(&mut others);

    originals
}

pub fn create_ppm(pixels: &Vec<PixelModel>, width: usize, height: usize) -> String {
    let mut ppm = format!("P3\n{} {}\n255\n", width, height);
    let mut line = String::new();
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let p = &pixels[idx];
            let pixel_as_string = format!("{} {} {} ", p.r, p.g, p.b);
            if line.len() + pixel_as_string.len() > 70 {
                ppm.push_str(&line);
                ppm.push_str("\n");
                line = pixel_as_string;
            } else {
                line.push_str(&pixel_as_string);
            }
        }
        ppm.push_str(&line);
        line = "".to_string();
    }

    ppm
}

pub fn mirror_image(pixels: &Vec<PixelModel>, width: usize, height: usize) -> Vec<PixelModel> {
    let mut mirrored = vec![];

    for y in 0..height {
        for x in 0..width {
            let xx = width - x - 1;
            let yy = height - y - 1;
            let idx = yy * width + xx;
            let p = pixels[idx].clone();
            mirrored.push(p);
        }
    }

    mirrored
}

pub fn crop_image(pixels: &Vec<PixelModel>, source_width: usize, cropped_width: usize, cropped_height: usize) -> Vec<PixelModel> {
    let mut cropped = vec![];

    for y in 0..cropped_height {
        for x in 0..cropped_width {
            let idx = y * source_width + x;
            let p = pixels[idx].clone();
            cropped.push(p);
        }
    }

    cropped
}


pub fn invert_image(pixels: &Vec<PixelModel>, width: usize, height: usize) -> Vec<PixelModel> {
    let mut inverted = vec![];

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let p = &pixels[idx];

            let inverted_pixel = PixelModel {
                r: 255 - p.r,
                g: 255 - p.g,
                b: 255 - p.b,
            };
            inverted.push(inverted_pixel);
        }
    }

    inverted
}

pub fn create_ppm_all_in_one(pixels: &Vec<PixelModel>, source_width: usize, source_height: usize, width: usize, height: usize) -> String {
    let mut ppm = format!("P3\n{} {}\n255\n", width, height);
    let mut line = String::new();
    for y in 0..height {
        for x in 0..width {
            let xx = source_width - x - 1;
            let yy = source_height - y - 1;
            let idx = yy * source_width + xx;
            let p = &pixels[idx];
            let pixel_as_string = format!("{} {} {} ", 255 - p.r, 255 - p.g, 255 - p.b);
            if line.len() + pixel_as_string.len() > 70 {
                ppm.push_str(&line);
                ppm.push_str("\n");
                line = pixel_as_string;
            } else {
                line.push_str(&pixel_as_string);
            }
        }
        ppm.push_str(&line);
        line = "".to_string();
    }

    ppm
}
