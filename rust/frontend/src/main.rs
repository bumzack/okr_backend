use reqwasm::http::Request;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::MouseEvent;

use commonbefe::models::{Article, Image};

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

const SERVER: &str = "http://localhost:2323";
const API_URL_SINGLE_THREADED: &str = "api/articles";

async fn post_single_threaded() -> Result<Vec<Article>, reqwasm::Error> {
    console_log!("post_single_threaded!");

    //  "/api/articles/{pageNumber}/{pageSize}";
    let page_number = 0;
    let page_size = 2;
    let url = format!(
        "{}/{}/{}/{}",
        SERVER, API_URL_SINGLE_THREADED, page_number, page_size
    );

    let re = Request::get(&url).send().await?.text().await;

    let response = re.expect("should be a valid Response/Body !!!");
    console_log!("response    {:?}", &response);
    let articles: serde_json::error::Result<Vec<Article>> = serde_json::from_str(&response);

    let articles = articles.unwrap();

    console_log!("list of articles {:?}", &articles);
    console_log!("updated data");

    Ok(articles)
}

#[component]
async fn Header<G: Html>(cx: Scope<'_>) -> View<G> {
    view! { cx,
        header(class = "py-3 mb-3 border-bottom") {
            div(class = "container-fluid d-grid gap-3 align-items-center", style ="rid-template-columns: 1fr 2fr;") {
                span(class="navbar-brand mb-0 h1") {
                    "Frontend for OKR Backend Thingi"
                }
            }
        }
    }
}

#[component(inline_props)]
async fn ServerTargetStatsComp<G: Html>(cx: Scope<'_>, stats: Vec<Image>) -> View<G> {
    // let images = stats.get().clone();
    // let x: Vec<&Image> = images.iter().map(|i| i).collect();
    let images = create_signal(cx, stats);

    view! {cx,
        div(class = "row", style ="margin-bottom: 10px;") {
            div (class="col-12") {
                Keyed (
                    iterable = images,
                    view =| cx, Image { resolution, image, .. }  | view! { cx,
                        h4 {
                            "resolution " (resolution)
                        }
                        img(src=format!("data:image/png;base64,  {}", image ))
                    },
                    key =|img | {
                         let a = img.clone();
                        a.resolution.clone()
                    },
                )
            }
        }
    }
}

#[component]
async fn MainContent<G: Html>(cx: Scope<'_>) -> View<G> {
    let mut iter = create_signal(cx, vec![]);

    let start_singlethreaded = move |e: MouseEvent| {
        e.prevent_default();
        console_log!("start_singlethreaded  clicked.   event {:?}", e.target());
        spawn_local_scoped(cx, {
            async move {
                let res = post_single_threaded().await;
                match res {
                    Ok(articles) => {
                        iter.set(articles);
                    }
                    Err(e) => {
                        console_log!("error calling server /api/articles/.... .  {:?}", e)
                    }
                }
            }
        });
    };

    view! { cx,
        div(class = "container-fluid") {
            div(class = "row") {
                div(class = "col-2") {
                    div(class = "list-group",  id="list-example") {
                       "left text"
                    }
                }
                div(class="col"){
                    div(class="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom") {
                        h1(class="h1"){
                            "Images"
                        }
                        div(class="btn-toolbar mb-2 mb-md-0"){
                            div(class="btn-group me-2"){
                                "bla"
                            }
                        }
                    }
                    div {
                        div(class ="canvas-container"  ) {
                            "canvas container"
                        }
                    }
                    div(class = "row", style ="margin-bottom: 10px;") {
                        div (class="col-12") {
                            button(class="btn btn-primary", type="button", id="singlethreaded" ,on:click=start_singlethreaded) {
                                "load images"
                            }
                            br {
                            }
                            p(id = "rust-single-threaded") {
                                "Duration:"
                            }
                        }
                    }

                     div(class = "row", style ="margin-bottom: 10px;") {
                        div (class="col-12") {
                            Keyed (
                                iterable = iter,
                                view =| cx, Article { code, description , images , .. }  | view! { cx,
                                    a(class="list-group-item list-group-item-action", href=format!("#list-item-{}", 1)) {
                                            ( description) "code: " (code)
                                    }
                                    ServerTargetStatsComp(stats = images)
                                 },
                                key =|article | {
                                     let a = article.clone();
                                    a.code
                                },
                            )
                        }
                    }
                }
            }
        }
    }
}

#[component]
async fn App<G: Html>(cx: Scope<'_>) -> View<G> {
    view! { cx,
       Header
        MainContent
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| view! { cx, App {} });
}
