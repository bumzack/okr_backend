use std::net::{SocketAddr, ToSocketAddrs};

use log::{info, LevelFilter};
use pretty_env_logger::env_logger::Builder;
use tokio_postgres::Error;
use warp::Filter;

use common::utils::{create_pool, warp_cors};

use crate::multi::article_routes_multi;
use crate::single::article_routes_single;

mod multi;
mod rayon;
mod single;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let pool = create_pool("dev".into());

    let host: String = "localhost".to_string();
    let port: u16 = 2345;

    let host = format!("{host}:{port}");
    let routes = article_routes_multi(pool.clone())
        .or(article_routes_single(pool))
        .with(warp_cors());

    info!("warp server host {}", host);
    let socket_addrs: Vec<SocketAddr> = host.to_socket_addrs().unwrap().collect();
    let addr = socket_addrs.first().unwrap();
    warp::serve(routes).run(*addr).await;

    Ok(())
}
