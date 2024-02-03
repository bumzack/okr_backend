use std::ffi::OsString;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::sync::{mpsc, Arc, Mutex};
use std::{fs, thread};

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::models::{ImportRequest, ImportResult, Sysinfo};

pub async fn import_articles_v3(Json(input): Json<ImportRequest>) -> impl IntoResponse {
    println!("request  {:?}", input);
    let res = ImportResult::default();
    let files = read_files().expect("should read files");

    let (sender, receiver) = mpsc::channel::<Vec<String>>();
    let receiver = Arc::new(Mutex::new(receiver));
    let res = Arc::new(Mutex::new(res));
    let cores = 4;
    let mut worker_threads = vec![];
    let lines_per_group = 1_000;
    let mut group_cnt = 0;

    for _ in 0..cores {
        let res = res.clone();
        let receiver = receiver.clone();
        let mut groups_processed = 0;
        let t = thread::spawn(move || {
            let mut lines;
            loop {
                lines = receiver.lock().unwrap().recv();
                match lines {
                    Ok(lines) => {
                        groups_processed += 1;
                        let cnt = lines.len();
                        let res_file = process_file_v3(lines, input.return_items)
                            .expect("should processs a Vec<String>");
                        println!(
                            "thread {:?}   lines cnt processed  {:?}   res  {:?}",
                            thread::current().id(),
                            cnt,
                            res_file
                        );
                        {
                            let mut res = res.lock().unwrap();
                            res.lines_processed += res_file.lines_processed;
                            res.db_rows_written += 1 //  res_file.db_rows_written;
                        }
                    }
                    Err(e) => {
                        println!("all work is done {:?} ", e);
                        break;
                    }
                }
            }
            (thread::current().id(), groups_processed)
        });
        worker_threads.push(t);
    }

    let producer_thread = thread::spawn(move || {
        for f in files {
            let filename = format!(
                "{}/{}",
                "/home/bumzack/stoff/okr_backend/data",
                f.to_str().expect("should be a filename")
            );
            println!("file   {}", &filename);
            let file = File::open(filename).expect("file open");
            let reader = BufReader::new(file);

            let mut iter = reader.lines();

            let mut lines_read = 0;
            let mut line = iter.next();
            while line.is_some() {
                let mut group: Vec<String> = vec![];
                while line.is_some() && lines_read < lines_per_group {
                    let l = line.as_ref().unwrap().as_ref().unwrap().clone();
                    group.push(l);
                    lines_read += 1;
                    line = iter.next();
                }
                // println!(
                //     "lines read  {}, group size {},   groups  {}",
                //     lines_read,
                //     group.len(),
                //     group_cnt
                // );
                lines_read = 0;

                group_cnt += 1;
                sender.send(group).expect("sending should work");
            }
        }

        println!("producer thread produced {} groups", group_cnt);
        group_cnt
    });

    println!("joining worker threads");

    for t in worker_threads {
        let th = t.join();
        match th {
            Ok((id, groups_processed)) => {
                println!("thread {:?} processed {} group", id, groups_processed)
            }
            Err(e) => {
                println!("thread crashed   {:?}", e);
            }
        }
    }

    println!("join producer thread");

    match producer_thread.join() {
        Ok(res) => {
            println!("producer thread produced  {:?} groups", res)
        }
        Err(e) => {
            println!("producer thread crashed   {:?}", e);
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

pub async fn sysinfo_v3() -> impl IntoResponse {
    let si = Sysinfo {
        author: "gsc".to_string(),
        comment: "work steal - group lines and pass to task".to_string(),
        framework: "axum".to_string(),
        language: "rust".to_string(),
        version: "v3".to_string(),
        multithreaded: true,
    };
    (StatusCode::OK, Json(si))
}

fn read_files() -> Result<Vec<OsString>, Error> {
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

fn process_file_v3(lines: Vec<String>, return_items: bool) -> Result<ImportResult, Error> {
    let mut lines_processed = 0;
    let mut db_rows_written = 0;

    // let mut line: Option<String> = lines.pop_front();
    // lines_processed += 1;
    //
    // let mut article_grouped_by_code_and_pos = vec![];
    // let mut articles_ready_to_write_to_db = vec![];
    //
    // loop {
    //     if article_grouped_by_code_and_pos.is_empty() {
    //         // new grouping start - because first article ever
    //         let tmp = line.expect("should work");
    //
    //         // FIXME to_string
    //         let prev = Article::from(tmp.to_string());
    //         article_grouped_by_code_and_pos.push(prev);
    //     } else {
    //         // is article part of current group?
    //         let previous_article = article_grouped_by_code_and_pos.last().unwrap();
    //         let current_line = line.expect("line should work");
    //
    //         if previous_article.code() == current_line.code()
    //             && previous_article.pos() == current_line.pos()
    //         {
    //             // FIXME to_string
    //             let article = Article::from(current_line.to_string());
    //             article_grouped_by_code_and_pos.push(article);
    //         } else {
    //             // article is not part of current group -> find cheapest
    //             article_grouped_by_code_and_pos.sort_by(|a, b| a.price.total_cmp(&b.price));
    //             let cheapest = article_grouped_by_code_and_pos[0].clone();
    //             if return_items {
    //                 articles_ready_to_write_to_db.push(cheapest);
    //             }
    //             db_rows_written += 1;
    //
    //             article_grouped_by_code_and_pos.clear();
    //             // FIXME to_string
    //             article_grouped_by_code_and_pos.push(Article::from(current_line.to_string()));
    //         }
    //     }
    //
    //     line = iter.next();
    //     if line.is_none() {
    //         break;
    //     }
    //     lines_processed += 1;
    // }
    //
    // article_grouped_by_code_and_pos.sort_by(|a, b| a.price.total_cmp(&b.price));
    // let cheapest = article_grouped_by_code_and_pos[0].clone();
    // if return_items {
    //     articles_ready_to_write_to_db.push(cheapest);
    // }
    // db_rows_written += 1;

    let ir = if return_items {
        ImportResult {
            lines_processed,
            db_rows_written,
            items: vec![], // articles_ready_to_write_to_db,
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
