use tokio_postgres::{Error, NoTls};

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    let client = create_connection().await?;

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query("SELECT $1::TEXT", &[&"hello world"])
        .await?;

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    println!("val {}", &value);


    let articles = client
        .query("SELECT * FROM articles", &[])
        .await?;

    println!("articles {:?}", &articles);

    let images = client
        .query("SELECT * FROM images", &[])
        .await?;

    println!("images {:?}", &images);


    let art2img = client
        .query("SELECT * FROM art2img", &[])
        .await?;

    println!("art2img {:?}", &art2img);


    let pool: Pool = create_pool()?;




    Ok(())
}

async fn get_connection(pool: &Pool) -> Result<Client, String> {
    pool.get().await.map_err(|err| err.to_string())
}

pub async fn insert_article(pool: &Pool) -> Result<(), String> {
    // Stuff related to kafka and the tokio runtime
    pool.get_connection()
    
    Ok(())
}

fn insert_article(pool: &Pool) {
    let title = "title".to_string();
    let desc = "supa dupa description".to_string();

    get_connection(pool).await?
    .execute(
        "INSERT INTO article (title, description) VALUES ($1, $2)", 
        &[&title, &desc]]
    )
    .await
    .map(|_| ())
    .map_err(|err| format!("Error while insertion: {}", err))

}


pub fn create_pool() -> Result<Pool, String> {
    Ok(get_db_config().create_pool(NoTls).map_err(|err| err.to_string())?)
}


fn get_db_config() -> deadpool_postgres::Config {
    let mut config = deadpool_postgres::Config::new();
    config.user = Some( "dummy");
    config.password =  "dummy".into();
    config.dbname = "DB_NAME", "dummy_dev".into();
    config.host = "localhost".into();
    config.port = "54321".into();

    config.manager =
       Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    config
}


async fn create_connection() -> Result<_, _> {
// Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost hostaddr=127.0.0.1 user=dummy password=dummy port=54321 dbname=dummy_dev", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}