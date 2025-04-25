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



use std::collections::HashMap;

pub fn Form() -> Element {
    let mut values = use_signal(HashMap::new);
    let mut submitted_values = use_signal(HashMap::new);

    rsx! {
        div { style: "display: flex",
            div { style: "width: 50%",
                h1 { "Form" }

                if !submitted_values.read().is_empty() {
                    h2 { "Submitted! ✅" }
                }

                // The form element is used to create an HTML form for user input
                // You can attach regular attributes to it
                form {
                    id: "cool-form",
                    style: "display: flex; flex-direction: column;",

                    // You can attach a handler to the entire form
                    oninput: move |ev| {
                        println!("Input event: {:#?}", ev);
                        values.set(ev.values());
                    },

                    // On desktop/liveview, the form will not navigate the page - the expectation is that you handle
                    // The form event.
                    // However, if your form doesn't have a submit handler, it might navigate the page depending on the webview.
                    // We suggest always attaching a submit handler to the form.
                    onsubmit: move |ev| {
                        println!("Submit event: {:#?}", ev);
                        submitted_values.set(ev.values());
                    },

                    // Regular text inputs with handlers
                    label { r#for: "username", "Username" }
                    input {
                        r#type: "text",
                        name: "username",
                        oninput: move |ev| {
                            println!("setting username");
                            values.set(ev.values());
                        }
                    }

                    // And then the various inputs that might exist
                    // Note for a value to be returned in .values(), it must be named!

                    label { r#for: "full-name", "Full Name" }
                    input { r#type: "text", name: "full-name" }
                    input { r#type: "text", name: "full-name" }

                    label { r#for: "email", "Email (matching <name>@example.com)" }
                    input { r#type: "email", pattern: ".+@example\\.com", size: "30", required: "true", id: "email", name: "email" }

                    label { r#for: "password", "Password" }
                    input { r#type: "password", name: "password" }

                    label { r#for: "color", "Color" }
                    input { r#type: "radio", checked: true, name: "color", value: "red" }
                    input { r#type: "radio", name: "color", value: "blue" }
                    input { r#type: "radio", name: "color", value: "green" }

                    // Select multiple comes in as a comma separated list of selected values
                    // You should split them on the comma to get the values manually
                    label { r#for: "country", "Country" }
                    select {
                        name: "country",
                        multiple: true,
                        oninput: move |ev| {
                            println!("Input event: {:#?}", ev);
                            println!("Values: {:#?}", ev.value().split(',').collect::<Vec<_>>());
                        },
                        option { value: "usa",  "USA" }
                        option { value: "canada",  "Canada" }
                        option { value: "mexico",  "Mexico" }
                    }

                    // Safari can be quirky with color inputs on mac.
                    // We recommend always providing a text input for color as a fallback.
                    label { r#for: "color", "Color" }
                    input { r#type: "color", value: "#000002", name: "head", id: "head" }

                    // Dates!
                    input {
                        min: "2018-01-01",
                        value: "2018-07-22",
                        r#type: "date",
                        name: "trip-start",
                        max: "2025-12-31",
                        id: "start"
                    }

                    // CHekcboxes
                    label { r#for: "cbox", "Color" }
                    div {
                        label { r#for: "cbox-red", "red" }
                        input { r#type: "checkbox", checked: true, name: "cbox", value: "red", id: "cbox-red" }
                    }
                    div {
                        label { r#for: "cbox-blue", "blue" }
                        input { r#type: "checkbox", name: "cbox", value: "blue", id: "cbox-blue" }
                    }
                    div {
                        label { r#for: "cbox-green", "green" }
                        input { r#type: "checkbox", name: "cbox", value: "green", id: "cbox-green" }
                    }
                    div {
                        label { r#for: "cbox-yellow", "yellow" }
                        input { r#type: "checkbox", name: "cbox", value: "yellow", id: "cbox-yellow" }
                    }


                    // Buttons will submit your form by default.
                    button { r#type: "submit", value: "Submit", "Submit the form" }
                }
            }
            /*
            div { style: "width: 50%",
                h1 { "Oninput Values" }
                pre { "{values:#?}" }
            }
            */
        }
    }
}