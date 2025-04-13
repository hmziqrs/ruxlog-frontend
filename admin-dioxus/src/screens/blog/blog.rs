use dioxus::{logger::tracing, prelude::*};

use super::form::{use_blog_form, BlogForm};
use crate::components::AppInput;

#[component]
pub fn BlogScreen() -> Element {
    let initial_blog_form = BlogForm::new();
    let blog_form_hook = use_blog_form(initial_blog_form);
    let mut form = blog_form_hook.form;
    let mut auto_slug = blog_form_hook.auto_slug;

    rsx! {
        div { class: "p-8 max-w-4xl mx-auto",
            div { class: "bg-base-100 shadow-2xl rounded-xl p-8",
                h1 { class: "text-2xl font-bold mb-6 text-primary", "New Blog Post" }
                form { class: "space-y-6",
                    // Title field
                    AppInput {
                        name: "title",
                        form,
                        label: "Title",
                        placeholder: "Post title",
                        onblur: move |_| {
                            if !*auto_slug.read() {
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
                        AppInput {
                            form,
                            name: "slug",
                            r#type: "text",
                            readonly: *auto_slug.read(),
                            placeholder: "Post slug",
                        }
                    }

                    // Excerpt field
                    div { class: "space-y-2",
                        label { class: "block text-sm font-medium text-primary", "Excerpt" }
                        textarea {
                            class: "w-full px-4 py-2 textarea",
                            placeholder: "Brief description of the post",
                            rows: "3",
                            value: form.read().data.excerpt.clone(),
                            onchange: move |event| {
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
                            label { class: "text-base", "Publish" }
                            div { class: "text-sm", "Make this post publicly available" }
                        }
                        div { class: "flex items-center",
                            input {
                                class: "checkbox checkbox-primary",
                                r#type: "checkbox",
                                checked: form.read().data.is_published,
                                onchange: move |event| {
                                    tracing::info!("is_published: {:?}", event.value());
                                },
                            }
                        }
                    }

                    // Content field
                    div { class: "space-y-2",
                        label { class: "block text-sm font-medium text-primary", "Content" }
                        textarea {
                            class: "w-full px-4 py-2 textarea min-h-[300px]",
                            placeholder: "Your blog post content",
                            rows: "10",
                            value: form.read().data.content.clone(),
                            onchange: move |event| {
                                form.write().update_field("content", event.value());
                            },
                        }
                    }

                    // Form actions
                    div { class: "flex justify-end gap-4 pt-4",
                        button { class: "btn", r#type: "button", "Cancel" }
                        button {
                            class: "btn btn-primary",
                            onclick: move |e| {
                                e.prevent_default();
                                tracing::info!("Form submitted OK ??");
                                form.write()
                                    .on_submit(|val| {
                                        tracing::info!("Form submitted: {:?}", val);
                                    });
                            },
                            "Create Post"
                        }
                    }
                }
            }
        }
    }
}