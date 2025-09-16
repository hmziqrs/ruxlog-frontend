use dioxus::prelude::*;

use crate::components::{use_toast, ToastOptions};
use crate::ui::shadcn::Button;

#[component]
pub fn AnalyticsScreen() -> Element {
    let toast = use_toast();

    rsx! {
        div { class: "container mx-auto p-8 min-h-screen",
            h1 { class: "text-3xl font-bold mb-4 text-zinc-900 dark:text-zinc-100", "Analytics" }
            p { class: "text-zinc-600 dark:text-zinc-400", "This page is under construction." }
            Button {
                onclick: move |_| {
                    toast.info("API error".to_string(), ToastOptions::new().description("Please try again later lmao.".to_string()));
                },
                "Show Toast"
            }
        }
    }
}
