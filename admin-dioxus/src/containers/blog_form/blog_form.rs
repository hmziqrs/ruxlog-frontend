use dioxus::{logger::tracing, prelude::*};

use super::form::{use_blog_form, BlogForm};
use crate::components::AppInput;
use crate::router::Route;
use crate::store::{
    use_categories, use_post, use_tag, PostCreatePayload, PostEditPayload, PostStatus,
};

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
            initial_form.set(Some(BlogForm::new()));
        }
    });

    // Wait for form to be initialized
    let form_data = initial_form.read();
    if form_data.is_none() {
        return rsx! {
            div { class: "flex items-center justify-center h-screen",
                div { class: "text-center",
                    div { class: "loading loading-spinner loading-lg" }
                    p { class: "mt-4", "Loading..." }
                }
            }
        };
    }

    let blog_form_hook = use_blog_form(form_data.clone().unwrap());
    let mut form = blog_form_hook.form;
    let mut auto_slug = blog_form_hook.auto_slug;

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
                nav.push(Route::PostsListScreen {});
            });
        }
    });

    rsx! {
        div { class: "p-8 max-w-4xl mx-auto",
            div { class: "bg-base-100 shadow-2xl rounded-xl p-8",
                h1 { class: "text-2xl font-bold mb-6 text-primary",
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
                            div { class: "alert alert-error mb-6",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    class: "stroke-current shrink-0 h-6 w-6",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                                    }
                                }
                                span {
                                    {error_message.unwrap_or_else(|| "Failed to save post".to_string())}
                                }
                            }
                        }
                    } else {
                        rsx! { }
                    }
                }

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
                            label { class: "block text-sm font-medium text-primary", "Slug" }
                            if !is_edit_mode {
                                div { class: "flex items-center gap-2",
                                    span { class: "text-sm", "Auto-generate" }
                                    div { class: "flex items-center",
                                        input {
                                            class: "checkbox checkbox-primary",
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
                        label { class: "block text-sm font-medium text-primary",
                            "Category "
                            span { class: "text-error", "*" }
                        }
                        select {
                            class: "select select-bordered w-full",
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
                        label { class: "block text-sm font-medium text-primary", "Tags" }
                        {
                            let tags_list = tags.list.read();
                            tags_list.data.as_ref().map(|tags_data| {
                                rsx! {
                                    div { class: "flex flex-wrap gap-2 p-4 border rounded-lg",
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
                                                            class: "checkbox checkbox-primary checkbox-sm",
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
                                                        span { class: "text-sm", "{tag_name}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }).unwrap_or_else(|| rsx! {
                                div { class: "text-sm text-gray-500", "Loading tags..." }
                            })
                        }
                    }

                    // Excerpt field
                    div { class: "space-y-2",
                        label { class: "block text-sm font-medium text-primary", "Excerpt" }
                        textarea {
                            class: "w-full px-4 py-2 textarea textarea-bordered",
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
                    div { class: "flex flex-row items-center justify-between border rounded-lg p-4",
                        div { class: "space-y-0.5",
                            label { class: "text-base font-medium", "Publish" }
                            div { class: "text-sm text-gray-500", "Make this post publicly available" }
                        }
                        div { class: "flex items-center",
                            input {
                                class: "toggle toggle-primary",
                                r#type: "checkbox",
                                checked: form.read().data.is_published,
                                onchange: move |event| {
                                    form.write().data.is_published = event.checked();
                                },
                            }
                        }
                    }

                    // Content field
                    div { class: "space-y-2",
                        label { class: "block text-sm font-medium text-primary",
                            "Content "
                            span { class: "text-error", "*" }
                        }
                        textarea {
                            class: "w-full px-4 py-2 textarea textarea-bordered min-h-[300px]",
                            placeholder: "Your blog post content (Markdown supported)",
                            rows: "10",

                            value: form.read().data.content.clone(),
                            oninput: move |event| {
                                form.write().update_field("content", event.value());
                            },
                        }
                    }

                    // Form actions
                    div { class: "flex justify-end gap-4 pt-4",
                        button {
                            class: "btn btn-ghost",
                            r#type: "button",
                            onclick: move |_| {
                                nav.push(Route::PostsListScreen {});
                            },
                            "Cancel"
                        }
                        button {
                            class: "btn btn-primary",
                            r#type: "submit",
                            disabled: {
                                let add_state = posts.add.read();
                                let edit_state = if let Some(id) = post_id {
                                    posts.edit.read().get(&id).cloned()
                                } else {
                                    None
                                };
                                add_state.is_loading() || edit_state.as_ref().map_or(false, |s| s.is_loading())
                            },
                            onclick: move |e| {
                                e.prevent_default();

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
                                    rsx! { span { class: "loading loading-spinner" } }
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
