use deadpool_postgres::Pool;
use log::info;
use tokio_postgres::Error;

use crate::models::{ArticleModel, NewArticleModel, TABLE_ARTICLES};

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
