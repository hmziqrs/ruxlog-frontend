use dioxus::prelude::*;

use crate::components::PageHeader;
use crate::containers::{CategoryForm, CategoryFormContainer};
use crate::store::use_categories;

#[component]
pub fn CategoriesAddScreen() -> Element {
    let categories = use_categories();

    rsx! {
        // Page wrapper
        div { class: "min-h-screen bg-zinc-50 dark:bg-zinc-950 text-zinc-900 dark:text-zinc-50",
            // Header
            PageHeader {
                title: "Create Category".to_string(),
                description: "Define category details, colors, and visibility.".to_string(),
            }

            // Content
            div { class: "container mx-auto px-4 py-10 md:py-12",
                CategoryFormContainer {
                    title: Some("New Category".to_string()),
                    submit_label: Some("Create Category".to_string()),
                    on_submit: move |val: CategoryForm| {
                        let payload = val.to_add_payload();
                        let categories = categories;
                        spawn(async move {
                            categories.add(payload).await;
                        });
                    },
                }
            }
        }
    }
}
