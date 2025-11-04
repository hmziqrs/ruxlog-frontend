use dioxus::prelude::*;

use crate::components::PageHeader;
use crate::containers::BlogFormContainer;
use crate::router::Route;
use crate::ui::shadcn::{Button, ButtonVariant};

#[component]
pub fn PostsEditScreen(id: i32) -> Element {
    let nav = use_navigator();

    rsx! {
        // Page wrapper
        div { class: "min-h-screen bg-transparent text-foreground",
            // Unified autonomous header
            PageHeader {
                title: "Edit Post".to_string(),
                description: "Update your post content, metadata, and publishing settings.".to_string(),
                actions: Some(rsx! {
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| { nav.push(Route::PostsListScreen {}); },
                        "Back to Posts"
                    }
                }),
            }

            // Content: render blog form container
            div { class: "container mx-auto px-4 py-10 md:py-12",
                BlogFormContainer { post_id: Some(id) }
            }
        }
    }
}
