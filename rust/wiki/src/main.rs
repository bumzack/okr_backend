use std::fs::File;
use std::io::Read;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use tokio_postgres::Error;

use common::db_wikipage::insert_wikipage;
use common::models::NewWikipageModel;
use common::utils::create_pool;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let filename = test_file_name();
    // let filename = "/Users/gsc/Downloads/wiki/dewiki-20231201-pages-articles-multistream.xml";
    let mut f = File::open(filename).expect("open file should work");

    let start = Instant::now();
    let mut s = String::new();
    let bytes_read = f
        .read_to_string(&mut s)
        .expect("should be readline into string");

    println!(
        "bytes read {} in {} secs",
        &bytes_read,
        start.elapsed().as_secs()
    );

    let start = Instant::now();
    let wiki: MediaWiki = from_str(&s).expect("should work");

    println!(
        "converting to structs took {} secs",
        &start.elapsed().as_secs()
    );

    let pool = create_pool("prod".to_string());

    let start = Instant::now();

    for p in wiki.pages {
        // println!("page title {:?}", &p.title);

        let text = match &p.revision {
            Some(r) => match &r.text {
                Some(t) => t.value.clone(),
                None => None,
            },
            None => None,
        };

        let sha1 = match &p.revision {
            Some(r) => r.sha1.clone(),
            None => None,
        };

        let contributor = match &p.revision {
            Some(r) => r.contributor.clone(),
            None => None,
        };

        let comment = match &p.revision {
            Some(r) => r.comment.clone(),
            None => None,
        };
        let model = match &p.revision {
            Some(r) => r.model.clone(),
            None => None,
        };
        let timestamp = match &p.revision {
            Some(r) => r.timestamp.clone(),
            None => "-1".to_string(),
        };

        let format = match &p.revision {
            Some(r) => r.format.clone(),
            None => None,
        };

        let id = match &p.id {
            Some(pp) => pp.parse::<i32>().unwrap_or_else(|e| -1),
            None => -1,
        };

        let contributor = contributor.map(|c| format!("{:?}", c));
        let redirect = p.redirect.clone().map(|c| format!("{:?}", c));
        let ns = p.ns.clone().map(|c| format!("{:?}", c));
        let title = p.title.clone().map(|c| format!("{:?}", c));

        let wiki_page = NewWikipageModel {
            id,
            title,
            ns,
            redirect,
            timestamp,
            contributor,
            comment,
            model,
            format,
            text,
            sha1,
        };
        let res = insert_wikipage(&pool, &wiki_page)
            .await
            .expect("insert into DB should work");

        // println!("inserted wiki page {:?}", &p);
        // println!("inserted wiki page res   {:?}", &res);
    }

    println!(
        "success importing data into DB. duration  {} secs",
        start.elapsed().as_secs()
    );

    // let json = json!("&wiki").to_string();
    // fs::write("result.json", &json).expect("write to JSON should work");

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MediaWiki {
    siteinfo: Siteinfo,
    #[serde(rename = "page", default)]
    pages: Vec<Page>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Siteinfo {
    sitename: Option<String>,
    dbname: Option<String>,
    base: Option<String>,
    generator: Option<String>,
    case: Option<String>,
    namespaces: Option<Vec<Namespace>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Namespace {
    key: Option<String>,
    case: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Page {
    title: Option<String>,
    ns: Option<String>,
    id: Option<String>,
    redirect: Option<Redirect>,
    revision: Option<Revision>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct Redirect {
    title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Revision {
    id: String,
    timestamp: String,
    contributor: Option<Contributor>,
    comment: Option<String>,
    model: Option<String>,
    format: Option<String>,
    text: Option<Text>,
    sha1: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct Contributor {
    username: Option<String>,
    id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct Text {
    bytes: Option<String>,
    // #[serde(rename = "xml:space", default)]
    // xml_space: Option<String>,
    #[serde(rename = "$value")]
    value: Option<String>,
}

fn test_file_name() -> String {
    let path = env!("CARGO_MANIFEST_DIR");
    let filename = "test.xml";
    let filename = format!("{}/{}", path, filename);
    filename
}
