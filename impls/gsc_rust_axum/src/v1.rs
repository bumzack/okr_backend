use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::models::{Article, ImportRequest, ImportResult, Sysinfo};
use crate::stuff::{Code, Pos};

pub async fn import_articles_v1(Json(input): Json<ImportRequest>) -> impl IntoResponse {
    println!("request  {:?}", input);
    let mut res = ImportResult {
        lines_processed: 1,
        db_rows_written: 2,
        items: vec![],
    };
    let files = read_files().await.expect("should read files");
    for f in &files {
        println!("file  {:?}", f);
        let mut res_file = process_file(f, input.return_items)
            .await
            .expect("should processs a file");

        res.db_rows_written += res_file.db_rows_written;
        res.lines_processed += res_file.lines_processed;
        res.items.append(&mut res_file.items);
    }

    (StatusCode::OK, Json(res))
}

pub async fn sysinfo_v1() -> impl IntoResponse {
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

async fn process_file(f: &OsString, return_items: bool) -> Result<ImportResult, Error> {
    let filename = format!(
        "{}/{}",
        "/home/bumzack/stoff/okr_backend/data",
        f.to_str().expect("should be a filename")
    );
    println!("filename  {}   return_items {}", &filename, return_items);

    let mut lines_processed = 0;
    let mut db_rows_written = 0;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut iter = reader.lines();
    let mut line: Option<Result<String, Error>> = iter.next();
    lines_processed += 1;

    let mut article_grouped_by_code_and_pos = vec![];
    let mut articles_ready_to_write_to_db = vec![];

    loop {
        if article_grouped_by_code_and_pos.is_empty() {
            // new grouping start - because first article ever
            let tmp = line.expect("should work").expect("should work 2");
            let prev = Article::from(tmp);
            article_grouped_by_code_and_pos.push(prev);
            println!("new group");
        } else {
            // is article part of current group?
            let previous_article = article_grouped_by_code_and_pos.last().unwrap();
            let current_line = line.expect("line should work").expect("line should work 2");

            println!(
                "previous_article  code '{}'  pos '{}'",
                previous_article.code(),
                previous_article.pos()
            );
            println!(
                "current_line      code '{}'  pos '{}'",
                current_line.code(),
                current_line.pos()
            );

            if previous_article.code() == current_line.code()
                && previous_article.pos() == current_line.pos()
            {
                let article = Article::from(current_line);
                article_grouped_by_code_and_pos.push(article);
                println!("article add to current group");
            } else {
                // article is not part of current group -> find cheapest
                println!("find cheapest");
                article_grouped_by_code_and_pos.sort_by(|a, b| a.price.total_cmp(&b.price));
                let cheapest = article_grouped_by_code_and_pos[0].clone();
                if return_items {
                    articles_ready_to_write_to_db.push(cheapest);
                }
                db_rows_written += 1;

                article_grouped_by_code_and_pos.clear();
                article_grouped_by_code_and_pos.push(Article::from(current_line));
            }
        }

        line = iter.next();
        if line.is_none() {
            break;
        }
        lines_processed += 1;
    }

    article_grouped_by_code_and_pos.sort_by(|a, b| a.price.total_cmp(&b.price));
    let cheapest = article_grouped_by_code_and_pos[0].clone();
    if return_items {
        articles_ready_to_write_to_db.push(cheapest);
    }
    db_rows_written += 1;

    let ir = if return_items {
        ImportResult {
            lines_processed,
            db_rows_written,
            items: articles_ready_to_write_to_db,
        }
    } else {
        ImportResult {
            lines_processed,
            db_rows_written,
            items: vec![],
        }
    };

    Ok(ir)
}
