use dioxus::prelude::*;

use crate::components::{FormTwoColumnSkeleton, PageHeader};
use crate::containers::{TagForm, TagFormContainer};
use crate::hooks::use_tag_view;
use crate::store::use_tag;
use crate::ui::shadcn::Button;

#[component]
pub fn TagsEditScreen(id: i32) -> Element {
    let state = use_tag_view(id);
    let is_loading = state.is_loading;
    let is_failed = state.is_failed;
    let message = state.message.clone();
    let tag_opt = state.tag.clone();

    // Compute initial form state from loaded tag
    let initial_form: Option<TagForm> = tag_opt.clone().map(|t| TagForm {
        name: t.name.clone(),
        slug: t.slug.clone(),
        description: t.description.unwrap_or_default(),
        color: t.color.clone(),
        custom_text_color: true, // assume custom color initially; we compute default contrast on submit if not
        text_color: t.text_color.clone(),
        active: t.is_active,
    });

    rsx! {
        div { class: "min-h-screen bg-zinc-50 dark:bg-zinc-950 text-zinc-900 dark:text-zinc-50",
            PageHeader {
                title: "Edit Tag".to_string(),
                description: "Update tag details, colors, and visibility.".to_string(),
            }

            div { class: "container mx-auto px-4 py-10 md:py-12 space-y-4",
                if is_failed {
                    div { class: "rounded-md border border-red-200 bg-red-50 p-3 text-red-700 dark:border-red-900/40 dark:bg-red-900/20 dark:text-red-300",
                        span { class: "text-sm", "Failed to load tag." }
                        if let Some(msg) = message { span { class: "ml-1 text-sm opacity-80", "{msg}" } }
                        Button { class: "ml-3", onclick: move |_| { spawn({ let tags = use_tag(); async move { tags.view(id).await; }}); }, "Retry" }
                    }
                }

                if is_loading && initial_form.is_none() {
                    FormTwoColumnSkeleton {}
                } else if let Some(initial) = initial_form.clone() {
                    TagFormContainer {
                        title: Some("Edit Tag".to_string()),
                        submit_label: Some("Save Changes".to_string()),
                        initial: Some(initial.clone()),
                        on_submit: move |val: TagForm| {
                            let payload = val.to_edit_payload();
                            let tags = use_tag();
                            spawn(async move {
                                tags.edit(id, payload).await;
                            });
                        },
                    }
                }
            }
        }
    }
}
