use tokio_postgres::Row;

pub const TABLE_ARTICLES: &str = "articles";
pub const TABLE_IMAGES: &str = "images";
pub const TABLE_ART2IMG: &str = "art2img";
pub const TABLE_RESOLUTION: &str = "resolutions";

#[derive(Debug, Clone)]
pub struct ArticleModel {
    pub id: i64,
    pub code: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug)]
pub struct NewArticleModel {
    pub code: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug)]
pub struct NewImageModel {
    pub filename: String,
    pub img_data: String,
}

#[derive(Debug)]
pub struct ImageModel {
    pub id: i64,
    pub filename: String,
    pub img_data: String,
}

#[derive(Debug)]
pub struct NewArt2ImgModel {
    pub article_id: i64,
    pub image_id: i64,
}

#[derive(Debug)]
pub struct Art2ImgModel {
    pub id: i64,
    pub article_id: i64,
    pub image_id: i64,
}

#[derive(Debug)]
pub struct NewResolutionModel {
    pub resolution: String,
}

#[derive(Debug)]
pub struct ResolutionModel {
    pub id: i64,
    pub resolution: String,
}

impl From<&Row> for ArticleModel {
    fn from(value: &Row) -> Self {
        ArticleModel {
            id: value.get("id"),
            code: value.get("code"),
            title: value.get("title"),
            description: value.get("description"),
        }
    }
}

impl From<&Row> for ImageModel {
    fn from(value: &Row) -> Self {
        ImageModel {
            id: value.get("id"),
            filename: value.get("filename"),
            img_data: value.get("image"),
        }
    }
}

impl From<&Row> for Art2ImgModel {
    fn from(value: &Row) -> Self {
        Art2ImgModel {
            id: value.get("id"),
            article_id: value.get("article_id"),
            image_id: value.get("image_id"),
        }
    }
}

impl From<&Row> for ResolutionModel {
    fn from(value: &Row) -> Self {
        ResolutionModel {
            id: value.get("id"),
            resolution: value.get("resolution"),
        }
    }
}
