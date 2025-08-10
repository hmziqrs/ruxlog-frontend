use dioxus::prelude::*;
use crate::components::PageHeader;
use crate::containers::{TagFormContainer, TagForm};
use crate::store::use_tag;
 

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
                        let payload = val.to_add_payload();
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
