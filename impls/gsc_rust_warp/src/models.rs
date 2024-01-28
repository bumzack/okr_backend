use std::env;
use std::io::Error;

use chrono::{DateTime, Utc};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use log::info;
use tokio_postgres::{NoTls, Row};

#[derive(Debug)]
pub struct ImportResult {
    pub lines_processed: usize,
    pub db_rows_written: usize,
    pub items: Vec<Article>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct Sysinfo {
    pub author: String,
    pub version: String,
    pub comment: String,
    pub language: String,
    pub framework: String,
    pub multithreaded: bool,
}
