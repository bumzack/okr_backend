use std::env;
use std::io::Error;

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use dotenvy::dotenv;
use log::{info, LevelFilter};
use pretty_env_logger::env_logger::Builder;
use tokio_postgres::{NoTls, Row};

lazy_static::lazy_static! {
    static ref POOL: Pool = create_pool( );
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    dotenv().expect(".env file not found");

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    let c = POOL.get().await.expect("should get client");
    let row = c.query("SELECT COUNT(*) FROM articles", &[]).await.expect("select count");

    row.iter().for_each(|r| {
        info!("row {:?}", r);
        info!("row {:?}", r.get::<&str, i64>("count"));
    });

    Ok(())
}


#[derive(Debug, Clone)]
struct NewArticleModel {
    code: String,
    title: String,
    description: String,
    categories: String,
    attributes: String,
    price: f64,
    start_date: String,
    end_date: String,
    pos: String,
}


#[derive(Debug, Clone)]
struct ArticleModel {
    id: String,
    code: String,
    title: String,
    description: String,
    categories: String,
    attributes: String,
    price: f64,
    start_date: String,
    end_date: String,
    pos: String,
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
        "INSERT INTO {}  (code, title, description) VALUES ($1, $2, $3) RETURNING *",
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
            ],
        )
        .await;

    let a = ArticleModel::from(&row.unwrap());
    Ok(a)
}

impl From<&Row> for ArticleModel {
    fn from(value: &Row) -> Self {
        ArticleModel {
            id: value.get("id"),
            code: value.get("code"),
            title: value.get("title"),
            description: value.get("description"),
            categories: value.get("categories"),
            attributes: value.get("attributes"),
            price: value.get("price"),
            start_date: value.get("start_date"),
            end_date: value.get("end_date"),
            pos: value.get("pos"),
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
