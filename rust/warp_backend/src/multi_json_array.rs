use crossbeam_channel::bounded;
use log::{error, info};
use tokio::time::Instant;
use warp::{Filter, Rejection, Reply};

use common::db_art2img::read_art2img;
use common::db_articles::{read_articles, read_articles_paginated};
use common::db_image::read_images;
use common::db_resolution::read_resolutions;
use common::models::{ArticleModel, ImageModel, PixelModel};
use common::utils::build_response_from_json;
use commonbefe::models::{Article, Image, Resolution};

use crate::POOL;
use crate::utils::{create_ppm, crop_image, get_sorted_resolutions, invert_image, mirror_image};

pub fn article_routes_multi_json_array() -> impl Filter<Extract=(impl Reply, ), Error=Rejection> + Clone {
    let server1 = warp::path!("multithreaded" / "api" / "articles");
    let find_all_multi = server1
        .and(warp::get())
        .and_then(|| {
            info!("GET /multithreaded/api/articles");
            find_all_multi()
        });

    let server1 = warp::path!("multithreaded" / "api" / "articles" / u32 / u32);
    let find_paginated_multi = server1
        .and(warp::get())
        .and_then(|page_number: u32, page_size: u32| {
            info!("GET /multithreaded/api/articles/{page_number}/{page_size}");
            find_paginated_multi(page_number, page_size)
        });

    find_all_multi.or(find_paginated_multi)
}

async fn find_all_multi() -> Result<impl Reply, Rejection> {
    let resolutions =
        get_sorted_resolutions(read_resolutions(&POOL).await.expect("read resolutions"));
    let articles = read_articles(&POOL).await.expect("read articles");
    let response = resize_all_images_multi(articles, resolutions).await?;
    Ok(response)
}

async fn find_paginated_multi(
    page_number: u32,
    page_size: u32,
) -> Result<impl Reply, Rejection> {
    let resolutions =
        get_sorted_resolutions(read_resolutions(&POOL).await.expect("read resolutions"));
    let articles = read_articles_paginated(&POOL, page_number, page_size)
        .await
        .expect("read articles");

    let response = resize_all_images_multi(articles, resolutions).await?;

    Ok(response)
}

async fn resize_all_images_multi(
    raw_articles: Vec<ArticleModel>,
    resolutions: Vec<Resolution>,
) -> Result<impl Reply, Rejection> {
    let cores = 4;// num_cpus::get();

    let cnt_articles = raw_articles.len();

    let (sender, receiver) = bounded::<ArticleModel>(cnt_articles);
    let (tx, rx) = bounded::<Article>(cnt_articles);

    info!("total articles to process {}", raw_articles.len());
    for a in raw_articles {
        sender.send(a).expect("sending should work");
        info!("sending article ");
    }

    let (res, duration) = crossbeam::scope(|s| {
        let start = Instant::now();
        let mut threads = vec![];

        for i in 0..cores {
            let receiver = receiver.clone();
            let tx = tx.clone();
            let resolutions = resolutions.clone();
            let worker = s.spawn(move |_| {
                let id = i;
                let start = Instant::now();
                info!("worker_thread {:?} started", id);
                let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
                let mut articles_processed = 0;
                loop {
                    if receiver.is_empty() {
                        info!("receiver is empty     thread {}", id);
                        break;
                    }
                    let finished = match receiver.recv() {
                        Ok(article) => {
                            info!(
                                "thread {} received a raw article    {:?}",
                                id, article.code
                            );
                            info!("got an article to process     in thread {:?}", id);
                            let mut images_resized = vec![];
                            for resolution in &resolutions {
                                // https://stackoverflow.com/questions/52521201/how-do-i-synchronously-return-a-value-calculated-in-an-asynchronous-future
                                let art2imgs = runtime
                                    .block_on(read_art2img(&POOL, article.id as i32))
                                    .expect("read art2imgs");

                                let imgids: Vec<i32> =
                                    art2imgs.iter().map(|art2img| art2img.image_id as i32).collect();
                                let images = runtime
                                    .block_on(read_images(&POOL, &imgids))
                                    .expect("read images");

                                images_resized.append(&mut resize_multi(images, resolution));
                            }
                            let full_article = Article {
                                code: article.code.clone(),
                                title: article.title.clone(),
                                description: article.description.clone(),
                                images: images_resized,
                            };
                            info!("sending an article      in thread {:?}", id);
                            articles_processed += 1;
                            tx.send(full_article).expect("sending should work");
                            false
                        }

                        Err(e) => {
                            error!("last raw article received  {:?}", e);
                            true
                        }
                    };
                    if finished {
                        info!("thread id {}.  break out of loop of raw articles ", id);
                        break;
                    }
                }

                let duration = start.elapsed().as_millis();
                info!("worker_thread {:?} finished", id);

                (id, duration)
            });
            threads.push(worker);
        }
        info!("after starting all threads");

        let mut articles = vec![];

        for i in 0..cnt_articles {
            match rx.recv() {
                Ok(article) => {
                    info!("received a finished  {} article   {:?}", i, article.code);
                    articles.push(article);
                }
                Err(e) => {
                    error!("last article received  {:?}", e);
                }
            };
        }

        info!("no more articles in rx");

        for child in threads {
            let (id, duration) = child.join().unwrap();
            info!("worker thread {:?} finished. run for {} ms", id, duration);
        }

        info!("no more full articles to receive");

        let duration = start.elapsed().as_millis();

        (articles, duration)
    })
        .expect("TODO: something went wrong");

    info!("PROCESSING is finished.   it took   {} ms ", duration);

    let response = build_response_from_json(res);
    Ok(response)
}

fn resize_multi(images: Vec<ImageModel>, resolution: &Resolution) -> Vec<Image> {
    let res_images: Vec<Image> = images
        .iter()
        .map(|img| resize_image_multi(resolution, img))
        .collect();

    res_images
}

fn resize_image_multi(resolution: &Resolution, img: &ImageModel) -> Image {
    let pixels: Vec<PixelModel> = serde_json::from_str(&img.image_as_json_pixels_array).expect("deserialize should work");
    let res = match resolution.original {
        true => {
            Resolution {
                name: resolution.name.clone(),
                width: img.width as i32,
                height: img.height as i32,
                original: true,
            }
        }
        false => {
            Resolution {
                name: resolution.name.clone(),
                width: resolution.width,
                height: resolution.height,
                original: false,
            }
        }
    };

    let mirrored_pixels = mirror_image(&pixels, img.width as usize, img.height as usize);
    let cropped_pixels = crop_image(&mirrored_pixels, img.width as usize, res.width as usize, res.height as usize);
    let inverted_pixels = invert_image(&cropped_pixels, res.width as usize, res.height as usize);
    let ppm = create_ppm(&inverted_pixels, res.width as usize, res.height as usize);

    Image {
        filename: img.filename.clone(),
        image: ppm,
        resolution: res.name.clone(),
        width: res.width,
        height: res.height,
    }
}
