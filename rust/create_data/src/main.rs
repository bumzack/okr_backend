use std::fs::File;
use std::io::{Error, Write};
use std::ops::{Add, Sub};
use std::time::Instant;

use chrono::Duration;
use log::{info, LevelFilter};
use pretty_env_logger::env_logger::Builder;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let mut rng = thread_rng();

    // prod_data(&mut rng);
    dev_data(&mut rng);

    Ok(())
}

fn dev_data(rng: &mut ThreadRng) {
    let cnt_articles_per_file_avg = 900;
    let cnt_articles_min = 3000;
    let cnt_articles_max = 4000;
    let cnt_pos = 5;

    write_files(
        rng,
        cnt_articles_per_file_avg,
        cnt_articles_min,
        cnt_articles_max,
        cnt_pos,
    );
}

fn prod_data(mut rng: &mut ThreadRng) {
    let cnt_articles_per_file_avg = 100_000;
    let cnt_articles_min = 1_000_000;
    let cnt_articles_max = 1_500_000;
    let cnt_pos = 10;

    write_files(
        &mut rng,
        cnt_articles_per_file_avg,
        cnt_articles_min,
        cnt_articles_max,
        cnt_pos,
    );
}

fn write_files(
    mut rng: &mut ThreadRng,
    cnt_articles_per_file_avg: usize,
    cnt_articles_min: usize,
    cnt_articles_max: usize,
    cnt_pos: usize,
) {
    let path = env!("CARGO_MANIFEST_DIR");

    let start = Instant::now();
    let cnt_articles: usize = rng.gen_range(cnt_articles_min..cnt_articles_max);
    let mut articles_per_file = get_article_cnt_for_file(&mut rng, cnt_articles_per_file_avg);
    let mut current_cnt_articles_per_file = 0;

    let mut articles = vec![];
    let mut file_cnt = 1;

    for i in 1..=cnt_articles {
        // info!("create article code  {}", i);

        for pos in 1..=cnt_pos {
            let pos = format!("{0:0>10}", pos);

            let cnt_prices: usize = rng.gen_range(2..8);
            let shirt_or_notebook = rng.gen_range(0..1);

            let mut article = match shirt_or_notebook {
                0 => tshirt(i, &mut rng),
                _ => notebook(i, &mut rng),
            };
            article.pos = pos;

            for _ in 0..cnt_prices {
                let price: f64 = rng.gen_range(23.0..2345.0);

                let start_date = chrono::Utc::now().sub(Duration::days(rng.gen_range(23..400)));
                let end_date = chrono::Utc::now().add(Duration::days(rng.gen_range(123..400)));

                article.start_date = start_date.timestamp().to_string();
                article.end_date = end_date.timestamp().to_string();
                article.price = price;
                articles.push(convert_to_string(&article));
            }
        }

        current_cnt_articles_per_file += 1;
        if current_cnt_articles_per_file > articles_per_file {
            info!(
                "current_cnt_articles_per_file {},   articles_per_file {}",
                current_cnt_articles_per_file, articles_per_file
            );

            let start_file = Instant::now();

            let filename = format!("{}/../articles_{:0>6}.txt", path, file_cnt);
            let mut f = File::create(&filename).expect("creating file should work");
            info!("write file start {} ", filename);

            articles.iter().for_each(|a| {
                let _ = f.write(a.as_bytes()).expect("should write an article");
            });

            info!(
                "finished writing file {}.  took  {}ms",
                filename,
                start_file.elapsed().as_millis()
            );
            articles.clear();
            articles_per_file = get_article_cnt_for_file(rng, cnt_articles_per_file_avg);
            file_cnt += 1;
            current_cnt_articles_per_file = 0;
        }
    }

    info!(
        "finished writing all files      took  {}ms   or {}secs",
        start.elapsed().as_millis(),
        start.elapsed().as_secs()
    );
}

fn get_article_cnt_for_file(rng: &mut ThreadRng, cnt_articles_per_file_avg: usize) -> usize {
    let min = cnt_articles_per_file_avg - cnt_articles_per_file_avg / 15;
    let max = cnt_articles_per_file_avg + cnt_articles_per_file_avg / 15;
    rng.gen_range(min..max)
}

fn convert_to_string(article: &Article) -> String {
    let precision = 4;
    let p = format!("{:0>20.1$}", article.price, precision);

    format!(
        "{0:0>20}{1: <100}{2: <1700}{3: <200}{4: <200}{5: <30}{6}{7:0>25}{8:0>25}\n",
        article.code,
        article.title,
        article.description,
        article.attributes,
        article.categories,
        article.pos,
        p,
        article.start_date,
        article.end_date,
    )
}

#[derive(Clone)]
struct Article {
    code: String,
    title: String,
    description: String,
    categories: String,
    attributes: String,
    price: f64,
    start_date: String,
    end_date: String,
    pos: String,
}

#[derive(Clone)]
struct Attribute {
    name: String,
    value: String,
}

#[derive(Clone)]
struct Category {
    lvl0: String,
    lvl1: String,
    lvl2: String,
}

fn tshirt(code: usize, rng: &mut ThreadRng) -> Article {
    let attributes_color = vec![
        Attribute {
            name: "color".to_string(),
            value: "red".to_string(),
        },
        Attribute {
            name: "color".to_string(),
            value: "blue".to_string(),
        },
        Attribute {
            name: "color".to_string(),
            value: "yellow".to_string(),
        },
        Attribute {
            name: "color".to_string(),
            value: "green".to_string(),
        },
    ];

    let attributes_size = vec![
        Attribute {
            name: "size".to_string(),
            value: "XS".to_string(),
        },
        Attribute {
            name: "size".to_string(),
            value: "S".to_string(),
        },
        Attribute {
            name: "size".to_string(),
            value: "M".to_string(),
        },
        Attribute {
            name: "size".to_string(),
            value: "L".to_string(),
        },
        Attribute {
            name: "size".to_string(),
            value: "XL".to_string(),
        },
        Attribute {
            name: "size".to_string(),
            value: "XXL".to_string(),
        },
    ];

    let attributes_fit = vec![
        Attribute {
            name: "fit".to_string(),
            value: "slim".to_string(),
        },
        Attribute {
            name: "fit".to_string(),
            value: "athletic".to_string(),
        },
        Attribute {
            name: "fit".to_string(),
            value: "regular".to_string(),
        },
    ];

    let categories = vec![
        Category {
            lvl0: "cloth".to_string(),
            lvl1: "cloth > thsirt".to_string(),
            lvl2: "cloth > tshirt > sportswar".to_string(),
        },
        Category {
            lvl0: "cloth".to_string(),
            lvl1: "cloth > thsirt".to_string(),
            lvl2: "cloth > tshirt > vintage".to_string(),
        },
        Category {
            lvl0: "cloth".to_string(),
            lvl1: "cloth > thsirt".to_string(),
            lvl2: "cloth > tshirt > designer".to_string(),
        },
    ];
    let desc = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse molestie consequat, vel illum dolore eu feugiat nulla facilisis at vero eros et accumsan et iusto odio dignissim qui blandit praesent luptatum zzril delenit augue duis dolore te feugait nulla facilisi. Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam nonummy nibh euismod tincidunt ut laoreet dolore magna aliquam erat volutpat. Ut wisi enim ad minim veniam, quis nostrud exerci tation ullamcorper suscipit lobortis nisl ut aliquip ex ea commodo consequat. Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse molestie consequat, vel illum dolore eu feugiat nulla facilisis at vero eros et accumsan et iusto odio dignissim qui blandit praesent luptatum zzril delenit augue duis dolore te feugait nulla facilisi.";
    let length_description = rng.gen_range(100..desc.len());
    let description = desc[0..length_description].to_string();

    let idx_color: usize = rng.gen_range(0..attributes_color.len());
    let color = attributes_color[idx_color].clone();

    let idx_size: usize = rng.gen_range(0..attributes_size.len());
    let size = attributes_size[idx_size].clone();

    let idx_fit: usize = rng.gen_range(0..attributes_fit.len());
    let fit = attributes_fit[idx_fit].clone();

    let price: f64 = rng.gen_range(23.0..45.0);
    let title = format!("Article with code {0:0>8}", code);
    let code = format!("{0:0>8}", code);

    let idx_cat: usize = rng.gen_range(0..categories.len());
    let cat = categories[idx_cat].clone();
    let categories = format!("{}//{}//{}", cat.lvl0, cat.lvl1, cat.lvl2);

    let attributes = vec![color, size, fit];
    let attributes: Vec<String> = attributes
        .iter()
        .map(|a| format!("{}: {}", a.name, a.value))
        .collect();
    let attributes = attributes.join("//");

    Article {
        code,
        title,
        description,
        categories,
        attributes,
        price,
        start_date: "".to_string(),
        end_date: "".to_string(),
        pos: "".to_string(),
    }
}

fn notebook(code: usize, rng: &mut ThreadRng) -> Article {
    let attributes_ram = vec![
        Attribute {
            name: "RAM".to_string(),
            value: "8G".to_string(),
        },
        Attribute {
            name: "RAM".to_string(),
            value: "16G".to_string(),
        },
        Attribute {
            name: "RAM".to_string(),
            value: "32G".to_string(),
        },
    ];

    let attributes_display = vec![
        Attribute {
            name: "Display".to_string(),
            value: "11inch".to_string(),
        },
        Attribute {
            name: "Display".to_string(),
            value: "13inch".to_string(),
        },
        Attribute {
            name: "Display".to_string(),
            value: "14inch".to_string(),
        },
    ];

    let attributes_cpu = vec![
        Attribute {
            name: "CPU".to_string(),
            value: "M1".to_string(),
        },
        Attribute {
            name: "CPU".to_string(),
            value: "M2".to_string(),
        },
        Attribute {
            name: "CPU".to_string(),
            value: "M3".to_string(),
        },
    ];

    let category = Category {
        lvl0: "Notebook".to_string(),
        lvl1: "Notebook > Apple".to_string(),
        lvl2: "".to_string(),
    };

    let desc = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse molestie consequat, vel illum dolore eu feugiat nulla facilisis at vero eros et accumsan et iusto odio dignissim qui blandit praesent luptatum zzril delenit augue duis dolore te feugait nulla facilisi. Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam nonummy nibh euismod tincidunt ut laoreet dolore magna aliquam erat volutpat. Ut wisi enim ad minim veniam, quis nostrud exerci tation ullamcorper suscipit lobortis nisl ut aliquip ex ea commodo consequat. Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse molestie consequat, vel illum dolore eu feugiat nulla facilisis at vero eros et accumsan et iusto odio dignissim qui blandit praesent luptatum zzril delenit augue duis dolore te feugait nulla facilisi.";
    let length_description = rng.gen_range(100..desc.len());
    let description = desc[0..length_description].to_string();

    let idx_cpu: usize = rng.gen_range(0..attributes_cpu.len());
    let color = attributes_cpu[idx_cpu].clone();

    let idx_display: usize = rng.gen_range(0..attributes_display.len());
    let size = attributes_display[idx_display].clone();

    let idx_ram: usize = rng.gen_range(0..attributes_ram.len());
    let fit = attributes_ram[idx_ram].clone();

    let price: f64 = rng.gen_range(23.0..45.0);
    let title = format!("Article with code {0:0>8}", code);
    let code = format!("{0:0>8}", code);

    let categories = format!("{}//{}//{}", category.lvl0, category.lvl1, category.lvl2);

    let attributes = vec![color.value, size.value, fit.value];
    let attributes = attributes.join("//");

    Article {
        code,
        title,
        description,
        categories,
        attributes,
        price,
        start_date: "".to_string(),
        end_date: "".to_string(),
        pos: "".to_string(),
    }
}
