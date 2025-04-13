use dioxus::{logger::tracing, prelude::*};

use super::form::{use_tag_form, TagForm};
use crate::components::AppInput;

#[component]
pub fn TagScreen() -> Element {
    let initial_tag_form = TagForm::new();
    let tag_form_hook = use_tag_form(initial_tag_form);
    let mut form = tag_form_hook.form;
    let mut auto_slug = tag_form_hook.auto_slug;

    rsx! {
        div { class: "p-8 max-w-4xl mx-auto",
            div { class: "bg-base-100 shadow-2xl rounded-xl p-8",
                h1 { class: "text-2xl font-bold mb-6 text-primary", "New Tag" }
                form { class: "space-y-6",
                    // Name field
                    AppInput {
                        name: "name",
                        form,
                        label: "Name",
                        placeholder: "Tag name",
                        onblur: move |_| {
                            if !*auto_slug.read() {
                                let name_value = form.peek().get_field("name").unwrap().value.clone();
                                let sanitized = TagForm::sanitize_slug(&name_value);
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
                            placeholder: "tag-slug",
                        }
                    }

                    // Description field
                    div { class: "space-y-2",
                        label { class: "block text-sm font-medium text-primary", "Description" }
                        textarea {
                            class: "w-full px-4 py-2 textarea",
                            placeholder: "Brief description of the tag",
                            rows: "3",
                            value: form.read().data.description.clone(),
                            onchange: move |event| {
                                form.write().update_field("description", event.value());
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
                                        tracing::info!("Tag form submitted: {:?}", val);
                                    });
                            },
                            "Create Tag"
                        }
                    }
                }
            }
        }
    }
}