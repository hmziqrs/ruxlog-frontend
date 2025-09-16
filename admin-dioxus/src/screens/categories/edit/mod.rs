use dioxus::prelude::*;

use crate::components::{FormTwoColumnSkeleton, PageHeader};
use crate::containers::{CategoryForm, CategoryFormContainer};
use crate::hooks::use_category_view;
use crate::store::use_category;
use crate::ui::shadcn::Button;

#[component]
pub fn CategoriesEditScreen(id: i32) -> Element {
    let state = use_category_view(id);
    let is_loading = state.is_loading;
    let is_failed = state.is_failed;
    let message = state.message.clone();
    let cat_opt = state.category.clone();

    // Compute initial form state from loaded category
    let initial_form: Option<CategoryForm> = cat_opt.clone().map(|c| CategoryForm {
        name: c.name.clone(),
        slug: c.slug.clone(),
        description: c.description.unwrap_or_default(),
        color: c.color.clone(),
        custom_text_color: true, // respect existing text color; container computes contrast when not custom
        text_color: c.text_color.clone(),
        active: c.is_active,
        cover_image: c.cover_image.unwrap_or_default(),
        logo_image: c.logo_image.unwrap_or_default(),
        parent_id: c.parent_id.map(|v| v.to_string()).unwrap_or_default(),
    });

    rsx! {
        div { class: "min-h-screen bg-zinc-50 dark:bg-zinc-950 text-zinc-900 dark:text-zinc-50",
            PageHeader {
                title: "Edit Category".to_string(),
                description: "Update category details, colors, and visibility.".to_string(),
            }

            div { class: "container mx-auto px-4 py-10 md:py-12 space-y-4",
                if is_failed {
                    div { class: "rounded-md border border-red-200 bg-red-50 p-3 text-red-700 dark:border-red-900/40 dark:bg-red-900/20 dark:text-red-300",
                        span { class: "text-sm", "Failed to load category." }
                        if let Some(msg) = message { span { class: "ml-1 text-sm opacity-80", "{msg}" } }
                        Button { class: "ml-3", onclick: move |_| { spawn({ let cats = use_category(); async move { cats.view(id).await; }}); }, "Retry" }
                    }
                }

                if is_loading && initial_form.is_none() {
                    FormTwoColumnSkeleton {}
                } else if let Some(initial) = initial_form.clone() {
                    CategoryFormContainer {
                        title: Some("Edit Category".to_string()),
                        submit_label: Some("Save Changes".to_string()),
                        initial: Some(initial.clone()),
                        on_submit: move |val: CategoryForm| {
                            let payload = val.to_edit_payload();
                            let cats = use_category();
                            spawn(async move {
                                cats.edit(id, payload).await;
                            });
                        },
                    }
                }
            }
        }
    }
}
