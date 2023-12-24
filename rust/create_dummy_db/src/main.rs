use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};

use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use image::{GenericImageView, ImageFormat};
use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;
use rand::{thread_rng, Rng};
use serde_json::json;
use tokio_postgres::Error;

use common::db_art2img::insert_art2img;
use common::db_articles::insert_article;
use common::db_image::insert_image;
use common::db_resolution::insert_resolution;
use common::models::{NewArt2ImgModel, NewArticleModel, NewImageModel, NewResolutionModel};
use common::utils::{create_pool, dump_tables};

use crate::pngimages::create_image;

mod pngimages;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::new().filter_level(LevelFilter::Info).init();

    insert_dev_data().await?;
    dump_tables("dev".into()).await?;

    //  insert_prod_data().await?;

    Ok(())
}

async fn insert_dev_data() -> Result<(), Error> {
    let resolutions = vec![
        NewResolutionModel {
            resolution: "256x256".to_string(),
        },
        NewResolutionModel {
            resolution: "320x240".to_string(),
        },
        NewResolutionModel {
            resolution: "original".to_string(),
        },
        NewResolutionModel {
            resolution: "64x64".to_string(),
        },
    ];
    let id = "dev".to_string();
    let cnt_articles = 2;
    let min_cnt_images = 2;
    let max_cnt_images = 6;

    insert_data(
        id.clone(),
        cnt_articles,
        min_cnt_images,
        max_cnt_images,
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
            resolution: "640x480".to_string(),
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
        NewResolutionModel {
            resolution: "320x240".to_string(),
        },
        NewResolutionModel {
            resolution: "3840x2160".to_string(),
        },
        NewResolutionModel {
            resolution: "7680x4320".to_string(),
        },
    ];

    let id = "prod".to_string();
    let cnt_articles = 1_000;
    let min_cnt_images = 2;
    let max_cnt_images = 10;
    insert_data(
        id.clone(),
        cnt_articles,
        min_cnt_images,
        max_cnt_images,
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
    resolutions: Vec<NewResolutionModel>,
) -> Result<(), Error> {
    let mut rng = thread_rng();

    let path = env!("CARGO_MANIFEST_DIR");

    let img_min_width = 3000;
    let img_max_width = 4096;
    let ratio = 16.0 / 9.0;

    let pool = create_pool(id);

    for art_idx in 0..cnt_articles {
        let ts = Utc::now().timestamp_millis();
        let code = rng.gen_range(0..100_000_000);
        let article_code = format!("article_{:010}_{:010}_{}", code, art_idx + 1, ts);
        println!("art_idx {art_idx}  -->   code {article_code}");
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

            let filename = format!(
                "img_{}_{:02}_{:02}",
                article_code.clone(),
                img_idx + 1,
                cnt_images
            );

            create_image(
                img_width,
                img_height,
                img_idx,
                cnt_images,
                &filename,
                &mut rng,
                article_code.clone(),
            );
            let png_filename = format!("{}/images/png/{}.png", path, &filename);

            let f = File::open(png_filename).expect("open");
            let mut reader = BufReader::new(f);
            let mut buffer = Vec::new();

            reader
                .read_to_end(&mut buffer)
                .expect("read file into buffer");

            let converted_with_format =
                image::load_from_memory_with_format(&buffer, ImageFormat::Png).unwrap();

            let rgb_pixels: Vec<PixelModel> = converted_with_format
                .pixels()
                .into_iter()
                .map(|p| PixelModel {
                    r: p.2 .0[0],
                    g: p.2 .0[1],
                    b: p.2 .0[2],
                })
                .collect();

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

            let _ = insert_art2img(&pool, &new_art2img).await?;

            let png_filename = format!("{}/images/png/{}.png", path, &filename);
            fs::remove_file(&png_filename).expect("file delete should work");
            let svg_filename = format!("{}/images/svg/{}.svg", path, &filename);
            fs::remove_file(&svg_filename).expect("file delete should work");

            //  println!("new art2img inserted    {:?}", &art2img);
        }
    }

    for r in &resolutions {
        insert_resolution(&pool, r).await?;
    }

    Ok(())
}
