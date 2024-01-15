use std::time::Instant;

use log::info;
use warp::{Filter, Rejection, Reply};

use common::db_art2img::read_art2img;
use common::db_articles::{read_articles, read_articles_paginated};
use common::db_image::read_images;
use common::db_resolution::read_resolutions;
use common::models::{ArticleModel, ImageModel, PixelModel};
use common::utils::build_response_from_json;
use commonbefe::models::{Article, Image, Resolution};

use crate::POOL;
use crate::utils::{create_ppm, create_ppm_all_in_one, crop_image, get_sorted_resolutions, invert_image, mirror_image};

pub fn article_routes_single_json_array() -> impl Filter<Extract=(impl Reply, ), Error=Rejection> + Clone {
    let server1 = warp::path!("singlethreaded" / "api" /  "v1" / "articles");
    let find_all_single_json_array = server1
        .and(warp::get())
        .and_then(|| {
            info!("GET /singlethreaded/api/articles/");
            find_all_singlethreaded_json_array()
        });

    let server1 = warp::path!("singlethreaded" / "api" /  "v1" / "articles" / u32 / u32);
    let find_paginated_single_json_array = server1
        .and(warp::get())
        .and_then(|page_number: u32, page_size: u32| {
            info!("GET /singlethreaded/api/articles/{page_number}/{page_size}");
            find_paginated_single_json_array(page_number, page_size)
        });


    let server1 = warp::path!("singlethreaded" / "api" /  "v2" / "articles" / u32 / u32);
    let find_paginated_single_json_array_v2 = server1
        .and(warp::get())
        .and_then(|page_number: u32, page_size: u32| {
            info!("GET /singlethreaded/api/v2/articles/{page_number}/{page_size}");
            find_paginated_single_json_array_v2(page_number, page_size)
        });

    find_all_single_json_array
        .or(find_paginated_single_json_array)
        .or(find_paginated_single_json_array_v2)
}

pub async fn find_all_singlethreaded_json_array() -> Result<impl Reply, Rejection> {
    let resolutions =
        get_sorted_resolutions(read_resolutions(&POOL).await.expect("read resolutions"));
    let articles = read_articles(&POOL).await.expect("read articles");
    let response = resize_all_images_single_json_array(articles, resolutions).await?;
    Ok(response)
}

pub async fn resize_all_images_single_json_array(
    articles: Vec<ArticleModel>,
    resolutions: Vec<Resolution>,
) -> Result<impl Reply, Rejection> {
    let mut full_articles: Vec<Article> = vec![];
    for article in &articles {
        let mut full_images: Vec<Image> = vec![];
        for resolution in &resolutions {
            let art2imgs = read_art2img(&POOL, article.id as i32)
                .await
                .expect("read art2imgs");
            let imgids: Vec<i32> = art2imgs.iter().map(|art2img| art2img.image_id as i32).collect();
            let images = read_images(&POOL, &imgids).await.expect("read images");

            let mut images_resized = resize_single_json_array(images, resolution);
            full_images.append(&mut images_resized);
        }

        let full_article = Article {
            code: article.code.clone(),
            title: article.title.clone(),
            description: article.description.clone(),
            images: full_images,
        };
        full_articles.push(full_article);
    }
    let response = build_response_from_json(full_articles);

    Ok(response)
}

pub async fn resize_all_images_single_json_array_v2(
    articles: Vec<ArticleModel>,
    resolutions: Vec<Resolution>,
) -> Result<impl Reply, Rejection> {
    let mut full_articles: Vec<Article> = vec![];
    for article in &articles {
        let mut full_images: Vec<Image> = vec![];
        for resolution in &resolutions {
            let art2imgs = read_art2img(&POOL, article.id as i32)
                .await
                .expect("read art2imgs");
            let imgids: Vec<i32> = art2imgs.iter().map(|art2img| art2img.image_id as i32).collect();
            let images = read_images(&POOL, &imgids).await.expect("read images");

            let mut images_resized = resize_single_json_array_v2(images, resolution);
            full_images.append(&mut images_resized);
        }

        let full_article = Article {
            code: article.code.clone(),
            title: article.title.clone(),
            description: article.description.clone(),
            images: full_images,
        };
        full_articles.push(full_article);
    }
    let response = build_response_from_json(full_articles);

    Ok(response)
}


pub async fn find_paginated_single_json_array(
    page_number: u32,
    page_size: u32,
) -> Result<impl Reply, Rejection> {
    let start = Instant::now();
    let resolutions =
        get_sorted_resolutions(read_resolutions(&POOL).await.expect("read resolutions"));
    let articles = read_articles_paginated(&POOL, page_number, page_size)
        .await
        .expect("read articles");

    let dur = start.elapsed().as_millis();
    info!("find_paginated_single took  {} ms", dur);

    let response = resize_all_images_single_json_array(articles, resolutions).await?;

    Ok(response)
}


pub async fn find_paginated_single_json_array_v2(
    page_number: u32,
    page_size: u32,
) -> Result<impl Reply, Rejection> {
    let start = Instant::now();
    let resolutions =
        get_sorted_resolutions(read_resolutions(&POOL).await.expect("read resolutions"));
    let articles = read_articles_paginated(&POOL, page_number, page_size)
        .await
        .expect("read articles");

    let dur = start.elapsed().as_millis();
    info!("find_paginated_single took  {} ms", dur);

    let response = resize_all_images_single_json_array_v2(articles, resolutions).await?;

    Ok(response)
}


pub(crate) fn resize_single_json_array(images: Vec<ImageModel>, resolution: &Resolution) -> Vec<Image> {
    images
        .iter()
        .map(|img| resize_image_single_json_array(resolution, img))
        .collect()
}

fn resize_single_json_array_v2(images: Vec<ImageModel>, resolution: &Resolution) -> Vec<Image> {
    images
        .iter()
        .map(|img| resize_image_single_json_array_v2(resolution, img))
        .collect()
}

fn resize_image_single_json_array(resolution: &Resolution, img: &ImageModel) -> Image {
    let pixels: Vec<PixelModel> = serde_json::from_str(&img.image_as_json_pixels_array).expect("deserialize should work");
    let res = match resolution.original {
        true => {
            Resolution {
                name: resolution.name.clone(),
                width: img.width as i32,
                height: img.height as i32,
                original: true,
            }
        }
        false => {
            Resolution {
                name: resolution.name.clone(),
                width: resolution.width,
                height: resolution.height,
                original: false,
            }
        }
    };

    let mirrored_pixels = mirror_image(&pixels, img.width as usize, img.height as usize);
    let cropped_pixels = crop_image(&mirrored_pixels, img.width as usize, res.width as usize, res.height as usize);
    let inverted_pixels = invert_image(&cropped_pixels, res.width as usize, res.height as usize);
    let ppm = create_ppm(&inverted_pixels, res.width as usize, res.height as usize);

    Image {
        filename: img.filename.clone(),
        image: ppm,
        resolution: res.name.clone(),
        width: res.width,
        height: res.height,
    }
}

fn resize_image_single_json_array_v2(resolution: &Resolution, img: &ImageModel) -> Image {
    let pixels: Vec<PixelModel> = serde_json::from_str(&img.image_as_json_pixels_array).expect("deserialize should work");
    let res = match resolution.original {
        true => {
            Resolution {
                name: resolution.name.clone(),
                width: img.width as i32,
                height: img.height as i32,
                original: true,
            }
        }
        false => {
            Resolution {
                name: resolution.name.clone(),
                width: resolution.width,
                height: resolution.height,
                original: false,
            }
        }
    };

    let ppm = create_ppm_all_in_one(&pixels, img.width as usize, img.height as usize, res.width as usize, res.height as usize);

    Image {
        filename: img.filename.clone(),
        image: ppm,
        resolution: res.name.clone(),
        width: res.width,
        height: res.height,
    }
}


