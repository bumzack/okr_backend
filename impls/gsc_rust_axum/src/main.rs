use std::ffi::OsString;
use std::io::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

use axum::{BoxError, Json, Router};
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use serde_derive::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
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
        .route("/api/v1/sysinfo", get(sysinfo))
        .route("/api/v1/articles/import", post(import_articles))
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

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImportRequest {
    pub return_items: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
    pub lines_processed: usize,
    pub db_rows_written: usize,
    pub items: Vec<Article>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub code: String,
    pub title: String,
    pub description: String,
    pub categories: String,
    pub attributes: String,
    pub price: f64,
    pub start_date: String,
    pub end_date: String,
    pub pos: String,
}


#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Sysinfo {
    pub author: String,
    pub comment: String,
    pub framework: String,
    pub language: String,
    pub version: String,
    pub multithreaded: bool,
}

async fn import_articles(Json(input): Json<ImportRequest>) -> impl IntoResponse {
    println!("request  {:?}", input);
    let mut res = ImportResult {
        lines_processed: 1,
        db_rows_written: 2,
        items: vec![],
    };
    let files = read_files().await.expect("should read files");
    for f in &files {
        println!("file  {:?}", f);
        let mut res_file = process_file(f, input.return_items).await.expect("should processs a file");
        res.db_rows_written += res_file.db_rows_written;
        res.lines_processed += res_file.lines_processed;
        res.items.append(&mut res_file.items);
    }

    (StatusCode::OK, Json(res))
}


async fn sysinfo() -> impl IntoResponse {
    let si = Sysinfo {
        author: "gsc".to_string(),
        comment: "impl".to_string(),
        framework: "axum".to_string(),
        language: "rust".to_string(),
        version: "v1".to_string(),
        multithreaded: false,
    };
    (StatusCode::OK, Json(si))
}


async fn read_files() -> Result<Vec<OsString>, Error> {
    // let path = Path::new("/home/bumzack/stoff/okr_backend/data");
    let paths = fs::read_dir("/home/bumzack/stoff/okr_backend/data").unwrap();

    let mut files: Vec<OsString> = vec![];
    for path in paths {
        if path.as_ref().is_ok() {
            let f = path.unwrap();
            if f.file_name().to_str().unwrap().ends_with(".txt") {
                files.push(f.file_name());
            }
        }
    }
    files.sort_by(|a: &OsString, b: &OsString| a.to_str().partial_cmp(&b.to_str()).unwrap());
    Ok(files)
}

// enum AppError {
//     // The request body contained invalid JSON
//     JsonRejection(JsonRejection),
//     // Some error from a third party library we're using
//     TimeError(time_library::Error),
// }


async fn process_file(f: &OsString, return_items: bool) -> Result<ImportResult, Error> {
    let filename = format!("{}/{}","/home/bumzack/stoff/okr_backend/data", f.to_str().expect("should be a filename"));
    println!("filename  {}", &filename);
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    let ir = ImportResult {
        lines_processed: 1,
        db_rows_written: 2,
        items: vec![],
    };
    Ok(ir)
}

