use std::env;
use std::io::{Error};

use dotenvy::dotenv;
use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;
use warp::Filter;

use crate::server::routes;

mod models;
mod articleservice;
mod server;


#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    dotenv().expect(".env file not found");
    let port =  env::var("PORT").expect("PORT").parse::<u16>().expect("port must be a number");
    let routes = routes().with(warp::log("rust_warp"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;

    Ok(())
}
