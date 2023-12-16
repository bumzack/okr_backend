use tokio_postgres::Row;

pub const TABLE_ARTICLES: &str = "articles";
pub const TABLE_IMAGES: &str = "images";
pub const TABLE_ART2IMG: &str = "art2img";
pub const TABLE_RESOLUTION: &str = "resolutions";

#[derive(Debug)]
pub struct Article {
    pub id: i64,
    pub code: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug)]
pub struct NewArticle {
    pub code: String,
    pub title: String,
    pub description: String,
}

impl From<&Row> for Article {
    fn from(value: &Row) -> Self {
        Article {
            id: value.get("id"),
            code: value.get("code"),
            title: value.get("title"),
            description: value.get("description"),
        }
    }
}

#[derive(Debug)]
pub struct NewImage {
    pub filename: String,
    pub img_data: String,
}

#[derive(Debug)]
pub struct Image {
    pub id: i64,
    pub filename: String,
    pub img_data: String,
}

impl From<&Row> for Image {
    fn from(value: &Row) -> Self {
        Image {
            id: value.get("id"),
            filename: value.get("filename"),
            img_data: value.get("image"),
        }
    }
}

#[derive(Debug)]
pub struct NewArt2Img {
    pub article_id: i64,
    pub image_id: i64,
}

#[derive(Debug)]
pub struct Art2Img {
    pub id: i64,
    pub article_id: i64,
    pub image_id: i64,
}

impl From<&Row> for Art2Img {
    fn from(value: &Row) -> Self {
        Art2Img {
            id: value.get("id"),
            article_id: value.get("article_id"),
            image_id: value.get("image_id"),
        }
    }
}

#[derive(Debug)]
pub struct NewResolution {
    pub resolution: String,
}

#[derive(Debug)]
pub struct Resolution {
    pub id: i64,
    pub resolution: String,
}

impl From<&Row> for Resolution {
    fn from(value: &Row) -> Self {
        Resolution {
            id: value.get("id"),
            resolution: value.get("resolution"),
        }
    }
}
