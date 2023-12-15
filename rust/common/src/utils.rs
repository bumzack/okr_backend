use deadpool_postgres::Pool;
use deadpool_postgres::{Manager, ManagerConfig, RecyclingMethod};
use log::info;
use tokio_postgres::{Client, Error, NoTls};

use crate::models::{Art2Img, Article, Image, Resolution};

pub async fn dump_tables(id: String) -> Result<(), Error> {
    let client = create_connection(id).await?;

    let articles = client.query("SELECT * FROM articles", &[]).await?;
    articles.iter().for_each(|a| {
        println!("article {:?}", Article::from(a));
    });

    let images = client.query("SELECT * FROM images", &[]).await?;
    images.iter().for_each(|a| {
        let image = Image::from(a);
        println!("image    id {:?}, filename {}", image.id, image.filename);
    });

    let art2img = client.query("SELECT * FROM art2img", &[]).await?;
    art2img.iter().for_each(|a| {
        println!("art2img {:?}", Art2Img::from(a));
    });

    let resolutions = client.query("SELECT * FROM resolutions", &[]).await?;
    resolutions.iter().for_each(|a| {
        println!("resolutions {:?}", Resolution::from(a));
    });

    Ok(())
}

pub fn create_pool(id: String) -> Pool {
    let mut pg_config = tokio_postgres::Config::new();

    let user: String = id.clone();
    let password: String = id.clone();
    let host: String = "localhost".into();
    let dbname: String = id.clone();
    let port: u16 = 54321;

    info!("user {user}, password {password}, host {host}, dbname {dbname}");
    pg_config.user(&user);
    pg_config.password(&password);
    pg_config.host(&host);
    pg_config.dbname(&dbname);
    pg_config.port(port);
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    Pool::builder(mgr).max_size(16).build().unwrap()
}

pub fn get_db_config(id: String) -> deadpool_postgres::Config {
    let mut config = deadpool_postgres::Config::new();
    config.user = Some(id.clone());
    config.password = Some(id.clone());
    config.dbname = Some(id.clone());
    config.host = Some("localhost".into());
    config.port = Some(54321);

    config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    config
}

pub async fn create_connection(id: String) -> Result<Client, Error> {
    let config = format!(
        "host=localhost hostaddr=127.0.0.1 user={} password={} port=54321 dbname={}",
        id.clone(),
        id.clone(),
        id.clone()
    );

    let (client, connection) = tokio_postgres::connect(&config, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}
