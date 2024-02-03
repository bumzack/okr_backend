use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImportRequest {
    pub return_items: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
    pub lines_processed: usize,
    pub db_rows_written: usize,
    pub items: Vec<Article>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub code: String,
    pub title: String,
    pub description: String,
    pub categories: String,
    pub attributes: String,
    pub price: f64,
    pub start_date: String,
    pub end_date: String,
    pub pos: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Sysinfo {
    pub author: String,
    pub comment: String,
    pub framework: String,
    pub language: String,
    pub version: String,
    pub multithreaded: bool,
}
