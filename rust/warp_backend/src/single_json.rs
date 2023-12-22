use std::io::Cursor;
use std::time::Instant;

use base64::engine::general_purpose;
use base64::Engine;
use deadpool_postgres::Pool;
use image::imageops::FilterType;
use image::{ImageFormat, ImageOutputFormat};
use log::info;
use warp::{Filter, Rejection, Reply};

use common::db_art2img::read_art2img;
use common::db_articles::{read_articles, read_articles_paginated};
use common::db_image::read_images;
use common::db_resolution::read_resolutions;
use common::models::{ArticleModel, ImageModel};
use common::utils::{build_response_from_json, with_db};
use commonbefe::models::{Article, Image, Resolution};

use crate::utils::get_sorted_resolutions;

pub fn article_routes_single_json_array(
    pool: Pool,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let server1 = warp::path!("singlethreaded" / "api" / "articles");
    let find_all_single_json_array = server1
        .and(with_db(pool.clone()))
        .and(warp::get())
        .and_then(|pool: Pool| {
            info!("GET /singlethreaded/api/articles/");
            find_all_singlethreaded_json_array(pool)
        });

    let server1 = warp::path!("singlethreaded" / "api" / "articles" / u32 / u32);
    let find_paginated_single_json_array = server1
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(|page_number: u32, page_size: u32, pool: Pool| {
            info!("GET /singlethreaded/api/articles/{page_number}/{page_size}");
            find_paginated_single_json_array(pool, page_number, page_size)
        });

    find_all_single_json_array.or(find_paginated_single_json_array)
}

pub async fn find_all_singlethreaded_json_array(pool: Pool) -> Result<impl Reply, Rejection> {
    let resolutions =
        get_sorted_resolutions(read_resolutions(&pool).await.expect("read resolutions"));
    let articles = read_articles(&pool).await.expect("read articles");

    let response = resize_all_images_single_json_array(pool.clone(), articles, resolutions).await?;

    Ok(response)
}

pub async fn resize_all_images_single_json_array(
    pool: Pool,
    articles: Vec<ArticleModel>,
    resolutions: Vec<Resolution>,
) -> Result<impl Reply, Rejection> {
    let mut full_articles: Vec<Article> = vec![];
    for article in &articles {
        let mut ful_images: Vec<Image> = vec![];
        for resolution in &resolutions {
            let art2imgs = read_art2img(&pool, article.id)
                .await
                .expect("read art2imgs");
            let imgids: Vec<i32> = art2imgs.iter().map(|art2img| art2img.image_id).collect();
            let images = read_images(&pool, &imgids).await.expect("read images");

            let mut images_resized = resize_single_json_array(images, resolution);
            ful_images.append(&mut images_resized);
        }

        let full_article = Article {
            code: article.code.clone(),
            title: article.title.clone(),
            description: article.description.clone(),
            images: ful_images,
        };
        full_articles.push(full_article);
    }
    let response = build_response_from_json(full_articles);

    Ok(response)
}

pub async fn find_paginated_single_json_array(
    pool: Pool,
    page_number: u32,
    page_size: u32,
) -> Result<impl Reply, Rejection> {
    let start = Instant::now();
    let resolutions =
        get_sorted_resolutions(read_resolutions(&pool).await.expect("read resolutions"));
    let articles = read_articles_paginated(&pool, page_number, page_size)
        .await
        .expect("read articles");

    let dur = start.elapsed().as_millis();
    info!("find_paginated_single took  {} ms", dur);

    let response = resize_all_images_single_json_array(pool.clone(), articles, resolutions).await?;

    Ok(response)
}

fn resize_single_json_array(images: Vec<ImageModel>, resolution: &Resolution) -> Vec<Image> {
    images
        .iter()
        .map(|img| resize_image_single_json_array(resolution, img))
        .collect()
}

fn resize_image_single_json_array(resolution: &Resolution, img: &ImageModel) -> Image {
    // let time = Utc::now().timestamp_millis();
    // let filename = format!("./original_{}_{}.png", time, resolution.name);

    if resolution.original {
        // TODO criticala: img.img_data.clone() is probably costly
        Image {
            filename: img.filename.clone(),
            image: img.image_as_json_pixels_array.clone(),
            resolution: resolution.name.clone(),
        }
    } else {
        let decoded: Vec<u8> = general_purpose::STANDARD_NO_PAD
            .decode(&img.image_as_json_pixels_array)
            .expect("decoding should work");

        let i = image::load_from_memory_with_format(&decoded, ImageFormat::Png)
            .expect("Vec<u8> to PNG image");
        // i.save(filename).expect("should save a file");

        let resized = i.resize(
            resolution.width as u32,
            resolution.height as u32,
            FilterType::Gaussian,
        );

        // let filename = format!(
        //     "./resized_{}_{}x{}.png",
        //     time, resolution.width, resolution.height
        // );
        // resized.save(filename).expect("should save a file");

        // https://stackoverflow.com/questions/57457818/how-to-convert-dynamicimage-to-base64
        let mut image_data: Vec<u8> = Vec::new();
        resized
            .write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
            .unwrap();
        let encoded: String = general_purpose::STANDARD_NO_PAD.encode(image_data);

        Image {
            filename: img.filename.clone(),
            image: encoded,
            resolution: resolution.name.clone(),
        }
    }
}
