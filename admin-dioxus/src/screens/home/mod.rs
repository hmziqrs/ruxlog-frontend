use dioxus::prelude::*;

use crate::components::PageHeader;

#[component]
pub fn HomeScreen() -> Element {
    rsx! {
        div { class: "min-h-screen bg-transparent text-foreground",
            // Consistent page header for Dashboard
            PageHeader {
                title: "Dashboard".to_string(),
                description: "Overview of your blog performance, content, and activity.".to_string(),
            }

            // Clean dashboard shell for future analytics/insights
            div { class: "container mx-auto px-4 pb-10",
                div {
                    class: "mt-4 grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4",
                    PlaceholderCard { label: "Key metrics" }
                    PlaceholderCard { label: "Traffic & engagement" }
                    PlaceholderCard { label: "Content performance" }
                    PlaceholderCard { label: "User activity" }
                }

                div { class: "mt-6 grid grid-cols-1 lg:grid-cols-2 gap-4",
                    PlaceholderPanel { label: "Traffic & views summary" }
                    PlaceholderPanel { label: "Top posts & categories" }
                }
            }
        }
    }
}

#[component]
fn PlaceholderCard(label: &'static str) -> Element {
    rsx! {
        div {
            class: "h-24 rounded-xl border border-zinc-200/80 dark:border-zinc-800/80 \
                    bg-zinc-100/40 dark:bg-zinc-950/40 \
                    flex items-center px-4 text-xs text-zinc-500 \
                    shadow-sm backdrop-blur-sm",
            span { "{label} will appear here" }
        }
    }
}

#[component]
fn PlaceholderPanel(label: &'static str) -> Element {
    rsx! {
        div {
            class: "h-56 rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 \
                    bg-zinc-100/40 dark:bg-zinc-950/40 \
                    flex items-center px-6 text-sm text-zinc-500 \
                    shadow-sm backdrop-blur-sm",
            span { "{label} will appear here" }
        }
    }
}
