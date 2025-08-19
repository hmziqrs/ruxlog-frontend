use dioxus::logger::tracing;
use dioxus::prelude::*;
use crate::components::PageHeader;
use crate::containers::{TagFormContainer, TagForm};
use crate::hooks::use_previous;
use crate::store::use_tag;
use crate::components::sonner::{use_sonner, ToastOptions, ToastType};
 

#[component]
pub fn TagsAddScreen() -> Element {
    let tags = use_tag();
    let sonner = use_sonner();
    let mut toast_id = use_signal::<Option<u64>>(|| None);


    let state = tags.add.read();
    let loading = state.is_loading();
    let success = state.is_success();
    let failed = state.is_failed();

    let prev_loading = use_previous(loading);

    use_effect(use_reactive!(|(loading,)| {
        if prev_loading.is_some() && prev_loading.unwrap_or_default() != loading {
            if toast_id().is_none() {
                let id = sonner.loading("Creating tag...".to_string(), ToastOptions::default().with_duration(None));
                toast_id.set(Some(id));
            } else {
                sonner.update_loading(toast_id().unwrap(), "Creating tag...".to_string(), ToastOptions::default().with_duration(None));
            }
            if success {
                sonner.update_success(toast_id().unwrap(), "Togs created successfully", ToastOptions::default());
            } else if failed {
                sonner.update_error(toast_id().unwrap(), "Failed to create tag", ToastOptions::default());
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
