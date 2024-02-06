use std::convert::Infallible;

use log::info;
use warp::Filter;

use crate::articleservice::import_articles;
use crate::models::{ImportRequest, ImportResult, Sysinfo};

pub fn routes() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
    let server = warp::path!("api" / "v1" / "sysinfo" );
    let get_sysinfo = server
        .and(warp::get())
        .and_then(|| {
            info!("GET /api/v1/sysinfo");
            sysinfo_v1()
        });

    let server = warp::path!("api" / "v1" / "articles" / "import" );
    let import_articles = server
        .and(warp::post())
        .and(import_request())
        .and_then(|req: ImportRequest| {
            info!("POST /api/v1/articles/import");
            import_articles_v1(req.return_items)
        });

    get_sysinfo.or(import_articles)
}

fn import_request() -> impl Filter<Extract=(ImportRequest, ), Error=warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn import_articles_v1(return_items: bool) -> Result<impl warp::Reply, Infallible> {
    let response = import_articles().expect("importing data should work");
    if return_items {
        let response = warp::reply::json(&response);
        Ok(response)
    } else {
        let res = ImportResult {
            lines_processed: response.lines_processed,
            db_rows_written: response.db_rows_written,
            items: vec![],
        };
        let response = warp::reply::json(&res);
        Ok(response)
    }
}

pub async fn sysinfo_v1() -> Result<impl warp::Reply, Infallible> {
    let sysinfo = Sysinfo {
        author: "gsc".to_string(),
        version: "v1".to_string(),
        comment: "none".to_string(),
        language: "rust".to_string(),
        framework: "warp".to_string(),
        multithreaded: false,
    };
    let response = warp::reply::json(&sysinfo);

    Ok(response)
}
