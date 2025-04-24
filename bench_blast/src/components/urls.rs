use dioxus::prelude::*;
use crate::components::base_view::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")] Home {},
    #[route("/:..route")] PageNotFound { route: Vec<String>, },
} 