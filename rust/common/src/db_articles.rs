use deadpool_postgres::Pool;
use tokio_postgres::Error;

use crate::models::{Article, NewArticle, TABLE_ARTICLES};

pub async fn insert_article(pool: &Pool, new_article: &NewArticle) -> Result<Article, Error> {
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

    let a = Article::from(&row.unwrap());
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

    Ok(articles)
}
