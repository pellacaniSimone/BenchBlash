use dioxus::prelude::*;
use crate::components::templates::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")] Home {},
    #[route("/form")] Form {},
    #[route("/:..route")] PageNotFound { route: Vec<String>, },
} 