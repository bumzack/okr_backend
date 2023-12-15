use deadpool_postgres::Pool;
use log::info;
use tokio_postgres::Error;

use crate::models::{Article, NewArticle, TABLE_ARTICLES};

pub async fn insert_article(pool: &Pool, new_article: &NewArticle) -> Result<Article, Error> {
    let query = format!(
        "INSERT INTO {}  (article_code, title, description) VALUES ($1, $2, $3) RETURNING *",
        TABLE_ARTICLES
    );

    let row = pool
        .get()
        .await
        .unwrap()
        .query_one(
            query.as_str(),
            &[
                &new_article.article_code,
                &new_article.title,
                &new_article.description,
            ],
        )
        .await;

    info!("returned  {:?}", row);
    let a = Article::from(&row.unwrap());
    info!("returned  article {:?}", a);

    Ok(a)
}

pub async fn read_articles(pool: &Pool) -> Result<Vec<Article>, Error> {
    let query = format!("SELECT * FROM  {} ", TABLE_ARTICLES);

    let rows = pool
        .get()
        .await
        .unwrap()
        .query(query.as_str(), &[])
        .await
        .expect("should read all articles");

    let articles: Vec<Article> = rows.iter().map(Article::from).collect();

    // info!("returned  {:?}", articles);

    Ok(articles)
}
