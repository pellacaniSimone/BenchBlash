use dioxus::prelude::*;
use crate::components::ui::templates::{PageNotFound,Form,Home,NavBar,Home2};

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")] Home2 {},
    #[route("/form")] Form {},
    #[route("/:..route")] PageNotFound { route: Vec<String>, },
} 