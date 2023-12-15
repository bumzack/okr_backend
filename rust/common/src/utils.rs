use deadpool_postgres::Pool;
use deadpool_postgres::{Manager, ManagerConfig, RecyclingMethod};
use log::info;
use tokio_postgres::{Client, Error, NoTls};

use crate::models::{Art2Img, Article, Image};

pub async fn dump_tables() -> Result<(), Error> {
    let client = create_connection().await?;

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    println!("val {}", &value);

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
    Ok(())
}

pub fn create_pool() -> Pool {
    let mut pg_config = tokio_postgres::Config::new();

    let user: String = "dev".into();
    let password: String = "dev".into();
    let host: String = "localhost".into();
    let dbname: String = "dev".into();
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

pub fn get_db_config() -> deadpool_postgres::Config {
    let mut config = deadpool_postgres::Config::new();
    config.user = Some("dev".into());
    config.password = Some("dev".into());
    config.dbname = Some("dev".into());
    config.host = Some("localhost".into());
    config.port = Some(54321);

    config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    config
}

pub async fn create_connection() -> Result<Client, Error> {
    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(
        "host=localhost hostaddr=127.0.0.1 user=dev password=dev port=54321 dbname=dev",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}
