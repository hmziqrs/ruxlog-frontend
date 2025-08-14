use dioxus::prelude::*;

use crate::components::sonner::{use_sonner, ToastOptions};

#[component]
pub fn SonnerDemoScreen() -> Element {
    let sonner = use_sonner();

    rsx! {
        div { class: "p-6 space-y-4",
            h2 { class: "text-xl font-semibold", "Sonner Demo" }
            div { class: "space-x-2",
                button {
                    class: "px-3 py-2 rounded-md bg-green-600 text-white hover:bg-green-700",
                    onclick: move |_| {
                        sonner.success("Saved successfully".to_string(), ToastOptions::default());
                    },
                    "Show Success"
                }
                button {
                    class: "px-3 py-2 rounded-md bg-red-600 text-white hover:bg-red-700",
                    onclick: move |_| {
                        sonner.error("Something went wrong".to_string(), ToastOptions::default());
                    },
                    "Show Error"
                }
            }
            p { class: "text-sm text-muted-foreground", "Navigate away and back to test persistence; hover to test pause in Phase 3." }
        }
    }
}
