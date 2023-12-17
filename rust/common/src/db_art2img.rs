use deadpool_postgres::Pool;
use tokio_postgres::Error;

use crate::models::{Art2ImgModel, NewArt2ImgModel, TABLE_ART2IMG};

pub async fn insert_art2img(
    pool: &Pool,
    new_art2im: &NewArt2ImgModel,
) -> Result<Art2ImgModel, Error> {
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

    let art2img = Art2ImgModel::from(&row.unwrap());
    Ok(art2img)
}

pub async fn read_art2img(pool: &Pool, aritcle_id: i64) -> Result<Vec<Art2ImgModel>, Error> {
    let query = format!(
        "SELECT * FROM  {}   WHERE  article_id  = $1 ",
        TABLE_ART2IMG
    );

    let row = pool
        .get()
        .await
        .unwrap()
        .query(query.as_str(), &[&aritcle_id])
        .await
        .expect("should read a lot of art2img entries");

    let art2imgs: Vec<Art2ImgModel> = row.iter().map(Art2ImgModel::from).collect();

    Ok(art2imgs)
}
