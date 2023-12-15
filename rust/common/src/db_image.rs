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

    // info!("returned  {:?}", row);
    let img = Image::from(&row.unwrap());
    // info!("returned  img {:?}", &img);

    Ok(img)
}

pub async fn read_images(pool: &Pool, image_ids: &[i32]) -> Result<Vec<Image>, Error> {
    let mut params = vec![];
    image_ids.iter().for_each(|id| {
        params.push(format!("{}", id));
    });

    let id_list = params.join(", ");

    let query = format!(
        "SELECT * FROM  {}  WHERE id IN ( {} )  ",
        TABLE_IMAGES, id_list
    );
    // let query = format!("SELECT * FROM  {}  WHERE id = 1  ", TABLE_IMAGES);
    println!("query   {}", query);

    let row = pool
        .get()
        .await
        .unwrap()
        .query(query.as_str(), &[])
        .await
        .expect("should read a lot of image entries");

    println!("================================================================================================");
    println!("================================================================================================");
    row.iter().for_each(|r| println!("row   {:?}", r));
    println!("================================================================================================");
    println!("================================================================================================");

    let images: Vec<Image> = row.iter().map(Image::from).collect();
    println!("================================================================================================");
    println!("================================================================================================");
    images
        .iter()
        .for_each(|img| println!("img code    {:?}, img filename {}", img.id, img.filename));
    println!("================================================================================================");
    println!("================================================================================================");

    Ok(images)
}
