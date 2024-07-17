// routes.rs
// Routes for the Dioxus Router

use dioxus::prelude::*;

use crate::components::{Home, Blog};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Routes {

    #[route("/")]
    Home {},

    #[route("/blog/:id")]
    Blog { id: i32 },
}
