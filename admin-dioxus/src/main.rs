use dioxus::{ prelude::*};

pub mod containers;
pub mod components;
pub mod hooks;
pub mod router;
pub mod screens;
pub mod store;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            "crossorigin": "",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Geist+Mono:wght@400..600&family=Geist:wght@400..600&display=swap",
        }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        Router::<crate::router::Route> {}
    }
}
