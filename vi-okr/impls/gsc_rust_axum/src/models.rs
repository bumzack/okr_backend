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

impl Default for ImportResult {
    fn default() -> Self {
        ImportResult {
            lines_processed: 0,
            db_rows_written: 0,
            items: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub attributes: String,
    pub categories: String,
    pub code: String,
    pub description: String,
    pub end_date: String,
    pub pos: String,
    pub price: f64,
    pub start_date: String,
    pub title: String,
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
