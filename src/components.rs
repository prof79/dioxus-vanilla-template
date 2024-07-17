// components.rs
// Dioxus components comprising the app

use dioxus::prelude::*;

use crate::routes::Routes;

#[component]
pub fn App() -> Element {
    rsx! {
        div {
            id: "links",

            Router::<Routes> {}
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
pub fn Blog(id: i32) -> Element {
    rsx! {
        Header {}

        Link { to: Routes::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Header {}

        Link {
            to: Routes::Blog {
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
