#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let config = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="main.css">"#.to_string());

    LaunchBuilder::desktop().with_cfg(config).launch(App);

    //dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            id: "links",

            Router::<Route> {}
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        div {
            id: "header",

            img {
                src: "header.svg",
                alt: "Header",
                border: 0,
            }
        }
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Header {}

        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Header {}

        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
