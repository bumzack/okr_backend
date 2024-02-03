use std::ffi::OsString;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::{fs, thread};

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::models::{Article, ImportRequest, ImportResult, Sysinfo};
use crate::stuff::{Code, Pos};

pub async fn import_articles_v2(Json(input): Json<ImportRequest>) -> impl IntoResponse {
    println!("request  {:?}", input);
    let res = ImportResult::default();
    let files = read_files().await.expect("should read files");
    let files = Arc::new(Mutex::new(files));
    let res = Arc::new(Mutex::new(res));
    let cores = 16;

    let mut worker_threads = vec![];
    for _ in 0..cores {
        let files = files.clone();
        let res = res.clone();

        let mut files_processed = 0;
        let t = thread::spawn(move || {
            let mut filename;
            while files.lock().unwrap().len() > 0 {
                {
                    filename = files.lock().unwrap().pop();
                    println!(
                        "thread {:?}  processing file  {:?}",
                        thread::current().id(),
                        &filename
                    );
                }
                if filename.is_some() {
                    files_processed += 1;

                    let res_file = process_file_v2(&filename.as_ref().unwrap(), input.return_items)
                        .expect("should processs a file");
                    println!(
                        "res filename  {:?}   res  {:?}",
                        &filename.as_ref(),
                        res_file
                    );
                    {
                        let mut res = res.lock().unwrap();
                        res.lines_processed += res_file.lines_processed;
                        res.db_rows_written += res_file.db_rows_written;
                    }
                }
            }
            (thread::current().id(), files_processed)
        });
        worker_threads.push(t);
    }

    for t in worker_threads {
        let th = t.join();
        match th {
            Ok((id, files_processed)) => {
                println!("thread {:?} processed {} files", id, files_processed)
            }
            Err(e) => {
                println!("thread crashed   {:?}", e);
            }
        }
    }

    let res1 = Arc::try_unwrap(res).unwrap();
    let res1 = res1.into_inner().unwrap();

    println!(
        "res1   db_rows_written {},   lines_processed {} ",
        res1.db_rows_written, res1.lines_processed
    );
    (StatusCode::OK, Json(res1))
}

pub async fn sysinfo_v2() -> impl IntoResponse {
    let si = Sysinfo {
        author: "gsc".to_string(),
        comment: "work steal - 1 file per thread".to_string(),
        framework: "axum".to_string(),
        language: "rust".to_string(),
        version: "v2".to_string(),
        multithreaded: true,
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

fn process_file_v2(f: &OsString, return_items: bool) -> Result<ImportResult, Error> {
    let filename = format!(
        "{}/{}",
        "/home/bumzack/stoff/okr_backend/data",
        f.to_str().expect("should be a filename")
    );
    //println!("filename  {}   return_items {}", &filename, return_items);

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
        } else {
            // is article part of current group?
            let previous_article = article_grouped_by_code_and_pos.last().unwrap();
            let current_line = line.expect("line should work").expect("line should work 2");

            if previous_article.code() == current_line.code()
                && previous_article.pos() == current_line.pos()
            {
                let article = Article::from(current_line);
                article_grouped_by_code_and_pos.push(article);
            } else {
                // article is not part of current group -> find cheapest
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
    println!(
        "import result   lines_processed   {:?}   db_rows_written   {}",
        &ir.lines_processed, &ir.db_rows_written
    );

    Ok(ir)
}
