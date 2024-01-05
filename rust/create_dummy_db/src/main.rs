use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Instant;

use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;
use deadpool_postgres::Pool;
use image::{GenericImageView, ImageBuffer, ImageFormat, RgbImage};
use log::{info, LevelFilter};
use pretty_env_logger::env_logger::Builder;
use rand::{Rng, thread_rng};
use serde_json::json;
use tokio_postgres::Error;

use common::db_art2img::insert_art2img;
use common::db_articles::insert_article;
use common::db_image::insert_image;
use common::db_resolution::insert_resolution;
use common::models::{
    NewArt2ImgModel, NewArticleModel, NewImageModel, NewResolutionModel, PixelModel,
};
use common::utils::create_pool;

use crate::pngimages::create_image_vec_u8;

mod pngimages;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let start = Instant::now();
    insert_dev_data().await?;

    let elapsed = start.elapsed().as_secs();

    info!("inserting articles & images took {} secs", elapsed);

    Ok(())
}

async fn insert_dev_data() -> Result<(), Error> {
    let resolutions = vec![
        NewResolutionModel {
            resolution: "32x32".to_string(),
        },
        NewResolutionModel {
            resolution: "original".to_string(),
        },
        NewResolutionModel {
            resolution: "256x256".to_string(),
        },
    ];
    let id = "dev".to_string();
    let cnt_articles = 100;
    let min_cnt_images = 2;
    let max_cnt_images = 5;

    let img_min_width = 600;
    let img_max_width = 700;
    let ratio = 16.0 / 9.0;

    insert_data(
        id.clone(),
        cnt_articles,
        min_cnt_images,
        max_cnt_images,
        img_min_width,
        img_max_width,
        ratio,
        resolutions,
    )
        .await?;
    Ok(())
}

async fn insert_prod_data() -> Result<(), Error> {
    let resolutions = vec![
        NewResolutionModel {
            resolution: "1920x1200".to_string(),
        },
        NewResolutionModel {
            resolution: "64x64".to_string(),
        },
        NewResolutionModel {
            resolution: "original".to_string(),
        },
        NewResolutionModel {
            resolution: "1280x720".to_string(),
        },
        NewResolutionModel {
            resolution: "256x256".to_string(),
        },
    ];

    let id = "prod".to_string();
    let cnt_articles = 1000;
    let min_cnt_images = 4;
    let max_cnt_images = 10;

    let img_min_width = 3000;
    let img_max_width = 4000;
    let ratio = 16.0 / 9.0;

    insert_data(
        id.clone(),
        cnt_articles,
        min_cnt_images,
        max_cnt_images,
        img_min_width,
        img_max_width,
        ratio,
        resolutions,
    )
        .await?;
    Ok(())
}

async fn insert_data(
    id: String,
    cnt_articles: usize,
    min_cnt_images: usize,
    max_cnt_images: usize,
    img_min_width: usize,
    img_max_width: usize,
    ratio: f64,
    resolutions: Vec<NewResolutionModel>,
) -> Result<(), Error> {
    let mut rng = thread_rng();

    let pool = create_pool(id);

    for art_idx in 0..cnt_articles {
        let ts = Utc::now().timestamp_millis();
        let code = rng.gen_range(0..100_000_000);
        let article_code = format!("article_{:010}_{:010}_{}", code, art_idx + 1, ts);

        let new_article = NewArticleModel {
            code: article_code.clone(),
            title: format!("title for article code {:010}", article_code.clone()),
            description: format!(
                "a long text description for the article with code {}. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.    Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse molestie consequat, vel illum dolore eu feugiat nulla facilisis at vero eros et accumsan et iusto odio dignissim qui blandit praesent luptatum zzril delenit augue duis dolore te feugait nulla facilisi. Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam nonummy nibh euismod tincidunt ut laoreet dolore magna aliquam erat volutpat.    Ut wisi enim ad minim veniam, quis nostrud exerci tation ullamcorper suscipit lobortis nisl ut aliquip ex ea commodo consequat. Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse molestie consequat, vel illum dolore eu feugiat nulla facilisis at vero eros et accumsan et iusto odio dignissim qui blandit praesent luptatum zzril delenit augue duis dolore te feugait nulla facilisi.    Nam liber tempor cum soluta nobis eleifend option congue nihil imperdiet doming id quod mazim placerat facer possim assum. Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam nonummy nibh euismod tincidunt ut laoreet dolore magna aliquam erat volutpat. Ut wisi enim ad minim veniam, quis nostrud exerci tation ullamcorper suscipit lobortis nisl ut aliquip ex ea commodo consequat.    Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse molestie consequat, vel illum dolore eu feugiat nulla facilisis.   At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, At accusam aliquyam diam diam dolore dolores duo eirmod eos erat, et nonumy sed tempor et et invidunt justo labore Stet clita ea et gubergren, kasd magna no rebum. sanctus sea sed takimata ut vero voluptua. est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur",
                article_code
            ),
        };

        let article = insert_article(&pool, &new_article).await?;

        let cnt_images = rng.gen_range(min_cnt_images..max_cnt_images);
        for img_idx in 0..cnt_images {
            let img_width = rng.gen_range(img_min_width..img_max_width);
            let img_height = (img_width as f64 / ratio) as usize;

            let buffer = create_image_vec_u8(
                img_width,
                img_height,
                img_idx,
                cnt_images,
                &mut rng,
                article_code.clone(),
            );

            let converted_with_format =
                image::load_from_memory_with_format(&buffer, ImageFormat::Png).unwrap();

            let mut rgb_pixels: Vec<PixelModel> = vec![];

            for y in 0..img_height {
                for x in 0..img_width {
                    let yy = img_height - 1 - y;
                    let xx = img_width - 1 - x;

                    let p = converted_with_format.get_pixel(xx as u32, yy as u32);
                    let new_pixel = PixelModel {
                        r: 255 - p[0],
                        g: 255 - p[1],
                        b: 255 - p[2],
                    };
                    rgb_pixels.push(new_pixel);
                }
            }

            let rgb_pixels = json!(&rgb_pixels).to_string();
            let encoded: String = general_purpose::STANDARD_NO_PAD.encode(buffer);

            let new_image = NewImageModel {
                filename: format!(
                    "img_{}_{:02}_{:02}.png",
                    article_code,
                    img_idx + 1,
                    cnt_images
                ),
                image_as_rgb_png: encoded,
                image_as_json_pixels_array: rgb_pixels,
                width: img_width as u32,
                height: img_height as u32,
            };

            let image = insert_image(&pool, &new_image).await?;

            let new_art2img = NewArt2ImgModel {
                article_id: article.id,
                image_id: image.id,
            };

            let art2img = insert_art2img(&pool, &new_art2img).await?;

            info!("new art2img inserted    {:?}", &art2img);
        }
    }

    insert_resolutions(&resolutions, &pool).await?;

    Ok(())
}

async fn insert_resolutions(resolutions: &Vec<NewResolutionModel>, pool: &Pool) -> Result<(), Error> {
    for r in resolutions {
        insert_resolution(&pool, r).await?;
    }
    Ok(())
}

fn save_ppm(pixels: &[PixelModel], filename: &String, width: usize, height: usize) {
    let mut f = File::create(filename).expect("create file");
    let s = format!("P3 \n {} {} \n {}", width, height, 255);
    let _ = f.write(&s.into_bytes()).expect("expect to write files");
    pixels.iter().for_each(|p| {
        let s = format!(" {} {} {} \n ", p.r, p.g, p.b);
        let _ = f.write(&s.into_bytes()).expect("expect to write files");
    })
}

fn save_png(pixels: &[PixelModel], filename: &String, width: usize, height: usize) {
    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let idx = (y * width as u32 + x) as usize;

        let r = pixels[idx].r;
        let g = pixels[idx].g;
        let b = pixels[idx].b;

        *pixel = image::Rgb([r, g, b]);
    }

    let p = Path::new(filename);
    img.save_with_format(p, ImageFormat::Png)
        .expect("saving png should work");
}
