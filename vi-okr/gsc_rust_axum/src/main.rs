use std::time::Duration;

use axum::{BoxError, Router};
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::routing::{get, post};
use dotenvy::dotenv;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::v1::{import_articles_v1, sysinfo_v1};
use crate::v2::{import_articles_v2, sysinfo_v2};
use crate::v3::{import_articles_v3, sysinfo_v3};
use std::{env};

mod models;
mod stuff;
mod v1;
mod v2;
mod v3;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_todos=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Builder::new()
    //     .filter_level(LevelFilter::Info)
    //     .init();

    // Compose the routes
    let app = Router::new()
        .route("/api/v1/sysinfo", get(sysinfo_v1))
        .route("/api/v1/articles/import", post(import_articles_v1))
        .route("/api/v2/sysinfo", get(sysinfo_v2))
        .route("/api/v2/articles/import", post(import_articles_v2))
        .route("/api/v3/sysinfo", get(sysinfo_v3))
        .route("/api/v3/articles/import", post(import_articles_v3))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let port = env::var("PORT").expect("PORT");
    let server = format!("127.0.0.1:{}", port);

    println!("server running on {}", &server);

    let listener = tokio::net::TcpListener::bind(server)
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
