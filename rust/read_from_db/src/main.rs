use std::fs;
use std::io::Write;

use base64::Engine;
use base64::engine::general_purpose;
use log::LevelFilter;
use pretty_env_logger::env_logger::Builder;
use tokio_postgres::Error;

use common::db_art2img::read_art2img;
use common::db_articles::read_articles;
use common::db_image::read_images;
use common::utils::create_pool;

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let path = env!("CARGO_MANIFEST_DIR");
    let pool = create_pool("dev".into());
    let articles = read_articles(&pool).await?;

    for a in articles.iter() {
        println!("article {:?} ", a);

        let art2imgs = read_art2img(&pool, a.id).await?;

        for art2img in &art2imgs {
            println!("found art2img   {:?}", art2img);
        }

        let img_ids: Vec<i32> = art2imgs.iter().map(|art2img| art2img.image_id).collect();

        let images = read_images(&pool, &img_ids).await?;

        for img in &images {
            let decoded: Vec<u8> = general_purpose::STANDARD_NO_PAD
                .decode(&img.img_data)
                .expect("decoding should work");

            let filename = format!("{}/images/{}", path, img.filename);
            println!("writing img               {}", filename);
            let mut file = fs::OpenOptions::new()
                .create(true) // To create a new file
                .write(true)
                // either use the ? operator or unwrap since it returns a Result
                .open(filename)
                .expect("should be able to open a file to write");

            file.write_all(&decoded)
                .expect("should be able to write all bytes to the file");
        }
    }

    Ok(())
}
