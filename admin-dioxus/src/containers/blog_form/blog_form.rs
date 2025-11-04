use dioxus::{logger::tracing, prelude::*};

use super::form::{use_blog_form, BlogForm};
// use crate::components::editor::RichTextEditor; // Moved to legacy - using TypeScript editor instead
use crate::components::{AppInput, EditorJsHost};
use crate::router::Route;
use crate::store::{
    use_categories, use_post, use_tag, PostAutosavePayload, PostCreatePayload, PostEditPayload,
    PostStatus,
};
use crate::ui::shadcn::{Button, ButtonVariant};
use chrono::Utc;
use dioxus_time::sleep;
use std::time::Duration;
use wasm_bindgen::{closure::Closure, JsCast};

#[component]
pub fn BlogFormContainer(post_id: Option<i32>) -> Element {
    let posts = use_post();
    let categories = use_categories();
    let tags = use_tag();
    let nav = use_navigator();

    // Initialize form with existing post data if editing
    let mut initial_form = use_signal(|| None::<BlogForm>);
    let is_edit_mode = post_id.is_some();

    // Fetch existing post if editing
    use_effect(move || {
        if let Some(id) = post_id {
            spawn(async move {
                posts.view_by_id(id).await;
            });
        }
    });

    // Fetch categories and tags
    use_effect(move || {
        spawn(async move {
            categories.list().await;
            tags.list().await;
        });
    });

    // Populate form when post data is loaded
    use_effect(move || {
        if let Some(id) = post_id {
            let view_frame = posts.view.read();
            if let Some(post_frame) = view_frame.get(&id) {
                if let Some(post) = &post_frame.data {
                    let form = BlogForm {
                        title: post.title.clone(),
                        content: post.content.clone(),
                        slug: post.slug.clone(),
                        excerpt: post.excerpt.clone().unwrap_or_default(),
                        featured_image_url: post
                            .featured_image
                            .as_ref()
                            .map(|m| m.file_url.clone())
                            .unwrap_or_default(),
                        is_published: post.status == PostStatus::Published,
                        category_id: Some(post.category.id),
                        tag_ids: post.tags.iter().map(|t| t.id).collect(),
                    };
                    initial_form.set(Some(form));
                }
            }
        } else {
            // New post: prefill content from localStorage draft if present
            let mut form = BlogForm::new();
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(draft)) = storage.get_item("blog_form_draft_content") {
                        form.content = draft;
                    }
                }
            }
            initial_form.set(Some(form));
        }
    });

    // Wait for form to be initialized
    let form_data = initial_form.read();
    if form_data.is_none() {
        return rsx! {
            div { class: "flex items-center justify-center min-h-[400px]",
                div { class: "text-center",
                    div { class: "inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]" }
                    p { class: "mt-4 text-sm text-muted-foreground", "Loading..." }
                }
            }
        };
    }

    let blog_form_hook = use_blog_form(form_data.clone().unwrap());
    let mut form = blog_form_hook.form;
    let mut auto_slug = blog_form_hook.auto_slug;
    let autosave_gen = use_signal(|| 0u64);
    let mut listener_handle =
        use_signal(|| None::<(web_sys::EventTarget, Closure<dyn FnMut(web_sys::Event)>)>);
    let mut form_signal = form;
    let mut autosave_signal = autosave_gen;
    let posts_store = posts;
    let post_id_value = post_id;

    use_effect(move || {
        if let Some((event_target, listener)) = listener_handle.write().take() {
            let _ = event_target.remove_event_listener_with_callback(
                "editor:change",
                listener.as_ref().unchecked_ref(),
            );
        }

        if let Some(window) = web_sys::window() {
            let event_target: web_sys::EventTarget = window.clone().into();
            let listener = Closure::wrap(Box::new(move |event: web_sys::Event| {
                if let Ok(custom_event) = event.dyn_into::<web_sys::CustomEvent>() {
                    if let Some(detail) = custom_event.detail().as_string() {
                        form_signal.write().update_field("content", detail.clone());

                        if let Some(window) = web_sys::window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                let _ = storage.set_item("blog_form_draft_content", &detail);
                            }
                        }

                        if let Some(edit_id) = post_id_value {
                            let this_tick = autosave_signal() + 1;
                            autosave_signal.set(this_tick);
                            let posts_ref = posts_store;
                            let debounce_signal = autosave_signal;
                            let content_for_save = detail.clone();
                            spawn(async move {
                                sleep(Duration::from_millis(1500)).await;
                                if debounce_signal() != this_tick {
                                    return;
                                }
                                posts_ref
                                    .autosave(PostAutosavePayload {
                                        post_id: edit_id,
                                        content: content_for_save,
                                        updated_at: Utc::now(),
                                    })
                                    .await;
                            });
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);

            if event_target
                .add_event_listener_with_callback(
                    "editor:change",
                    listener.as_ref().unchecked_ref(),
                )
                .is_ok()
            {
                listener_handle.set(Some((event_target, listener)));
            } else {
                listener.forget();
            }
        }
    });

    // Handle successful submission
    use_effect(move || {
        let add_state = posts.add.read();
        let edit_state = if let Some(id) = post_id {
            posts.edit.read().get(&id).cloned()
        } else {
            None
        };

        if add_state.is_success() || edit_state.as_ref().map_or(false, |s| s.is_success()) {
            spawn(async move {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        let _ = storage.remove_item("blog_form_draft_content");
                    }
                }
                nav.push(Route::PostsListScreen {});
            });
        }
    });

    rsx! {
        div {
            h1 { class: "sr-only",
                if is_edit_mode { "Edit Blog Post" } else { "New Blog Post" }
            }

            // Show error message if submission failed
            {
                let add_state = posts.add.read();
                let edit_state = if let Some(id) = post_id {
                    posts.edit.read().get(&id).cloned()
                } else {
                    None
                };

                let is_failed = add_state.is_failed() || edit_state.as_ref().map_or(false, |s| s.is_failed());
                let error_message = add_state.message.clone().or_else(|| edit_state.as_ref().and_then(|s| s.message.clone()));

                if is_failed {
                    rsx! {
                        div { class: "rounded-md border border-red-200 bg-red-50 p-4 mb-6 text-red-700 dark:border-red-900/40 dark:bg-red-900/20 dark:text-red-300",
                            div { class: "flex items-start gap-3",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    class: "h-5 w-5 flex-shrink-0",
                                    view_box: "0 0 20 20",
                                    fill: "currentColor",
                                    path {
                                        fill_rule: "evenodd",
                                        d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.28 7.22a.75.75 0 00-1.06 1.06L8.94 10l-1.72 1.72a.75.75 0 101.06 1.06L10 11.06l1.72 1.72a.75.75 0 101.06-1.06L11.06 10l1.72-1.72a.75.75 0 00-1.06-1.06L10 8.94 8.28 7.22z",
                                        clip_rule: "evenodd"
                                    }
                                }
                                span { class: "text-sm font-medium",
                                    {error_message.unwrap_or_else(|| "Failed to save post".to_string())}
                                }
                            }
                        }
                    }
                } else {
                    rsx! { }
                }
            }

            div { class: "rounded-xl border border-border/70 bg-transparent",
                div { class: "px-6 py-6 space-y-6",
                    form {
                        class: "space-y-6",
                        onsubmit: move |e| {
                            e.prevent_default();
                        },

                    // Title field
                    AppInput {
                        name: "title",
                        form,
                        label: "Title",
                        placeholder: "Post title",
                        onblur: move |_| {
                            if !*auto_slug.read() && !is_edit_mode {
                                let title_value = form.peek().get_field("title").unwrap().value.clone();
                                let sanitized = BlogForm::sanitize_slug(&title_value);
                                form.write().update_field("slug", sanitized);
                            }
                        },
                    }

                    // Slug field with auto-generate option
                    div { class: "space-y-2",
                        div { class: "flex justify-between items-center",
                            label { class: "block text-sm font-medium text-foreground", "Slug" }
                            if !is_edit_mode {
                                div { class: "flex items-center gap-2",
                                    span { class: "text-sm text-muted-foreground", "Auto-generate" }
                                    div { class: "flex items-center",
                                        input {
                                            class: "h-4 w-4 rounded border-border/70 bg-transparent text-primary focus:ring-2 focus:ring-ring/40",
                                            r#type: "checkbox",
                                            checked: *auto_slug.read(),
                                            onclick: move |_| {
                                                let current = *auto_slug.peek();
                                                auto_slug.set(!current);
                                            },
                                        }
                                    }
                                }
                            }
                        }
                        AppInput {
                            form,
                            name: "slug",
                            r#type: "text",
                            readonly: *auto_slug.read() && !is_edit_mode,
                            placeholder: "Post slug",
                        }
                    }

                    // Category selection
                    div { class: "space-y-2",
                        label { class: "block text-sm font-medium text-foreground",
                            "Category "
                            span { class: "text-red-500", "*" }
                        }
                        select {
                            class: "w-full rounded-md border border-border/70 bg-transparent px-3 py-2 text-sm text-foreground focus:border-ring focus:ring-2 focus:ring-ring/40 transition-colors",
                            required: true,
                            value: form.read().data.category_id.map(|id| id.to_string()).unwrap_or_default(),
                            onchange: move |event| {
                                if let Ok(id) = event.value().parse::<i32>() {
                                    form.write().data.category_id = Some(id);
                                }
                            },
                            option { value: "", disabled: true, selected: form.read().data.category_id.is_none(),
                                "Select a category"
                            }
                            for category in categories.list.read().data.as_ref().map(|d| &d.data).unwrap_or(&vec![]) {
                                option {
                                    key: "{category.id}",
                                    value: "{category.id}",
                                    selected: form.read().data.category_id == Some(category.id),
                                    "{category.name}"
                                }
                            }
                        }
                    }

                    // Tags selection
                    div { class: "space-y-2",
                        label { class: "block text-sm font-medium text-foreground", "Tags" }
                        {
                            let tags_list = tags.list.read();
                            tags_list.data.as_ref().map(|tags_data| {
                                rsx! {
                                    div { class: "flex flex-wrap gap-2 p-4 border border-border/70 rounded-lg bg-transparent",
                                        for tag in &tags_data.data {
                                            {
                                                let tag_id = tag.id;
                                                let tag_name = tag.name.clone();
                                                rsx! {
                                                    label {
                                                        key: "{tag_id}",
                                                        class: "flex items-center gap-2 cursor-pointer",
                                                        input {
                                                            r#type: "checkbox",
                                                            class: "h-4 w-4 rounded border-border/70 bg-transparent text-primary focus:ring-2 focus:ring-ring/40",
                                                            checked: form.read().data.tag_ids.contains(&tag_id),
                                                            onchange: move |_| {
                                                                let mut form_write = form.write();
                                                                if form_write.data.tag_ids.contains(&tag_id) {
                                                                    form_write.data.tag_ids.retain(|&id| id != tag_id);
                                                                } else {
                                                                    form_write.data.tag_ids.push(tag_id);
                                                                }
                                                            },
                                                        }
                                                        span { class: "text-sm text-foreground", "{tag_name}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }).unwrap_or_else(|| rsx! {
                                div { class: "text-sm text-muted-foreground", "Loading tags..." }
                            })
                        }
                    }

                    // Excerpt field
                    div { class: "space-y-2",
                        label { class: "block text-sm font-medium text-foreground", "Excerpt" }
                        textarea {
                            class: "w-full h-24 resize-none rounded-md border border-border/70 bg-transparent px-4 py-3 text-sm text-foreground placeholder:text-muted-foreground transition-colors duration-200 focus:border-ring focus:ring-2 focus:ring-ring/40",
                            placeholder: "Brief description of the post",
                            rows: "3",
                            value: form.read().data.excerpt.clone(),
                            oninput: move |event| {
                                form.write().update_field("excerpt", event.value());
                            },
                        }
                    }

                    // Featured image URL field
                    AppInput {
                        name: "featured_image_url",
                        form,
                        label: "Featured Image URL",
                        placeholder: "https://example.com/image.jpg",
                    }

                    // Published status switch
                    div { class: "flex flex-row items-center justify-between border border-border/70 rounded-lg p-4 bg-transparent",
                        div { class: "space-y-0.5",
                            label { class: "text-base font-medium text-foreground", "Publish" }
                            div { class: "text-sm text-muted-foreground", "Make this post publicly available" }
                        }
                        div { class: "flex items-center",
                            input {
                                class: "relative h-6 w-11 cursor-pointer appearance-none rounded-full bg-border/50 transition-colors duration-200 checked:bg-primary focus:ring-2 focus:ring-ring/40 before:pointer-events-none before:absolute before:h-5 before:w-5 before:translate-x-0.5 before:translate-y-0.5 before:rounded-full before:bg-white before:shadow-sm before:transition-transform before:duration-200 checked:before:translate-x-5",
                                r#type: "checkbox",
                                checked: form.read().data.is_published,
                                onchange: move |event| {
                                    form.write().data.is_published = event.checked();
                                },
                            }
                        }
                    }

                    // Content field
                    {
                        let content_value = {
                            let reader = form.read();
                            reader.data.content.clone()
                        };

                        let initial_json = if content_value.trim().is_empty() {
                            None
                        } else {
                            Some(content_value)
                        };

                        rsx! {
                            div { class: "space-y-2",
                                label { class: "block text-sm font-medium text-foreground",
                                    "Content "
                                    span { class: "text-red-500", "*" }
                                }
                                EditorJsHost { initial_json }
                            }
                        }
                    }

                    // Form actions
                    div { class: "flex justify-end gap-4 pt-4",
                        Button {
                            variant: ButtonVariant::Ghost,
                            onclick: move |_| {
                                nav.push(Route::PostsListScreen {});
                            },
                            "Cancel"
                        }
                        Button {
                            disabled: {
                                let add_state = posts.add.read();
                                let edit_state = if let Some(id) = post_id {
                                    posts.edit.read().get(&id).cloned()
                                } else {
                                    None
                                };
                                add_state.is_loading() || edit_state.as_ref().map_or(false, |s| s.is_loading())
                            },
                            onclick: move |_| {
                                let form_data = form.read();

                                // Validate required fields
                                if form_data.data.title.is_empty() {
                                    tracing::error!("Title is required");
                                    return;
                                }
                                if form_data.data.content.is_empty() {
                                    tracing::error!("Content is required");
                                    return;
                                }
                                if form_data.data.slug.is_empty() {
                                    tracing::error!("Slug is required");
                                    return;
                                }
                                if form_data.data.category_id.is_none() {
                                    tracing::error!("Category is required");
                                    return;
                                }

                                if let Some(id) = post_id {
                                    // Edit existing post
                                    let payload = PostEditPayload {
                                        title: Some(form_data.data.title.clone()),
                                        content: Some(form_data.data.content.clone()),
                                        slug: Some(form_data.data.slug.clone()),
                                        excerpt: if form_data.data.excerpt.is_empty() {
                                            None
                                        } else {
                                            Some(form_data.data.excerpt.clone())
                                        },
                                        featured_image: if form_data.data.featured_image_url.is_empty() {
                                            None
                                        } else {
                                            form_data.data.featured_image_url.parse::<i32>().ok()
                                        },
                                        status: Some(if form_data.data.is_published {
                                            PostStatus::Published
                                        } else {
                                            PostStatus::Draft
                                        }),
                                        category_id: form_data.data.category_id,
                                        tag_ids: Some(form_data.data.tag_ids.clone()),
                                        published_at: None,
                                    };

                                    spawn(async move {
                                        posts.edit(id, payload).await;
                                    });
                                } else {
                                    // Create new post
                                    let payload = PostCreatePayload {
                                        title: form_data.data.title.clone(),
                                        content: form_data.data.content.clone(),
                                        slug: form_data.data.slug.clone(),
                                        excerpt: if form_data.data.excerpt.is_empty() {
                                            None
                                        } else {
                                            Some(form_data.data.excerpt.clone())
                                        },
                                        featured_image: if form_data.data.featured_image_url.is_empty() {
                                            None
                                        } else {
                                            form_data.data.featured_image_url.parse::<i32>().ok()
                                        },
                                        is_published: form_data.data.is_published,
                                        category_id: form_data.data.category_id.unwrap(),
                                        tag_ids: form_data.data.tag_ids.clone(),
                                        published_at: None,
                                    };

                                    spawn(async move {
                                        posts.add(payload).await;
                                    });
                                }
                            },
                            {
                                let add_state = posts.add.read();
                                let edit_state = if let Some(id) = post_id {
                                    posts.edit.read().get(&id).cloned()
                                } else {
                                    None
                                };
                                let is_submitting = add_state.is_loading() || edit_state.as_ref().map_or(false, |s| s.is_loading());

                                if is_submitting {
                                    rsx! {
                                        span { class: "mr-2 inline-block h-4 w-4 animate-spin rounded-full border-2 border-solid border-current border-r-transparent" }
                                    }
                                } else {
                                    rsx! { }
                                }
                            }
                            if is_edit_mode { "Update Post" } else { "Create Post" }
                        }
                    }
                    }
                }
            }
        }
    }
}
