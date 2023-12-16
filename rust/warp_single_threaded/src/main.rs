use std::convert::Infallible;
use std::net::{SocketAddr, ToSocketAddrs};

use base64::engine::general_purpose;
use base64::Engine;
use deadpool_postgres::Pool;
use image::imageops::FilterType;
use image::ImageFormat;
use log::{info, LevelFilter};
use pretty_env_logger::env_logger::Builder;
use serde::Serialize;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use tokio_postgres::Error;
use warp::http::StatusCode;
use warp::reply::Response;
use warp::{Filter, Rejection, Reply};

use common::db_art2img::read_art2img;
use common::db_articles::read_articles;
use common::db_image::read_images;
use common::db_resolution::read_resolutions;
use common::models::{Image, Resolution};
use common::utils::create_pool;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let pool = create_pool("dev".into());

    let host: String = "localhost".to_string();
    let port: u16 = 2345;

    let host = format!("{host}:{port}");
    let routes = price_route(pool);

    info!("priceservice host {}", host);
    let socket_addrs: Vec<SocketAddr> = host.to_socket_addrs().unwrap().collect();
    let addr = socket_addrs.first().unwrap();
    warp::serve(routes).run(*addr).await;

    Ok(())
}

pub fn price_route(pool: Pool) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let server1 = warp::path!("api" / "articles");
    let search_name = server1
        .and(with_db(pool.clone()))
        .and(warp::get())
        .and_then(|pool: Pool| {
            info!("GET /api/articles");
            read_price_entry(pool)
        });

    search_name
}

pub async fn read_price_entry(pool: Pool) -> Result<impl Reply, Rejection> {
    let resolutions = read_resolutions(&pool).await.expect("read resolutions");
    let articles = read_articles(&pool).await.expect("read articles");

    let mut full_articles: Vec<FullArticle> = vec![];
    for article in &articles {
        let mut ful_images: Vec<FullImage> = vec![];
        for resolution in &resolutions {
            let art2imgs = read_art2img(&pool, article.id)
                .await
                .expect("read art2imgs");
            let imgids: Vec<i64> = art2imgs.iter().map(|art2img| art2img.image_id).collect();
            let images = read_images(&pool, &imgids).await.expect("read images");

            let mut images_resized = resize(images, resolution);
            ful_images.append(&mut images_resized);
        }

        let full_article = FullArticle {
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

fn resize(images: Vec<Image>, resolution: &Resolution) -> Vec<FullImage> {
    let res_images: Vec<FullImage> = images
        .iter()
        .map(|img| resize_image(resolution, img))
        .collect();

    res_images
}

fn resize_image(resolution: &Resolution, img: &Image) -> FullImage {
    // y let time = Utc::now().timestamp_millis();
    //  let filename = format!("./original_{}.png", time);
    let decoded: Vec<u8> = general_purpose::STANDARD_NO_PAD
        .decode(&img.img_data)
        .expect("decoding should work");

    let i = image::load_from_memory_with_format(&decoded, ImageFormat::Png)
        .expect("Vec<u8> to PNG image");
    // i.save(filename).expect("should save a file");

    let width_height: Vec<&str> = resolution.resolution.split('x').collect();
    let w = width_height[0].parse::<u32>().unwrap();
    let h = width_height[1].parse::<u32>().unwrap();

    let resized = i.resize(w, h, FilterType::Nearest);

    // let filename = format!("./resized_{}.png", time);
    // resized.save(filename).expect("should save a file");

    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(resized.as_bytes());

    FullImage {
        filename: img.filename.clone(),
        image: encoded,
        resolution: resolution.resolution.clone(),
    }
}

pub fn with_db(pool: Pool) -> impl Filter<Extract = (Pool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
//
// fn search_prices_request(
// ) -> impl Filter<Extract = (SearchPricesRequest,), Error = Rejection> + Clone {
//     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
// }

#[derive(Serialize, Deserialize)]
struct FullImage {
    filename: String,
    image: String,
    resolution: String,
}

#[derive(Serialize, Deserialize)]
struct FullArticle {
    code: String,
    title: String,
    description: String,
    images: Vec<FullImage>,
}

pub fn build_response_from_json<T: Serialize>(json: T) -> Response {
    build_response_from_json_with_status(json, StatusCode::OK)
}

pub fn build_response_from_json_with_status<T: Serialize>(json: T, status: StatusCode) -> Response {
    let value = json!(&json);
    let reply = warp::reply::json(&value);
    let reply = warp::reply::with_status(reply, status);
    reply.into_response()
}
