use tokio_postgres::Row;

pub const TABLE_ARTICLES: &'static str = "articles";
pub const TABLE_IMAGES: &'static str = "images";
pub const TABLE_ART2IMG: &'static str = "art2img";

#[derive(Debug)]
pub struct Article {
    pub id: i32,
    article_code: String,
    title: String,
    description: String,
}

#[derive(Debug)]
pub struct NewArticle {
    pub article_code: String,
    pub title: String,
    pub description: String,
}

impl From<&Row> for Article {
    fn from(value: &Row) -> Self {
        Article {
            id: value.get("id"),
            article_code: value.get("article_code"),
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
    pub id: i32,
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
    pub article_id: i32,
    pub image_id: i32,
}

#[derive(Debug)]
pub struct Art2Img {
    pub id: i32,
    pub article_id: i32,
    pub image_id: i32,
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
