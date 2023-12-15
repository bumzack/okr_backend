use deadpool_postgres::Pool;
use log::info;
use tokio_postgres::Error;

use crate::models::{Art2Img, NewArt2Img, TABLE_ART2IMG};

pub async fn insert_art2img(pool: &Pool, new_art2im: &NewArt2Img) -> Result<Art2Img, Error> {
    let query = format!(
        "INSERT INTO {}  (article_id, image_id) VALUES ($1, $2) RETURNING * ",
        TABLE_ART2IMG
    );

    let row = pool
        .get()
        .await
        .unwrap()
        .query_one(
            query.as_str(),
            &[&new_art2im.article_id, &new_art2im.image_id],
        )
        .await;

    info!("returned  {:?}", row);
    let art2img = Art2Img::from(&row.unwrap());
    info!("returned  art2img {:?}", &art2img);

    Ok(art2img)
}

pub async fn read_art2img(pool: &Pool, aritcle_id: i32) -> Result<Vec<Art2Img>, Error> {
    let query = format!(
        "SELECT * FROM  {}   WHERE  article_id  = $1 ",
        TABLE_ART2IMG
    );

    let row = pool
        .get()
        .await
        .unwrap()
        .query(
            query.as_str(),
            &[&aritcle_id],
        )
        .await
        .expect("should read a lot of art2img entries");

    let art2imgs: Vec<Art2Img> = row.iter()
        .map(|art2img| Art2Img::from(art2img))
        .collect();

    Ok(art2imgs)
}