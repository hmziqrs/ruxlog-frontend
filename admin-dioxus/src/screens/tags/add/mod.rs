use dioxus::prelude::*;
use crate::components::PageHeader;
use crate::containers::{TagFormContainer, TagForm};
use crate::hooks::use_previous;
use crate::store::use_tag;
use crate::components::sonner::{use_sonner, ToastOptions, SonnerToaster, PromiseConfig};
 

#[component]
pub fn TagsAddScreen() -> Element {
    let tags = use_tag();
    let sonner = use_sonner();


    let state = tags.add.read();
    let loading = state.is_loading();
    let success = state.is_success();
    let failed = state.is_failed();

    let prev_loading = use_previous(loading);

    use_effect(use_reactive!(|(loading,)| {
        if prev_loading != Some(loading) {
            if success {
                sonner.success("Tag created successfully".to_string(), ToastOptions::default());
            } else if failed {
                sonner.error("Failed to create tag".to_string(), ToastOptions::default());
            }
        }
    }));


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
