use std::fs::File;
use std::io::{Error, Write};
use std::ops::{Add, Sub};
use std::time::Instant;

use chrono::Duration;
use log::{info, LevelFilter};
use pretty_env_logger::env_logger::Builder;
use rand::{Rng, thread_rng};
use rand::prelude::ThreadRng;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::new().filter_level(LevelFilter::Info).init();

    let mut rng = thread_rng();

    let cnt_files = 1;
    let cnt_articles_min = 5_000_000;
    let cnt_articles_max = 7_000_000;
    let cnt_pos = 50;

    write_files(&mut rng, cnt_files, cnt_articles_min, cnt_articles_max, cnt_pos);

    Ok(())
}

fn write_files(mut rng: &mut ThreadRng, cnt_files: usize, cnt_articles_min: usize, cnt_articles_max: usize, cnt_pos: usize) {
    let path = env!("CARGO_MANIFEST_DIR");

    let filenames = [
        "articles_001_010.txt".to_string(),
        "articles_002_010.txt".to_string(),
        "articles_003_010.txt".to_string(),
        "articles_004_010.txt".to_string(),
        "articles_005_010.txt".to_string(),
        "articles_006_010.txt".to_string(),
        "articles_007_010.txt".to_string(),
        "articles_008_010.txt".to_string(),
        "articles_009_010.txt".to_string(),
        "articles_010_010.txt".to_string(),
    ];
    let mut articles = vec![];

    let filenames: Vec<&String> = filenames.iter().take(cnt_files).collect();


    for f in filenames {
        let start = Instant::now();
        info!("start writing file {} at {:?}", f,& start);

        let cnt_articles: usize = rng.gen_range(cnt_articles_min..cnt_articles_max);

        for i in 0..cnt_articles {
            for pos in 0..cnt_pos {
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
        }
        info!("write file start");
        let filename = format!("{}/{}", path, f);
        let mut file = File::create(filename).expect("should open file");

        articles.iter().for_each(|a| {
            file.write(a.as_bytes()).expect("should write an article");
            file.write("\n".as_bytes()).expect("should write an article");
        });
        info!("finished writing file {} at {:?}. took  {}ms", f,& Instant::now(), start.elapsed().as_millis());
    }
}

fn convert_to_string(a: &Article) -> String {
    let precision = 4;
    let p = format!("{:0>20.1$}", a.price, precision);

    format!("{0:0>20}{1: <100}{2: <1700}{3: <200}{4: <200}{5: <30}{6}{7:0>25}{8:0>25}",
            a.code,
            a.title,
            a.description,
            a.attributes,
            a.categories,
            a.pos,
            p,
            a.start_date,
            a.end_date,
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
    let attributes: Vec<String> = attributes.iter()
        .map(|a| {
            format!("{}: {}", a.name, a.value)
        })
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

    let category =
        Category {
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
