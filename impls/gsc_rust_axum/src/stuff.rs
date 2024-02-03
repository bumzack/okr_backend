use chrono::{DateTime, Utc};

use crate::models::Article;

pub const LEN_CODE: usize = 20;
pub const LEN_TITLE: usize = 100;
pub const LEN_DESC: usize = 1700;
pub const LEN_ATTRIBUTES: usize = 200;
pub const LEN_CATEGORIES: usize = 200;
pub const LEN_POS: usize = 30;
pub const LEN_PRICE: usize = 20;
pub const LEN_START_DATE: usize = 25;
pub const LEN_END_DATE: usize = 25;

pub trait Code {
    fn code(&self) -> String;
}

pub trait Pos {
    fn pos(&self) -> String;
}

pub trait Price {
    fn price(&self) -> String;
}

pub trait Title {
    fn title(&self) -> String;
}

pub trait Description {
    fn description(&self) -> String;
}

pub trait Attributes {
    fn attributes(&self) -> String;
}

pub trait Categories {
    fn categories(&self) -> String;
}

pub trait StartDate {
    fn start_date(&self) -> String;
}

pub trait EndDate {
    fn end_date(&self) -> String;
}

impl Code for String {
    fn code(&self) -> String {
        self[0..LEN_CODE].trim_start_matches('0').to_string()
    }
}

impl Title for String {
    fn title(&self) -> String {
        self[START_TITLE..START_DESC].to_string()
    }
}

impl Description for String {
    fn description(&self) -> String {
        self[START_DESC..START_ATTR].to_string()
    }
}

impl Attributes for String {
    fn attributes(&self) -> String {
        self[START_ATTR..START_CAT].to_string()
    }
}

impl Categories for String {
    fn categories(&self) -> String {
        self[START_CAT..START_POS].to_string()
    }
}

impl Pos for String {
    fn pos(&self) -> String {
        self[START_POS..START_PRICE]
            .trim_start_matches('0')
            .trim()
            .to_string()
    }
}

impl Price for String {
    fn price(&self) -> String {
        self[START_PRICE..START_START_DATE].to_string()
    }
}

impl StartDate for String {
    fn start_date(&self) -> String {
        self[START_START_DATE..START_END_DATE].to_string()
    }
}

impl EndDate for String {
    fn end_date(&self) -> String {
        self[START_END_DATE..START_END_DATE + LEN_END_DATE].to_string()
    }
}

const START_CODE: usize = 0;

const START_TITLE: usize = START_CODE + LEN_CODE;
const START_DESC: usize = START_TITLE + LEN_TITLE;
const START_ATTR: usize = START_DESC + LEN_DESC;
const START_CAT: usize = START_ATTR + LEN_ATTRIBUTES;

const START_POS: usize = START_CAT + LEN_CATEGORIES;
const START_PRICE: usize = START_POS + LEN_POS;
const START_START_DATE: usize = START_PRICE + LEN_PRICE;
const START_END_DATE: usize = START_START_DATE + LEN_START_DATE;

impl Code for Article {
    fn code(&self) -> String {
        self.code.to_string()
    }
}

impl Title for Article {
    fn title(&self) -> String {
        self.title.to_string()
    }
}

impl Description for Article {
    fn description(&self) -> String {
        self.description.to_string()
    }
}

impl Attributes for Article {
    fn attributes(&self) -> String {
        self.attributes.to_string()
    }
}

impl Categories for Article {
    fn categories(&self) -> String {
        self.categories.to_string()
    }
}

impl Pos for Article {
    fn pos(&self) -> String {
        self.pos.to_string()
    }
}

impl Price for Article {
    fn price(&self) -> String {
        self.price.to_string()
    }
}

impl StartDate for Article {
    fn start_date(&self) -> String {
        self.start_date.to_string()
    }
}

impl EndDate for Article {
    fn end_date(&self) -> String {
        self.end_date.to_string()
    }
}

impl From<String> for Article {
    fn from(value: String) -> Self {
        let l = value.as_str();
        let code = &l[0..LEN_CODE];
        let title = &l[START_TITLE..START_DESC];
        let desc = &l[START_DESC..START_ATTR];
        let attr = &l[START_ATTR..START_CAT];
        let cat = &l[START_CAT..START_POS];
        let pos = &l[START_POS..START_PRICE];
        //println!("price {}", &l[START_PRICE..START_START_DATE]);
        let price = l[START_PRICE..START_START_DATE]
            .parse::<f64>()
            .expect("parsing price");
        let start_date = l[START_START_DATE..START_END_DATE]
            .parse::<i64>()
            .expect("parsing start date");
        let end_date = l[START_END_DATE..START_END_DATE + LEN_END_DATE]
            .parse::<i64>()
            .expect("parsing end date");
        let start_time =
            DateTime::<Utc>::from_timestamp(start_date, 0).expect("invalid timestamp starte date");
        let end_time =
            DateTime::<Utc>::from_timestamp(end_date, 0).expect("invalid timestamp end date");

        let start_date = start_time.format("%Y-%m-%dT%H:%M:%S").to_string();
        let end_date = end_time.format("%Y-%m-%dT%H:%M:%S").to_string();

        Article {
            code: code.trim_start_matches('0').to_string(),
            title: title.trim().to_string(),
            description: desc.trim().to_string(),
            categories: cat.trim().to_string(),
            attributes: attr.trim().to_string(),
            price,
            start_date,
            end_date,
            pos: pos.trim_start_matches('0').trim().to_string(),
        }
    }
}
