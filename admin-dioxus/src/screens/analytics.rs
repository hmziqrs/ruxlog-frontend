use dioxus::prelude::*;

#[component]
pub fn AnalyticsScreen() -> Element {
    rsx! {
        div { class: "container mx-auto p-8 min-h-screen",
            h1 { class: "text-3xl font-bold mb-4 text-zinc-900 dark:text-zinc-100", "Analytics" }
            p { class: "text-zinc-600 dark:text-zinc-400", "This page is under construction." }
        }
    }
}