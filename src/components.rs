// components.rs
// Dioxus components comprising the app

use dioxus::prelude::*;

use crate::routes::Routes;

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Routes> {}
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
pub fn About() -> Element {
    rsx! {
        Header {}

        div {
            id: "links",
            class: "row",

            Link { to: Routes::Home {}, "Go to counter" }
        }

        h2 { "About" }

        div { "This is a Dioxus template application." }

        div {
            r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Duis tristique sollicitudin nibh sit amet commodo nulla. Neque sodales ut etiam sit amet. Suscipit adipiscing bibendum est ultricies. Non tellus orci ac auctor augue mauris augue neque. Lectus urna duis convallis convallis tellus id interdum velit. Dictum varius duis at consectetur. Donec pretium vulputate sapien nec sagittis aliquam malesuada bibendum. In cursus turpis massa tincidunt dui ut ornare lectus. Lorem ipsum dolor sit amet. Vitae purus faucibus ornare suspendisse sed nisi lacus. Ornare quam viverra orci sagittis eu volutpat odio. Congue quisque egestas diam in arcu cursus euismod quis. Dignissim suspendisse in est ante in nibh mauris cursus mattis."#
        }
    }
}

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        Header {}

        div {
            id: "links",
            class: "row",

            Link { to: Routes::Home {}, "Go to counter" }
            Link { to: Routes::About {}, "About" }
        }
        
        h2 { "Blog" }

        "Blog post {id}"
    }
}

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Header {}

        div {
            id: "links",
            class: "row",

            Link {
                to: Routes::Blog {
                    id: count()
                },
                "Go to blog"
            }
    
            Link { to: Routes::About {}, "About" }
        }

        h2 { "High-Five counter: {count}" }

        div {
            class: "row",
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
