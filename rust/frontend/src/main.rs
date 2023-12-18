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

const SERVER_JAVA_REF_IMPL: &str = "http://localhost:2323";
const SERVER_RUST_WARP: &str = "http://localhost:2345";
const API_ARTICLES_PAGINATED: &str = "api/articles";

const API_ARTICLES_PAGINATED_RUST_SINGLE: &str = "singlethreaded/api/articles";
const API_ARTICLES_PAGINATED_RUST_MULTI: &str = "multithreaded/api/articles";
const API_ARTICLES_PAGINATED_RUST_RAYON: &str = "multithreaded/api/articles";

const PAGE_NUMBER: i32 = 0;
const PAGE_SIZE: i32 = 3;

async fn get_java_ref_impl() -> Result<Vec<Article>, reqwasm::Error> {
    console_log!("get_java_ref_impl");

    let url = format!(
        "{}/{}/{}/{}",
        SERVER_JAVA_REF_IMPL, API_ARTICLES_PAGINATED, PAGE_NUMBER, PAGE_SIZE
    );

    get_articles(&url).await
}

async fn get_rust_singlethreaded() -> Result<Vec<Article>, reqwasm::Error> {
    console_log!("get_rust_singlethreaded");
    let url = format!(
        "{}/{}/{}/{}",
        SERVER_RUST_WARP, API_ARTICLES_PAGINATED_RUST_SINGLE, PAGE_NUMBER, PAGE_SIZE
    );
    get_articles(&url).await
}

async fn get_rust_multithreaded() -> Result<Vec<Article>, reqwasm::Error> {
    console_log!("get_rust_multithreaded");
    let url = format!(
        "{}/{}/{}/{}",
        SERVER_RUST_WARP, API_ARTICLES_PAGINATED_RUST_MULTI, PAGE_NUMBER, PAGE_SIZE
    );
    get_articles(&url).await
}

async fn get_rust_rayon() -> Result<Vec<Article>, reqwasm::Error> {
    console_log!("get_rust_rayon");
    let url = format!(
        "{}/{}/{}/{}",
        SERVER_RUST_WARP, API_ARTICLES_PAGINATED_RUST_RAYON, PAGE_NUMBER, PAGE_SIZE
    );
    get_articles(&url).await
}

async fn get_articles(url: &String) -> Result<Vec<Article>, reqwasm::Error> {
    console_log!("get_articles.    url {}", url);

    let re = Request::get(&url).send().await?.text().await;
    let response = re.expect("should be a valid Response/Body !!!");
    // console_log!("response    {:?}", &response);
    let articles: serde_json::error::Result<Vec<Article>> = serde_json::from_str(&response);
    let articles = articles.unwrap();
    // console_log!("list of articles {:?}", &articles);

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
                    view =| cx, Image { resolution, image, filename }  | view! { cx,

                         div(class = "row") {
                            div ( class = "col-12") {

                                div (class="card",  style = "margin-bottom: 20px;") {
                                    img(   src=format!("data:image/png;base64,  {}", image ))

                                    div (class="card-body" ,  style = "background: #CCCCCC;" ) {
                                        h5 (class="card-title" ) {
                                            (filename)
                                        }
                                        p (class="card-text") {
                                           (resolution)
                                        }
                                    }
                                }
                            }
                        }
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
    let iter = create_signal(cx, vec![]);
    let duration_java_ref_impl = create_signal(cx, "Duration: ".to_string());
    let duration_rust_single = create_signal(cx, "Duration: ".to_string());
    let duration_rust_multi = create_signal(cx, "Duration: ".to_string());
    let duration_rust_rayon = create_signal(cx, "Duration: ".to_string());

    let start_java_reference_impl = move |e: MouseEvent| {
        e.prevent_default();
        console_log!(
            "start_java_reference_impl  clicked.   event {:?}",
            e.target()
        );
        spawn_local_scoped(cx, {
            async move {
                iter.set(vec![]);
                let window = web_sys::window().expect("should have a window in this context");
                let performance = window
                    .performance()
                    .expect("performance should be available");
                let start = performance.now() as i64;
                let res = get_java_ref_impl().await;
                let end = performance.now() as i64;
                let dur = end - start;
                let dur = format!("Duration: {} ms", dur);
                duration_java_ref_impl.set(dur);

                match res {
                    Ok(articles) => {
                        iter.set(articles);
                    }
                    Err(e) => {
                        console_log!("error calling server start_java_reference_impl   {:?}", e)
                    }
                }
            }
        });
    };

    let start_rust_single_threaded = move |e: MouseEvent| {
        e.prevent_default();
        console_log!(
            "start_rust_single_threaded  clicked.   event {:?}",
            e.target()
        );
        spawn_local_scoped(cx, {
            async move {
                iter.set(vec![]);
                let window = web_sys::window().expect("should have a window in this context");
                let performance = window
                    .performance()
                    .expect("performance should be available");
                let start = performance.now() as i64;
                let res = get_rust_singlethreaded().await;
                let end = performance.now() as i64;
                let dur = end - start;
                let dur = format!("Duration: {} ms", dur);
                duration_rust_single.set(dur);

                match res {
                    Ok(articles) => {
                        iter.set(articles);
                    }
                    Err(e) => {
                        console_log!("error calling server start_java_reference_impl   {:?}", e)
                    }
                }
            }
        });
    };

    let start_rust_multi_threaded = move |e: MouseEvent| {
        e.prevent_default();
        console_log!(
            "start_rust_multi_threaded  clicked.   event {:?}",
            e.target()
        );
        spawn_local_scoped(cx, {
            async move {
                iter.set(vec![]);
                let window = web_sys::window().expect("should have a window in this context");
                let performance = window
                    .performance()
                    .expect("performance should be available");
                let start = performance.now() as i64;
                let res = get_rust_multithreaded().await;
                let end = performance.now() as i64;
                let dur = end - start;
                let dur = format!("Duration: {} ms", dur);
                duration_rust_multi.set(dur);

                match res {
                    Ok(articles) => {
                        iter.set(articles);
                    }
                    Err(e) => {
                        console_log!("error calling server start_java_reference_impl   {:?}", e)
                    }
                }
            }
        });
    };

    let start_rust_rayon = move |e: MouseEvent| {
        e.prevent_default();
        console_log!("start_rust_rayon  clicked.   event {:?}", e.target());
        spawn_local_scoped(cx, {
            async move {
                iter.set(vec![]);
                let window = web_sys::window().expect("should have a window in this context");
                let performance = window
                    .performance()
                    .expect("performance should be available");
                let start = performance.now() as i64;
                let res = get_rust_rayon().await;
                let end = performance.now() as i64;
                let dur = end - start;
                let dur = format!("Duration: {} ms", dur);
                duration_rust_rayon.set(dur);

                match res {
                    Ok(articles) => {
                        iter.set(articles);
                    }
                    Err(e) => {
                        console_log!("error calling server start_java_reference_impl   {:?}", e)
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
                div(class="col") {
                    div(class="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom") {
                        h1 (class="h1"){
                            "Images"
                        }
                        div(class="btn-toolbar mb-2 mb-md-0"){
                            div(class="btn-group me-2"){
                                "bla"
                            }
                        }
                    }
                    div(class = "row", style ="margin-bottom: 10px;") {
                        div (class="col-12") {
                            button(class="btn btn-primary", type="button", id="java-ref-impl" ,on:click=start_java_reference_impl) {
                                "Java Reference implementation"
                            }

                            p(id = "java-ref-impl-p") {
                                (duration_java_ref_impl.get())
                            }
                        }
                    }

                    div(class = "row", style ="margin-bottom: 10px;") {
                        div (class="col-12") {
                            button(class="btn btn-primary", type="button", id="java-ref-impl" ,on:click=start_rust_single_threaded) {
                                "Rust Single Threaded (Warp)"
                            }

                            p(id = "java-ref-impl-p") {
                                (duration_rust_single.get())
                            }
                        }
                    }

                    div(class = "row", style ="margin-bottom: 10px;") {
                        div (class="col-12") {
                            button(class="btn btn-primary", type="button", id="java-ref-impl" ,on:click=start_rust_multi_threaded) {
                                "Rust Multi Threaded (Warp)"
                            }

                            p(id = "java-ref-impl-p") {
                                (duration_rust_multi.get())
                            }
                        }
                    }

                    div(class = "row", style ="margin-bottom: 10px;") {
                        div (class="col-12") {
                            button(class="btn btn-primary", type="button", id="java-ref-impl" ,on:click=start_rust_rayon) {
                                "Rust Rayon (Warp)"
                            }

                            p(id = "java-ref-impl-p") {
                                (duration_rust_rayon.get())
                            }
                        }
                    }

                     div(class = "row", style ="margin-bottom: 10px;") {
                        div (class="col-12") {
                            Keyed (
                                iterable = iter,
                                view =| cx, Article { code,title,  description , images , .. }  | view! { cx,
                                     div(class = "container-fluid") {
                                        div(class = "row") {
                                            div ( class = "col-12") {
                                                h4 {
                                                    (code) (title)
                                                }
                                                p {
                                                    (description)
                                                }
                                            }
                                        }
                                        ServerTargetStatsComp(stats = images)
                                     }
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
