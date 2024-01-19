use std::convert::Infallible;

use deadpool_postgres::Pool;
use deadpool_postgres::{Manager, ManagerConfig, RecyclingMethod};
use log::info;
use serde::Serialize;
use serde_json::json;
use tokio_postgres::{Client, Error, NoTls};
use warp::cors::Builder;
use warp::http::StatusCode;
use warp::reply::Response;
use warp::{Filter, Reply};

use crate::models::{Art2ImgModel, ArticleModel, ImageModel, ResolutionModel};

pub async fn dump_tables(id: String) -> Result<(), Error> {
    let client = create_connection(id).await?;

    let articles = client.query("SELECT * FROM articles", &[]).await?;
    articles.iter().for_each(|a| {
        let a = ArticleModel::from(a);
        println!("article {:?}, {}", a.code, a.title);
    });

    let images = client.query("SELECT * FROM images", &[]).await?;
    images.iter().for_each(|a| {
        let image = ImageModel::from(a);
        println!(
            "image    id {:?}, filename {}   {}x{}",
            image.id, image.filename, image.width, image.height
        );
    });

    let art2img = client.query("SELECT * FROM art2img", &[]).await?;
    art2img.iter().for_each(|a| {
        println!("art2img {:?}", Art2ImgModel::from(a));
    });

    let resolutions = client.query("SELECT * FROM resolutions", &[]).await?;
    resolutions.iter().for_each(|a| {
        println!("resolutions {:?}", ResolutionModel::from(a));
    });

    Ok(())
}

pub fn create_pool(id: String) -> Pool {
    let mut pg_config = tokio_postgres::Config::new();

    let user: String = id.clone();
    let password: String = id.clone();
    let host: String = "localhost".into();
    let dbname: String = id.clone();
    let port: u16 = 54321;

    info!("user {user}, password {password}, host {host}, dbname {dbname}");
    pg_config.user(&user);
    pg_config.password(&password);
    pg_config.host(&host);
    pg_config.dbname(&dbname);
    pg_config.port(port);
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    Pool::builder(mgr).max_size(16).build().unwrap()
}

pub fn get_db_config(id: String) -> deadpool_postgres::Config {
    let mut config = deadpool_postgres::Config::new();
    config.user = Some(id.clone());
    config.password = Some(id.clone());
    config.dbname = Some(id.clone());
    config.host = Some("localhost".into());
    config.port = Some(5434);

    config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    config
}

pub async fn create_connection(id: String) -> Result<Client, Error> {
    let config = format!(
        "host=localhost hostaddr=127.0.0.1 user={} password={} port=5434 dbname={}",
        id.clone(),
        id.clone(),
        id.clone()
    );
    let (client, connection) = tokio_postgres::connect(&config, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
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

pub fn warp_cors() -> Builder {
    warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "content-type",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Access-Control-Allow-Headers",
            "Access-Control-Allow-Methods",
            "Access-Control-Allow-Origin",
            "Access-Control-Expose-Headers",
            "Access-Control-Request-Headers",
            "Access-Control-Request-Methods",
            "Accept-Encoding",
            "Accept-Language",
            "Accept-Post",
            "Access-Control-Allow-Credentials",
            "keep-alive",
        ])
        .allow_methods(vec!["POST", "GET", "OPTIONS", "PUT", "DELETE", "HEAD"])
}

pub fn with_db(pool: Pool) -> impl Filter<Extract = (Pool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

//
// fn search_prices_request(
// ) -> impl Filter<Extract = (SearchPricesRequest,), Error = Rejection> + Clone {
//     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
// }
