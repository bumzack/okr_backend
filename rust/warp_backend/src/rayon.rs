use std::io::Cursor;

use base64::Engine;
use base64::engine::general_purpose;
use deadpool_postgres::Pool;
use image::{ImageFormat, ImageOutputFormat};
use image::imageops::FilterType;
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

pub fn article_routes_rayon(
    pool: Pool,
) -> impl Filter<Extract=(impl Reply, ), Error=Rejection> + Clone {
    let server1 = warp::path!("rayon" / "api" / "articles");
    let find_all_rayon = server1
        .and(with_db(pool.clone()))
        .and(warp::get())
        .and_then(|pool: Pool| {
            info!("GET /rayon/api/articles");
            find_all_rayonthreaded_rayon(pool)
        });

    let server1 = warp::path!("rayon" / "api" / "articles" / u32 / u32);
    let find_paginated_rayon = server1
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(|page_number: u32, page_size: u32, pool: Pool| {
            info!("GET /rayon/api/articles/{page_number}/{page_size}");
            find_paginated_rayon(pool, page_number, page_size)
        });

    find_all_rayon.or(find_paginated_rayon)
}

pub async fn find_all_rayonthreaded_rayon(pool: Pool) -> Result<impl Reply, Rejection> {
    let resolutions =
        get_sorted_resolutions(read_resolutions(&pool).await.expect("read resolutions"));
    let articles = read_articles(&pool).await.expect("read articles");

    let response = resize_all_images_rayon(pool.clone(), articles, resolutions).await?;

    Ok(response)
}

pub async fn resize_all_images_rayon(
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
            let imgids: Vec<i64> = art2imgs.iter().map(|art2img| art2img.image_id).collect();
            let images = read_images(&pool, &imgids).await.expect("read images");

            let mut images_resized = resize_rayon(images, resolution);
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

pub async fn find_paginated_rayon(
    pool: Pool,
    page_number: u32,
    page_size: u32,
) -> Result<impl Reply, Rejection> {
    let resolutions =
        get_sorted_resolutions(read_resolutions(&pool).await.expect("read resolutions"));
    let articles = read_articles_paginated(&pool, page_number, page_size)
        .await
        .expect("read articles");

    let response = resize_all_images_rayon(pool.clone(), articles, resolutions).await?;

    Ok(response)
}

fn resize_rayon(mut images: Vec<ImageModel>, resolution: &Resolution) -> Vec<Image> {
    // let mut others = vec![];
    // let res_images: Vec<Image> = images
    //     .par_iter_mut()
    //     .map(|img| {
    //         let i = resize_image_rayon(resolution, img);
    //         i
    //     })
    //     .collect();
    // 
    images
        .iter()
        .map(|img| resize_image_rayon(resolution, img))
        .collect()

    // images
    //     .par_iter()
    //     .map(|img| resize_image_rayon(resolution, img))
    //     .collect()
}

fn resize_image_rayon(resolution: &Resolution, img: &ImageModel) -> Image {
    // let time = Utc::now().timestamp_millis();
    // let filename = format!("./original_{}_{}.png", time, resolution.name);

    if resolution.original {
        // TODO criticala: img.img_data.clone() is probably costly
        Image {
            filename: img.filename.clone(),
            image: img.img_data.clone(),
            resolution: resolution.name.clone(),
        }
    } else {
        let decoded: Vec<u8> = general_purpose::STANDARD_NO_PAD
            .decode(&img.img_data)
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
