use deadpool_postgres::Pool;
use tokio_postgres::Error;

use crate::models::{Image, NewImage, TABLE_IMAGES};

pub async fn insert_image(pool: &Pool, new_image: &NewImage) -> Result<Image, Error> {
    let query = format!(
        "INSERT INTO {}  (filename, image) VALUES ($1, $2) RETURNING * ",
        TABLE_IMAGES
    );

    let row = pool
        .get()
        .await
        .unwrap()
        .query_one(query.as_str(), &[&new_image.filename, &new_image.img_data])
        .await;

    let img = Image::from(&row.unwrap());
    Ok(img)
}

pub async fn read_images(pool: &Pool, image_ids: &[i64]) -> Result<Vec<Image>, Error> {
    let mut params = vec![];
    image_ids.iter().for_each(|id| {
        params.push(format!("{}", id));
    });

    let id_list = params.join(", ");

    // boy oh boy :-(
    let query = format!(
        "SELECT * FROM  {}  WHERE id IN ( {} )  ",
        TABLE_IMAGES, id_list
    );

    let row = pool
        .get()
        .await
        .unwrap()
        .query(query.as_str(), &[])
        .await
        .expect("should read a lot of image entries");

    let images: Vec<Image> = row.iter().map(Image::from).collect();
    Ok(images)
}
