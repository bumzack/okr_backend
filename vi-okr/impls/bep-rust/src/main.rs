#[macro_use]
extern crate rocket;
extern crate struson;

use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use std::path::Path;
use struson::serde::JsonWriterSerializer;
use struson::writer::*;

mod parser;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct SysInfo<'a, 'b, 'c, 'd, 'e> {
    author: &'a str,
    language: &'b str,
    framework: &'c str,
    multithreaded: bool,
    version: &'d str,
    comment: &'e str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Article<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    price: f64,
    pos: usize,
    code: &'a str,
    title: &'b str,
    description: &'c str,
    attributes: &'d str,
    categories: &'e str,
    startDate: &'f str,
    endDate: &'g str,
}

#[get("/api/v1/sysinfo")]
fn sysinfo() -> Json<SysInfo<'static, 'static, 'static, 'static, 'static>> {
    Json(SysInfo {
        author: "Bernhard Posselt",
        language: "Rust",
        framework: "Rocket",
        multithreaded: false,
        version: "0.0.1",
        comment: "Rust",
    })
}

#[derive(Deserialize)]
struct Data {
    returnItems: bool,
}

#[post("/api/v1/articles/import", data = "<data>")]
fn import(data: Json<Data>) -> String {
    let mut writer = Vec::<u8>::new();
    let mut json_writer = JsonStreamWriter::new(&mut writer);
    json_writer.begin_object();
    json_writer.name("articles");
    json_writer.begin_array();
    let dir = std::env::var("DATA_DIRECTORY").expect("No DATA_DIRECTORY environment variable provided!");
    let test_dir = Path::new(&dir).to_path_buf();
    let files = parser::files_in_directory(&test_dir);
    let columns = parser::build_indices(&parser::COLUMNS);
    let mut processed = 0;
    let mut written = 0;
    parser::parse_files(&files, &columns).for_each(|article| {
        let processed = &mut processed;
        let written = &mut written;
        *processed += article.parsed_lines;
        *written += 1;

        if data.returnItems {
            let line = article.cheapest;
            let start_date = line.start_date();
            let end_date = line.start_date();

            let json = Article {
                price: line.price,
                title: line.title(),
                code: line.code.as_str(),
                pos: line.pos,
                attributes: &line.attributes(),
                categories: &line.categories(),
                description: &line.description(),
                endDate: &start_date.as_str(),
                startDate: &end_date.as_str(),
            };
            let mut serializer = JsonWriterSerializer::new(&mut json_writer);
            json.serialize(&mut serializer);
        }
    });
    json_writer.end_array();
    json_writer.name("linesProcessed");
    json_writer.number_value(processed);
    json_writer.name("dbRowsWritten");
    json_writer.number_value(written);
    json_writer.end_object();
    json_writer.finish_document();
    String::from_utf8(writer).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![sysinfo, import])
}