use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Article {
    pub code: String,
    pub title: String,
    pub description: String,
    pub images: Vec<Image>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Image {
    pub filename: String,
    pub image: String,
    pub resolution: String,
    pub width:i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Resolution {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub original: bool,
}
