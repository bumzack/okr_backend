use std::net::{SocketAddr, ToSocketAddrs};

use log::{info, LevelFilter};
use pretty_env_logger::env_logger::Builder;
use tokio_postgres::Error;
use warp::Filter;

use common::utils::{create_pool, warp_cors};

use crate::multi_base64::article_routes_multi_base64;
use crate::multi_json_array::article_routes_multi_json_array;
use crate::single_base64::article_routes_single_base64;
use crate::single_json::article_routes_single_json_array;

mod multi_base64;
mod multi_json_array;
mod single_base64;
mod single_json;
mod utils;

// #[tokio::main(worker_threads = 1)]
#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let pool = create_pool("prod".into());

    let host: String = "localhost".to_string();
    let port: u16 = 2345;

    let host = format!("{host}:{port}");
    let routes = article_routes_multi_base64(pool.clone())
        .or(article_routes_multi_json_array(pool.clone()))
        .or(article_routes_single_json_array(pool.clone()))
        .or(article_routes_single_base64(pool))
        .with(warp_cors());

    info!("warp server host {}", host);
    let socket_addrs: Vec<SocketAddr> = host.to_socket_addrs().unwrap().collect();
    let addr = socket_addrs.first().unwrap();
    warp::serve(routes).run(*addr).await;

    Ok(())
}
