use dioxus::prelude::*;
use crate::components::sonner::{Action, ToastOptions};
use crate::components::PageHeader;
use crate::containers::{TagFormContainer, TagForm};
use crate::store::use_tag;
use crate::hooks::{use_state_frame_toast, StateFrameToastConfig};
 

#[component]
pub fn TagsAddScreen() -> Element {
    let tags = use_tag();
    // Wire StateFrame->Sonner toast for add flow
    let cfg = StateFrameToastConfig {
        loading_title: "Creating tag...".into(),
        success_title: Some("Tag created successfully".into()),
        error_title: Some("Failed to create tag".into()),
        error_options: ToastOptions::default().with_action(Some(Action::with_on_click("Retry".into(), Callback::new(move |_| {
            //
        })))), 
        ..Default::default()
    };
    use_state_frame_toast(&tags.add, cfg);


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
