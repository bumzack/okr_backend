use std::{env, fs, io};
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead, Error};

use chrono::{DateTime, Utc};
use dotenvy::dotenv;
use log::{info, LevelFilter};
use pretty_env_logger::env_logger::Builder;

const LEN_CODE: usize = 20;
const LEN_TITLE: usize = 100;
const LEN_DESC: usize = 1700;
const LEN_ATTRIBUTES: usize = 200;
const LEN_CATEGORIES: usize = 200;
const LEN_POS: usize = 30;
const LEN_PRICE: usize = 20;
const LEN_START_DATE: usize = 25;
const LEN_END_DATE: usize = 25;



#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    dotenv().expect(".env file not found");

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    let import_result = import_articles().await.expect("import should succeed");

    info!("import_result {:?},  {}  {}", &import_result.lines_processed, import_result.db_rows_written, import_result.items.len());

    Ok(())
}

async fn import_articles() -> Result<ImportResult, Error> {
    let data_dir = env::var("DATA_DIR").expect("DATA_DIR");
    let paths = fs::read_dir(&data_dir).unwrap();
    let mut files: Vec<Result<fs::DirEntry, Error>> = paths.into_iter()
        .filter(|f| {
            let string = f.as_ref().expect("is a file").file_name().to_ascii_lowercase();
            let f = string.to_str().expect("to str");
            f.ends_with(".txt")
        })
        .collect();

    files.sort_by(|a, b| a.as_ref().unwrap().file_name().partial_cmp(&b.as_ref().unwrap().file_name()).unwrap());
    files.iter()
        .for_each(|f| {
            let n = f.as_ref().expect("is a file").file_name();
            info!("file name {:?}", n);
        });

    let mut res = vec![];
    for f in files {
        let n = f.as_ref().expect("is a file").file_name();
        info!("processing file name {:?}", n);
        let ir = process_file(&n, &data_dir).await;
        //  info!("import result {:?} for file {:?}", &ir, &n);
        res.push(ir);
    }

    let mut lines_processed = 0;
    let mut db_rows_written = 0;
    let mut articles = vec![];
    res.iter_mut()
        .for_each(|ir| {
            lines_processed += ir.lines_processed;
            db_rows_written += ir.db_rows_written;
            articles.append(&mut ir.items)
        });

    let ir = ImportResult {
        lines_processed,
        db_rows_written,
        items: articles,
    };

    Ok(ir)
}

async fn process_file(file_name: &OsString, data_dir: &String) -> ImportResult {
    let exp_msg = format!("file open of file '{}' should work", file_name.to_str().expect("filename"));
    let f = format!("{}/{}", data_dir, file_name.to_str().expect("sould do it"));
    let f = File::open(f).expect(&exp_msg);
    let lines = io::BufReader::new(f).lines();

    let mut articles: Vec<ArticleModel> = vec![];
    let mut current_article: Vec<ArticleModel> = vec![];

    let mut db_rows_written = 0;
    let mut lines_processed = 0;

    for line in lines {
        let article = convert_to_new_article_model(line.expect("line should be a string"));

        match current_article.last() {
            Some(last) => {
                if (last.code == article.code) && (last.pos == article.pos) {
                    current_article.push(article);
                } else {
                    current_article
                        .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
                    articles.push(current_article.first().expect("at least 1 article should be in the file").clone());
                    db_rows_written += 1;
                    current_article.clear();
                }
            }
            None => {
                articles.push(article);
                db_rows_written += 1;
            }
        }

        // if articles.len() > 50 {
        //     // for a in &articles {
        //     //     let _res = insert_article(&POOL, a).await.expect("insert should work");
        //     // };
        //     db_rows_written += articles.len();
        //     articles.clear();
        // }
        lines_processed += 1;
    }

    ImportResult {
        lines_processed,
        db_rows_written,
        items: articles,
    }
}

fn convert_to_new_article_model(line: String) -> ArticleModel {
    let start_title = LEN_CODE;
    let start_desc = start_title + LEN_TITLE;
    let start_attr = start_desc + LEN_DESC;
    let start_cat = start_attr + LEN_ATTRIBUTES;
    let start_pos = start_cat + LEN_CATEGORIES;
    let start_price = start_pos + LEN_POS;
    let start_start_date = start_price + LEN_PRICE;
    let start_end_date = start_start_date + LEN_START_DATE;
    let end_end_date = start_end_date + LEN_END_DATE;

    let l = line.as_str();
    let code = &l[0..LEN_CODE];
    let title = &l[start_title..start_desc - 1];
    let desc = &l[start_desc..start_attr - 1];
    let attr = &l[start_attr..start_cat - 1];
    let cat = &l[start_cat..start_pos - 1];
    let pos = &l[start_pos..start_price - 1];
    let price = l[start_price..start_start_date].parse::<f64>().expect("parsing price");
    let start_date = l[start_start_date..start_end_date].parse::<i64>().expect("parsing start date");
    let end_date = l[start_end_date..end_end_date].parse::<i64>().expect("parsing end date");
    ArticleModel {
        code: code.to_string().trim_start_matches("0").to_string(),
        title: title.to_string().trim().to_string(),
        description: desc.to_string().trim().to_string(),
        categories: cat.to_string().trim().to_string(),
        attributes: attr.to_string().trim().to_string(),
        price,
        start_date: DateTime::<Utc>::from_timestamp(start_date, 0).expect("invalid timestamp starte date"),
        end_date: DateTime::<Utc>::from_timestamp(end_date, 0).expect("invalid timestamp end date"),
        pos: pos.to_string().trim().trim_start_matches("0").to_string(),
    }
}

//
// fn convert_to_new_article_model_refs_only<'a>(line: String) -> NewArticleModelRefsOnly <'a>{
//     let start_title = LEN_CODE;
//     let start_desc = start_title + LEN_TITLE;
//     let start_attr = start_desc + LEN_DESC;
//     let start_cat = start_attr + LEN_ATTRIBUTES;
//     let start_pos = start_cat + LEN_CATEGORIES;
//     let start_price = start_pos + LEN_POS;
//     let start_start_date = start_price + LEN_PRICE;
//     let start_end_date = start_start_date + LEN_START_DATE;
//     let end_end_date = start_end_date + LEN_END_DATE;
//
//     let l = line.as_str();
//     let code = &l[0..LEN_CODE];
//     let title = &l[start_title..start_desc - 1];
//     let desc = &l[start_desc..start_attr - 1];
//     let attr = &l[start_attr..start_cat - 1];
//     let cat = &l[start_cat..start_pos - 1];
//     let pos = &l[start_pos..start_price - 1];
//     let price = l[start_price..start_start_date].parse::<f64>().expect("parsing price");
//     let start_date = l[start_start_date..start_end_date].parse::<i64>().expect("parsing start date");
//     let end_date = l[start_end_date..end_end_date].parse::<i64>().expect("parsing end date");
//     NewArticleModelRefsOnly {
//         code: code.to_string().trim_start_matches("0"),
//         title: title.to_string().trim(),
//         description: desc.to_string().trim(),
//         categories: cat.to_string().trim(),
//         attributes: attr.to_string().trim(),
//         price,
//         start_date: DateTime::<Utc>::from_timestamp(start_date, 0).expect("invalid timestamp starte date"),
//         end_date: DateTime::<Utc>::from_timestamp(end_date, 0).expect("invalid timestamp end date"),
//         pos: pos.clone().to_string().trim().trim_start_matches("0"),
//     }
// }
//


#[derive(Debug)]
pub struct ImportResult {
    pub lines_processed: usize,
    pub db_rows_written: usize,
    pub items: Vec<ArticleModel>,
}

#[derive(Debug, Clone)]
pub struct ArticleModel {
    pub code: String,
    pub title: String,
    pub description: String,
    pub categories: String,
    pub attributes: String,
    pub price: f64,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub pos: String,
}


