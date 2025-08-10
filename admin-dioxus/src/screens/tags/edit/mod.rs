use dioxus::prelude::*;

use crate::components::PageHeader;
use crate::containers::{TagForm, TagFormContainer};
use crate::store::{use_tag, TagsEditPayload};
use crate::utils::colors::get_contrast_yiq;
use crate::ui::shadcn::Button;

#[component]
pub fn TagsEditScreen(id: i32) -> Element {
    let tags = use_tag();
    // Kick off fetch of the tag if not present or not loading
    {
        let tags = tags;
        use_effect(move || {
            let view_map = tags.view.read();
            let needs_fetch = match view_map.get(&id) {
                None => true,
                Some(frame) => frame.is_init(),
            };
            if needs_fetch {
                spawn({
                    let tags = use_tag();
                    async move { tags.view(id).await; }
                });
            }
        });
    }

    let view_map = tags.view.read();
    let frame = view_map.get(&id);
    let is_loading = frame.map(|f| f.is_loading()).unwrap_or(true);
    let is_failed = frame.map(|f| f.is_failed()).unwrap_or(false);
    let message = frame.and_then(|f| f.message.clone());
    let tag_opt = frame.and_then(|f| f.data.clone()).flatten();

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
                    // Skeletons for header and form when no data yet
                    div { class: "grid grid-cols-1 gap-10 lg:grid-cols-3",
                        div { class: "lg:col-span-2 space-y-8",
                            div { class: "rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 shadow-sm p-6",
                                div { class: "h-5 w-40 rounded bg-muted animate-pulse" }
                                div { class: "mt-4 space-y-3",
                                    div { class: "h-9 w-full rounded bg-muted animate-pulse" }
                                    div { class: "h-9 w-1/2 rounded bg-muted animate-pulse" }
                                    div { class: "h-32 w-full rounded bg-muted animate-pulse" }
                                }
                            }
                        }
                        div { class: "space-y-8",
                            div { class: "rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 shadow-sm p-6 space-y-4",
                                div { class: "h-5 w-24 rounded bg-muted animate-pulse" }
                                div { class: "h-9 w-full rounded bg-muted animate-pulse" }
                                div { class: "h-9 w-full rounded bg-muted animate-pulse" }
                            }
                            div { class: "rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 shadow-sm p-6 space-y-4",
                                div { class: "h-5 w-24 rounded bg-muted animate-pulse" }
                                div { class: "h-9 w-full rounded bg-muted animate-pulse" }
                            }
                            div { class: "flex gap-3",
                                div { class: "h-9 w-24 rounded bg-muted animate-pulse" }
                                div { class: "h-9 w-28 rounded bg-muted animate-pulse" }
                            }
                        }
                    }
                } else if let Some(initial) = initial_form.clone() {
                    TagFormContainer {
                        title: Some("Edit Tag".to_string()),
                        submit_label: Some("Save Changes".to_string()),
                        initial: Some(initial.clone()),
                        on_submit: move |val: TagForm| {
                            let description = if val.description.trim().is_empty() { None } else { Some(val.description.clone()) };
                            let color = if val.color.trim().is_empty() { None } else { Some(val.color.clone()) };
                            let text_color = if val.custom_text_color && !val.text_color.trim().is_empty() {
                                Some(val.text_color.clone())
                            } else {
                                Some(get_contrast_yiq(&val.color).to_string())
                            };
                            let payload = TagsEditPayload {
                                name: Some(val.name.clone()),
                                slug: Some(val.slug.clone()),
                                description,
                                color,
                                text_color,
                                is_active: Some(val.active),
                            };
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

