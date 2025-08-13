use components::ToastManager;
use dioxus::prelude::*;

use crate::components::ToastProvider;

pub mod components;
mod config;
pub mod containers;
pub mod env;
pub mod hooks;
pub mod router;
pub mod screens;
pub mod services;
pub mod store;
pub mod ui;
pub mod utils;

fn main() {
    dioxus::launch(App);
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    // let toast = use_context_provider(|| Signal::new(ToastManager::default()));

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
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
        // document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        ToastProvider {
            Router::<crate::router::Route> {}
        }
    }
}
// ToastFrame component is temporarily commented out due to compatibility issues
// dioxus_toast::ToastFrame { manager: toast, style: None }
