use dioxus::prelude::*;
use crate::components::urls::Route;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            id: "home",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist 💔." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

const NAV_CSS: Asset = asset!("/assets/styling/main.css");
#[component]
pub fn NavBar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAV_CSS },
        div {
            id: "nav",
            nav { id: "navbar",  Link { to: Route::Home {}, "Home" , }  }
            Outlet::<Route> {}
        }
    }
}


