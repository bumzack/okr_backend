use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
    pub lines_processed: usize,
    pub db_rows_written: usize,
    pub items: Vec<Article>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportRequest {
    pub return_items: bool,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sysinfo {
    pub author: String,
    pub version: String,
    pub comment: String,
    pub language: String,
    pub framework: String,
    pub multithreaded: bool,
}


pub const LEN_CODE: usize = 20;
pub const LEN_TITLE: usize = 100;
pub const LEN_DESC: usize = 1700;
pub const LEN_ATTRIBUTES: usize = 200;
pub const LEN_CATEGORIES: usize = 200;
pub const LEN_POS: usize = 30;
pub const LEN_PRICE: usize = 20;
pub const LEN_START_DATE: usize = 25;
pub const LEN_END_DATE: usize = 25;

