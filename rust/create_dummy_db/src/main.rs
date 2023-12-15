use std::fs::File;
use std::io::{BufReader, Read};

use base64::{engine::general_purpose, Engine as _};
use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;
use rand::{thread_rng, Rng};
use tokio_postgres::Error;

use common::db_art2img::insert_art2img;
use common::db_articles::insert_article;
use common::db_image::insert_image;
use common::db_resolution::insert_resolution;
use common::models::{NewArt2Img, NewArticle, NewImage, NewResolution};
use common::utils::create_pool;

use crate::pngimages::create_image;

mod pngimages;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::new().filter_level(LevelFilter::Info).init();

    // insert_dev_data().await?;
    // dump_tables("dev".into()).await?;

    insert_prod_data().await?;

    Ok(())
}

async fn insert_dev_data() -> Result<(), Error> {
    let resolutions = vec![
        NewResolution {
            resolution: "64x64".to_string(),
        },
        NewResolution {
            resolution: "256x256".to_string(),
        },
        NewResolution {
            resolution: "320x240".to_string(),
        },
    ];
    let id = "dev".to_string();
    let cnt_articles = 5;
    let min_cnt_images = 1;
    let max_cnt_images = 3;
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
        NewResolution {
            resolution: "64x64".to_string(),
        },
        NewResolution {
            resolution: "256x256".to_string(),
        },
        NewResolution {
            resolution: "320x240".to_string(),
        },
        NewResolution {
            resolution: "640x480".to_string(),
        },
        NewResolution {
            resolution: "1280x720".to_string(),
        },
        NewResolution {
            resolution: "1920x1200".to_string(),
        },
        NewResolution {
            resolution: "3840x2160".to_string(),
        },
        NewResolution {
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
    resolutions: Vec<NewResolution>,
) -> Result<(), Error> {
    let mut rng = thread_rng();

    let path = env!("CARGO_MANIFEST_DIR");

    let img_min_width = 1000;
    let img_max_width = 4096;
    let ratio = 16.0 / 9.0;

    let pool = create_pool(id);

    for art_idx in 0..cnt_articles {
        let new_article = NewArticle {
            code: format!("{:010}", art_idx + 1),
            title: format!("title for article code {:010}", art_idx + 1),
            description: format!(
                "a long text description for the article with code {:010}",
                art_idx
            ),
        };

        let article = insert_article(&pool, &new_article).await?;

        let cnt_images = rng.gen_range(min_cnt_images..max_cnt_images);
        for img_idx in 0..cnt_images {
            let img_width = rng.gen_range(img_min_width..img_max_width);
            let img_height = (img_width as f64 / ratio) as usize;

            let filename = format!(
                "article_{:010}_{:02}_{:02}",
                art_idx + 1,
                img_idx + 1,
                cnt_images
            );

            create_image(
                img_width, img_height, art_idx, img_idx, cnt_images, &filename, &mut rng,
            );
            let png_filenamne = format!("{}/images/png/{}.png", path, &filename);

            let f = File::open(png_filenamne).expect("open");
            let mut reader = BufReader::new(f);
            let mut buffer = Vec::new();

            // Read file into vector.
            reader
                .read_to_end(&mut buffer)
                .expect("read file into buffer");

            let encoded: String = general_purpose::STANDARD_NO_PAD.encode(buffer);

            let new_image = NewImage {
                filename: format!(
                    "img_{:010}_{:02}_{:02}.png",
                    art_idx + 1,
                    img_idx + 1,
                    cnt_images
                ),
                img_data: encoded,
            };

            let image = insert_image(&pool, &new_image).await?;

            let new_art2img = NewArt2Img {
                article_id: article.id,
                image_id: image.id,
            };

            let art2img = insert_art2img(&pool, &new_art2img).await?;

            println!("new art2img inserted    {:?}", &art2img);
        }
    }

    for r in &resolutions {
        insert_resolution(&pool, r).await?;
    }

    Ok(())
}
