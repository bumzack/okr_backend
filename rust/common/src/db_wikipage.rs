use deadpool_postgres::Pool;
use tokio_postgres::Error;

use crate::models::{NewWikipageModel, TABLE_WIKIPAGE, WikipageModel};

pub async fn insert_wikipage(
    pool: &Pool,
    new_wiki_page: &NewWikipageModel,
) -> Result<WikipageModel, Error> {
    let query = format!(
        "INSERT INTO {}  (id, title, ns, redirect, timestamp, contributor, comment, model, format, text, sha1)  VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING *",
        TABLE_WIKIPAGE
    );

    let title = new_wiki_page
        .title
        .clone()
        .unwrap_or_else(|| "".to_string());
    let ns = new_wiki_page.ns.clone().unwrap_or_else(|| "".to_string());

    let redirect = new_wiki_page
        .redirect
        .clone()
        .unwrap_or_else(|| "".to_string())
        .clone();
    let timestamp = new_wiki_page.timestamp.clone();
    let contributor = new_wiki_page
        .contributor
        .clone()
        .unwrap_or_else(|| "".to_string());
    let comment = new_wiki_page
        .comment
        .clone()
        .unwrap_or_else(|| "".to_string());
    let model = new_wiki_page
        .model
        .clone()
        .unwrap_or_else(|| "".to_string());
    let format = new_wiki_page
        .format
        .clone()
        .unwrap_or_else(|| "".to_string());
    let txt = new_wiki_page.text.clone().unwrap_or_else(|| "".to_string());
    let sha1 = new_wiki_page.sha1.clone().unwrap_or_else(|| "".to_string());

    let row = pool
        .get()
        .await
        .unwrap()
        .query_one(
            query.as_str(),
            &[
                &new_wiki_page.id,
                &title,
                &ns,
                &redirect,
                &timestamp,
                &contributor,
                &comment,
                &model,
                &format,
                &txt,
                &sha1,
            ],
        )
        .await;

    let wiki_page = WikipageModel::from(&row.unwrap());

    Ok(wiki_page)
}

// pub async fn read_resolutions(pool: &Pool) -> Result<Vec<ResolutionModel>, Error> {
//     let query = format!("SELECT * FROM  {} ", TABLE_RESOLUTION);
//
//     let rows = pool
//         .get()
//         .await
//         .unwrap()
//         .query(query.as_str(), &[])
//         .await
//         .expect("should read all articles");
//
//     let resolutions: Vec<ResolutionModel> = rows.iter().map(ResolutionModel::from).collect();
//     Ok(resolutions)
// }
