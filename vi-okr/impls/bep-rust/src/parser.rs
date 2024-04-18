use chrono::{TimeZone, Utc};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

#[derive(PartialEq, Debug)]
struct LineIndices {
    from: usize,
    to: usize,
    to_exclusive: usize,
}

impl LineIndices {
    fn new(from: usize, to: usize) -> LineIndices {
        LineIndices {
            from,
            to,
            to_exclusive: to + 1,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Column<'a> {
    pub name: &'a str,
    pub length: usize,
    pub pad_left: char,
    pub pad_right: char,
}

impl Column<'_> {
    const fn default<'a>(name: &'a str, length: usize) -> Column<'a> {
        Column { name, length, pad_left: ' ', pad_right: ' ' }
    }

    const fn new<'a>(name: &'a str, length: usize, pad_left: char, pad_right: char) -> Column<'a> {
        Column { name, length, pad_left, pad_right }
    }
}

#[derive(PartialEq, Debug)]
pub struct ColumnAndIndices<'a> {
    pub column: &'a Column<'a>,
    pub indices: LineIndices,
}

type ColumnMap<'a> = HashMap<&'a str, ColumnAndIndices<'a>>;

pub(crate) fn build_indices<'a>(columns: &'a [Column]) -> ColumnMap<'a> {
    let summed_up_lengths: Vec<usize> =
        std::iter::once(&0)  // Rust scan method does not prepend the state
            .chain(columns.iter().map(|it| &it.length))
            .scan(0, |state, curr| {
                *state += curr;
                Some(*state)
            })
            .collect();
    let end_indices_plus_1 = summed_up_lengths.iter().skip(1);
    let indices = summed_up_lengths.iter()
        .zip(end_indices_plus_1)
        .map(|(from, to)| LineIndices::new(*from, *to - 1));
    columns.iter()
        .zip(indices)
        .map(|(column, indices)| (column.name, ColumnAndIndices { column, indices }))
        .collect()
}

pub(crate) const COLUMNS: [Column; 9] = [
    Column::new("code", 20, '0', ' '),
    Column::default("title", 100),
    Column::default("description", 1700),
    Column::default("attributes", 200),
    Column::default("categories", 200),
    Column::new("pos", 30, '0', ' '),
    Column::new("price", 20, '0', '0'),
    Column::new("startDate", 25, '0', '0'),
    Column::new("endDate", 25, '0', '0'),
];


// everything should act on string slices, so no allocation
fn parse_line<'a, 'b>(line: &'a str, column: &ColumnAndIndices<'b>) -> &'a str {
    let indexed_line = &line[column.indices.from..column.indices.to_exclusive];
    indexed_line.trim_start_matches(column.column.pad_left).trim_end_matches(column.column.pad_right)
}

fn parse_timestamp<'a>(line: &'a str) -> String {
    let timestamp: i64 = line.parse::<>().unwrap();
    let date = Utc.timestamp_millis_opt(timestamp * 1000).unwrap();
    date.to_rfc3339()
}

pub struct ArticleLine<'b> {
    line: String,
    pub(crate) price: f64,
    pub pos: usize,
    pub(crate) code: String,
    columns: &'b ColumnMap<'b>,
}

impl<'b> ArticleLine<'b> {
    fn new(line: String, columns: &'b ColumnMap<'b>) -> ArticleLine<'b> {
        // we crash if we can not parse it
        let pos = parse_line(&line, &columns["pos"]).parse::<>().unwrap();
        let code = String::from(parse_line(&line, &columns["code"]));
        let price = parse_line(&line, &columns["price"]).parse::<>().unwrap();
        ArticleLine { line, pos, code, price, columns }
    }

    pub(crate) fn title(&self) -> &str {
        parse_line(&self.line, &self.columns["title"])
    }

    pub fn description(&self) -> &str {
        parse_line(&self.line, &self.columns["description"])
    }

    pub fn attributes(&self) -> &str {
        parse_line(&self.line, &self.columns["attributes"])
    }

    pub fn categories(&self) -> &str {
        parse_line(&self.line, &self.columns["categories"])
    }

    pub fn start_date(&self) -> String {
        parse_timestamp(parse_line(&self.line, &self.columns["startDate"]))
    }

    pub fn end_date(&self) -> String {
        parse_timestamp(parse_line(&self.line, &self.columns["endDate"]))
    }

    pub fn is_in_same_group(&self, other: &ArticleLine) -> bool {
        return self.pos == other.pos && self.code == other.code;
    }

    pub fn is_cheaper_than(&self, other: &ArticleLine) -> bool {
        return self.price < other.price;
    }
}

pub struct ProcessedArticles<'b> {
    pub(crate) cheapest: ArticleLine<'b>,
    pub(crate) parsed_lines: usize,
}

// custom group by to deal with non GAT, shitty group_by from itertools
fn group_by<T, V: PartialEq>(iterator: impl Iterator<Item=T>, key: fn(&T) -> V) -> impl Iterator<Item=Vec<T>> {
    let mut iterator = iterator.peekable();
    std::iter::from_fn(move || {
        let mut result: Vec<T> = Vec::new();
        let first_key = iterator.peek().map(|f| key(f));
        let group = iterator.by_ref()
            .peeking_take_while(|elem| {
                first_key.as_ref().map_or(false, |f| *f == key(&elem))
            });
        for elem in group {
            result.push(elem);
        }
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    })
}


fn parse_lines<'b>(lines: Lines<BufReader<File>>, columns: &'b ColumnMap<'b>) -> impl Iterator<Item=ProcessedArticles<'b>> {
    group_by(lines.flatten()
        .into_iter()
        .map(move |line| ArticleLine::new(line, &columns)), |line| (line.code.clone(), line.pos))
        .into_iter()
        .flat_map(|articles| {
            let parsed_lines = articles.len();
            articles.into_iter()
                .min_by(|a, b| a.price.partial_cmp(&b.price).unwrap())
                .map(move |cheapest| ProcessedArticles { parsed_lines, cheapest })
                .into_iter()
        })
}

pub(crate) fn parse_files<'a, 'b>(files: &'a Vec<DirEntry>, columns: &'b ColumnMap) -> impl Iterator<Item=ProcessedArticles<'b>>
    where 'a: 'b {
    files.iter().flat_map(move |entry| {
        let path = &entry.path();
        println!("Beginning parsing file {:?}", path.file_name());
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        parse_lines(reader.lines(), &columns)
    })
}

pub(crate) fn files_in_directory(directory: &PathBuf) -> Vec<DirEntry> {
    fs::read_dir(directory).unwrap()
        .map(|path| path.unwrap())
        .filter(|path| path.metadata().unwrap().is_file() && path.file_name().into_string().unwrap().ends_with(".txt"))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::parser::{build_indices, Column, ColumnAndIndices, COLUMNS, files_in_directory, LineIndices, parse_files, parse_line, parse_timestamp, ProcessedArticles};

    #[test]
    fn should_build_indices() {
        let columns: [Column; 3] = [
            Column::default("code", 20),
            Column::default("test", 100),
            Column::default("kick", 1),
        ];
        let result = build_indices(&columns);
        assert_eq!(ColumnAndIndices { column: &columns[0], indices: LineIndices::new(0, 19) }, result["code"]);
        assert_eq!(ColumnAndIndices { column: &columns[1], indices: LineIndices::new(20, 119) }, result["test"]);
        assert_eq!(ColumnAndIndices { column: &columns[2], indices: LineIndices::new(120, 120) }, result["kick"]);
    }

    #[test]
    fn should_parse_date() {
        assert_eq!("2024-02-23T12:00:35+00:00", parse_timestamp("1708689635"))
    }

    #[test]
    fn should_parse_line() {
        let columns: [Column; 3] = [
            Column::default("code", 2),
            Column::new("test", 4, '0', ' '),
            Column::default("kick", 5),
        ];
        let indices = build_indices(&columns);
        assert_eq!("1", parse_line("x 001 a     ", &indices["test"]));
    }

    #[test]
    fn test_files_in_directory() {
        let test_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("test/");
        let files = files_in_directory(&test_dir);
        let columns = build_indices(&COLUMNS);
        let mut processed = 0;
        let mut written = 0;
        let articles: Vec<ProcessedArticles> = parse_files(&files, &columns)
            .collect();
        articles.iter().for_each(|article| {
            let processed = &mut processed;
            let written = &mut written;
            *processed += article.parsed_lines;
            *written += 1;
        });

        assert_eq!(3, written);
        assert_eq!(10usize, processed);

        let article = articles.first().unwrap();
        assert_eq!("1", article.cheapest.code);
        assert_eq!("Article with code 00000001", article.cheapest.title());
        assert_eq!(1494.7479, article.cheapest.price);
    }

    #[test]
    fn test_parse_files_in_directory() {
        let test_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("test/");
        let result = files_in_directory(&test_dir);

        assert_eq!(1, result.len());
        assert_eq!("articles_000001.txt", result[0].file_name());
    }
}