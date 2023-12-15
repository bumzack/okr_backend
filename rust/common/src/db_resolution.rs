use deadpool_postgres::Pool;
use log::info;
use tokio_postgres::Error;

use crate::models::{NewResolution, Resolution, TABLE_RESOLUTION};

pub async fn insert_resolution(
    pool: &Pool,
    new_resolution: &NewResolution,
) -> Result<Resolution, Error> {
    let query = format!(
        "INSERT INTO {}  (resolution) VALUES ($1) RETURNING *",
        TABLE_RESOLUTION
    );

    let row = pool
        .get()
        .await
        .unwrap()
        .query_one(query.as_str(), &[&new_resolution.resolution])
        .await;

    let a = Resolution::from(&row.unwrap());
    info!("returned  resolution {:?}", a);

    Ok(a)
}

pub async fn read_articles(pool: &Pool) -> Result<Vec<Resolution>, Error> {
    let query = format!("SELECT * FROM  {} ", TABLE_RESOLUTION);

    let rows = pool
        .get()
        .await
        .unwrap()
        .query(query.as_str(), &[])
        .await
        .expect("should read all articles");

    let resolutions: Vec<Resolution> = rows.iter().map(Resolution::from).collect();
    Ok(resolutions)
}
