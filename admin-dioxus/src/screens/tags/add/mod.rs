use dioxus::prelude::*;
use crate::components::PageHeader;
use crate::containers::{TagFormContainer, TagForm};
use crate::store::{use_tag, TagsAddPayload};
use crate::utils::colors::get_contrast_yiq;

#[component]
pub fn TagsAddScreen() -> Element {
    let tags = use_tag();


    rsx! {
        // Page wrapper
        div { class: "min-h-screen bg-zinc-50 dark:bg-zinc-950 text-zinc-900 dark:text-zinc-50",
            // Unified autonomous header
            PageHeader {
                title: "Create Tag".to_string(),
                description: "Define how your tag looks and behaves. Keep names concise and meaningful.".to_string(),
            }

            // Content: render reusable form component; submission handled here
            div { class: "container mx-auto px-4 py-10 md:py-12",
                TagFormContainer {
                    title: Some("New Tag".to_string()),
                    submit_label: Some("Create Tag".to_string()),
                    on_submit: move |val: TagForm| {
                        let description = if val.description.trim().is_empty() { None } else { Some(val.description.clone()) };
                        let color = if val.color.trim().is_empty() { None } else { Some(val.color.clone()) };
                        let text_color = if val.custom_text_color && !val.text_color.trim().is_empty() {
                            Some(val.text_color.clone())
                        } else {
                            Some(get_contrast_yiq(&val.color).to_string())
                        };
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
