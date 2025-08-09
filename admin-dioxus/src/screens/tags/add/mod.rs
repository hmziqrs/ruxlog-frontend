use dioxus::prelude::*;
use crate::ui::shadcn::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use crate::containers::{TagFormContainer, TagForm};
use crate::store::{use_tag, TagsAddPayload};

#[component]
pub fn TagsAddScreen() -> Element {
    let tags = use_tag();

    // Helpers to compute contrast for text_color
    fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
        let hex = hex.trim().trim_start_matches('#');
        let full = match hex.len() {
            3 => {
                let mut s = String::with_capacity(6);
                for c in hex.chars() { s.push(c); s.push(c); }
                s
            }
            6 => hex.to_string(),
            _ => return None,
        };
        u32::from_str_radix(&full, 16).ok().map(|val| {
            let r = ((val >> 16) & 0xFF) as u8;
            let g = ((val >> 8) & 0xFF) as u8;
            let b = (val & 0xFF) as u8;
            (r, g, b)
        })
    }
    fn get_contrast_yiq(hex: &str) -> &'static str {
        if let Some((r, g, b)) = hex_to_rgb(hex) {
            let yiq = (r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000;
            if yiq >= 128 { "#111111" } else { "#ffffff" }
        } else {
            "#111111"
        }
    }

    rsx! {
        // Page wrapper
        div { class: "min-h-screen bg-zinc-50 dark:bg-zinc-950 text-zinc-900 dark:text-zinc-50",
            // Top region with breadcrumb and header
            div { class: "border-b border-zinc-200 dark:border-zinc-800 bg-gradient-to-b from-zinc-50/60 to-transparent dark:from-zinc-950/40",
                div { class: "container mx-auto px-4 py-6 md:py-8",
                    // Breadcrumb
                    Breadcrumb {
                        BreadcrumbList {
                            BreadcrumbItem {
                                BreadcrumbLink { href: "/dashboard".to_string(), "Dashboard" }
                            }
                            BreadcrumbSeparator {}
                            BreadcrumbItem {
                                BreadcrumbLink { href: "/dashboard/tags".to_string(), "Tags" }
                            }
                            BreadcrumbSeparator {}
                            BreadcrumbItem { BreadcrumbPage { "New" } }
                        }
                    }

                    // Header row
                    div { class: "mt-4 flex flex-col items-start justify-between gap-4 md:flex-row md:items-center",
                        div { class: "space-y-1",
                            h1 { class: "text-3xl md:text-4xl font-bold tracking-tight", "Create Tag" }
                            p { class: "text-sm md:text-base text-zinc-600 dark:text-zinc-400",
                                "Define how your tag looks and behaves. Keep names concise and meaningful."
                            }
                        }
                        div { class: "flex items-center gap-2" }
                    }
                }
            }

            // Content: render reusable form component; submission handled here
            div { class: "container mx-auto px-4 py-8",
                TagFormContainer {
                    title: Some("New Tag".to_string()),
                    submit_label: Some("Create Tag".to_string()),
                    on_submit: move |val: TagForm| {
                        let description = if val.description.trim().is_empty() { None } else { Some(val.description.clone()) };
                        let color = if val.color.trim().is_empty() { None } else { Some(val.color.clone()) };
                        let text_color = Some(get_contrast_yiq(&val.color).to_string());
                        let payload = TagsAddPayload {
                            name: val.name.clone(),
                            slug: val.slug.clone(),
                            description,
                            color,
                            text_color,
                            is_active: Some(val.active),
                        };
                        let tags = tags;
                        spawn(async move {
                            tags.add(payload).await;
                        });
                    },
                }
            }
        }
    }
}
