use std::net::{SocketAddr, ToSocketAddrs};

use deadpool_postgres::Pool;
use log::{info, LevelFilter};
use pretty_env_logger::env_logger::Builder;
use tokio_postgres::Error;
use warp::Filter;

use common::utils::{create_pool, warp_cors};

use crate::multi_json_array::article_routes_multi_json_array;
use crate::single_json::article_routes_single_json_array;

mod multi_json_array;
mod single_json;
mod utils;

lazy_static::lazy_static! {
    static ref POOL: Pool = create_pool("dev".into());
}

// #[tokio::main(worker_threads = 1)]
#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let host: String = "192.168.0.115".to_string();
    let port: u16 = 2345;

    let host = format!("{host}:{port}");
    let routes =
        article_routes_multi_json_array()
            .or(article_routes_single_json_array())
            .with(warp_cors());

    info!("warp server host {}", host);
    let socket_addrs: Vec<SocketAddr> = host.to_socket_addrs().unwrap().collect();
    let addr = socket_addrs.first().unwrap();
    warp::serve(routes).run(*addr).await;

    Ok(())
}
