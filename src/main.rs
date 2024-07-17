// main.rs
// Dioxus Vanilla CSS Template

#![allow(non_snake_case)]

mod components;
mod routes;

use dioxus::prelude::*;
use tracing::Level;

use components::App;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let config = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="main.css">"#.to_string());

    LaunchBuilder::desktop().with_cfg(config).launch(App);
}
