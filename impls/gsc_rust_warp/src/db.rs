use std::env;
use std::io::Error;

use chrono::{DateTime, Utc};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use log::info;
use tokio_postgres::{NoTls, Row};

#[derive(Debug)]
pub struct ImportResult {
    pub lines_processed: usize,
    pub db_rows_written: usize,
}

#[derive(Debug, Clone)]
pub struct NewArticleModel {
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


#[derive(Debug, Clone)]
pub struct NewArticleModelRefsOnly<'a> {
    pub code: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub categories: &'a str,
    pub attributes: &'a str,
    pub price: f64,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub pos: &'a str,
}

#[derive(Debug, Clone)]
pub struct ArticleModel {
    pub id: i64,
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


pub const TABLE_ARTICLES: &str = "articles";

pub async fn read_articles(pool: &Pool) -> Result<Vec<ArticleModel>, Error> {
    let query = format!("SELECT * FROM  {}  ORDER BY code ASC ", TABLE_ARTICLES);

    let rows = pool
        .get()
        .await
        .unwrap()
        .query(query.as_str(), &[])
        .await
        .expect("should read all articles");

    let articles: Vec<ArticleModel> = rows.iter().map(ArticleModel::from).collect();

    Ok(articles)
}

pub async fn read_articles_paginated(
    pool: &Pool,
    page_number: u32,
    page_size: u32,
) -> Result<Vec<ArticleModel>, Error> {
    let offset = (page_number) * page_size;
    let query = format!(
        "SELECT * FROM  {}  ORDER BY CODE ASC LIMIT {} OFFSET {}",
        TABLE_ARTICLES, page_size, offset
    );
    info!("query  {}", &query);

    let rows = pool
        .get()
        .await
        .unwrap()
        .query(query.as_str(), &[])
        .await
        .expect("should read all articles");

    let articles: Vec<ArticleModel> = rows.iter().map(ArticleModel::from).collect();
    info!("{} articles from  query  {}", articles.len(), &query);

    Ok(articles)
}

pub async fn insert_article(
    pool: &Pool,
    new_article: &NewArticleModel,
) -> Result<ArticleModel, Error> {
    let query = format!(
        "INSERT INTO {}  (code, title, description, attributes, categories, pos, price, start_date, end_date)  \
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
        TABLE_ARTICLES
    );

    let row = pool
        .get()
        .await
        .unwrap()
        .query_one(
            query.as_str(),
            &[
                &new_article.code,
                &new_article.title,
                &new_article.description,
                &new_article.attributes,
                &new_article.categories,
                &new_article.pos,
                &new_article.price,
                &new_article.start_date,
                &new_article.end_date,
            ],
        )
        .await;

    let a = ArticleModel::from(&row.unwrap());
    Ok(a)
}

impl From<&Row> for ArticleModel {
    fn from(value: &Row) -> Self {
        ArticleModel {
            id: value.get::<&str, i64>("id"),
            code: value.get::<&str, String>("code"),
            title: value.get::<&str, String>("title"),
            description: value.get::<&str, String>("description"),
            categories: value.get::<&str, String>("categories"),
            attributes: value.get::<&str, String>("attributes"),
            price: value.get::<&str, f64>("price"),
            start_date: value.get::<&str, DateTime<Utc>>("start_date"),
            end_date: value.get::<&str, DateTime<Utc>>("end_date"),
            pos: value.get::<&str, String>("pos"),
        }
    }
}


pub fn create_pool() -> Pool {
    let mut pg_config = tokio_postgres::Config::new();

    let user: String = env::var("DBUSER").expect("DBUSER");
    let password: String = env::var("DBPASSWORD").expect("DBPASSWORD");
    let host: String = env::var("DBHOST").expect("DBUHOST");
    let dbname: String = env::var("DBNAME").expect("DBNAME");
    let port: String = env::var("DBPORT").expect("DBPORT");


    info!("user {user}, password {password}, host {host}, dbname {dbname}");
    pg_config.user(&user);
    pg_config.password(&password);
    pg_config.host(&host);
    pg_config.dbname(&dbname);
    pg_config.port(port.parse().unwrap());
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    Pool::builder(mgr).max_size(16).build().unwrap()
}
