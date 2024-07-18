// routes.rs
// Routes for the Dioxus Router

use dioxus::prelude::*;

use crate::components::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Routes {

    #[route("/")]
    Home {},

    #[route("/blog/:id")]
    Blog { id: i32 },

    #[route("/about")]
    About {},
}
