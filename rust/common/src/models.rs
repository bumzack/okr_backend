use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

pub const TABLE_ARTICLES: &str = "articles";
pub const TABLE_IMAGES: &str = "images";
pub const TABLE_ART2IMG: &str = "art2img";
pub const TABLE_RESOLUTION: &str = "resolutions";
pub const TABLE_WIKIPAGE: &str = "wiki_page";

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
    pub image_as_rgb_png: String,
    pub image_as_json_pixels_array: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct ImageModel {
    pub id: i64,
    pub filename: String,
    pub image_as_rgb_png: String,
    pub image_as_json_pixels_array: String,
    pub width: u32,
    pub height: u32,
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
        let w: i32 = value.get("width");
        let h: i32 = value.get("height");
        ImageModel {
            id: value.get("id"),
            filename: value.get("filename"),
            image_as_json_pixels_array: value.get("image_as_json_pixels_array"),
            image_as_rgb_png: value.get("image_as_rgb_png"),
            width: w as u32,
            height: h as u32,
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

#[derive(Debug)]
pub struct NewWikipageModel {
    pub id: i64,
    pub title: Option<String>,
    pub ns: Option<String>,
    pub redirect: Option<String>,
    pub timestamp: String,
    pub contributor: Option<String>,
    pub comment: Option<String>,
    pub model: Option<String>,
    pub format: Option<String>,
    pub text: Option<String>,
    pub sha1: Option<String>,
}

#[derive(Debug)]
pub struct WikipageModel {
    pub dbid: i64,
    pub id: i64,
    pub title: Option<String>,
    pub ns: Option<String>,
    pub redirect: Option<String>,
    pub timestamp: String,
    pub contributor: Option<String>,
    pub comment: Option<String>,
    pub model: Option<String>,
    pub format: Option<String>,
    pub text: Option<String>,
    pub sha1: Option<String>,
}

impl From<&Row> for WikipageModel {
    fn from(value: &Row) -> Self {
        WikipageModel {
            dbid: value.get("dbid"),
            id: value.get("id"),
            title: value.get("title"),
            ns: value.get("ns"),
            redirect: value.get("redirect"),
            timestamp: value.get("timestamp"),
            contributor: value.get("contributor"),
            comment: value.get("comment"),
            model: value.get("model"),
            format: value.get("format"),
            text: value.get("text"),
            sha1: value.get("sha1"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PixelModel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
