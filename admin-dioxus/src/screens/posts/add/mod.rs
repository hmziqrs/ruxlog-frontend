use dioxus::prelude::*;

use crate::components::PageHeader;
use crate::containers::BlogFormContainer;
use crate::router::Route;
use crate::ui::shadcn::{Button, ButtonVariant};

#[component]
pub fn PostsAddScreen() -> Element {
    let nav = use_navigator();

    rsx! {
        // Page wrapper
        div { class: "min-h-screen bg-transparent text-foreground",
            // Unified autonomous header
            PageHeader {
                title: "Create Post".to_string(),
                description: "Write and publish your blog post. Add rich content, images, and categorize your post.".to_string(),
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
                BlogFormContainer { post_id: None }
            }
        }
    }
}
