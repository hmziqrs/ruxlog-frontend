use dioxus::{logger::tracing, prelude::*};

use super::form::{use_category_form, CategoryForm};
use crate::components::AppInput;

#[derive(Props, PartialEq, Clone)]
pub struct CategoryFormContainerProps {
    #[props(default)]
    pub initial: Option<CategoryForm>,
    pub on_submit: EventHandler<CategoryForm>,
    #[props(default)]
    pub title: Option<String>,
    #[props(default)]
    pub submit_label: Option<String>,
}

#[component]
pub fn CategoryFormContainer(props: CategoryFormContainerProps) -> Element {
    let initial_category_form = props.initial.clone().unwrap_or_else(CategoryForm::new);
    let category_form_hook = use_category_form(initial_category_form);
    let mut form = category_form_hook.form;
    let mut auto_slug = category_form_hook.auto_slug;

    rsx! {
        div { class: "p-8 max-w-4xl mx-auto",
            div { class: "bg-base-100 shadow-2xl rounded-xl p-8",
                if let Some(t) = props.title.clone() { h1 { class: "text-2xl font-bold mb-6 text-primary", {t} } }
                form { class: "space-y-6",
                    // Name field
                    AppInput {
                        name: "name",
                        form,
                        label: "Name",
                        placeholder: "Category name",
                        onblur: move |_| {
                            if !*auto_slug.read() {
                                let name_value = form.peek().get_field("name").unwrap().value.clone();
                                let sanitized = CategoryForm::sanitize_slug(&name_value);
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
                            placeholder: "category-slug",
                        }
                    }

                    // Description field
                    div { class: "space-y-2",
                        label { class: "block text-sm font-medium text-primary", "Description" }
                        textarea {
                            class: "w-full px-4 py-2 textarea",
                            placeholder: "Brief description of the category",
                            rows: "3",
                            value: form.read().data.description.clone(),
                            onchange: move |event| {
                                form.write().update_field("description", event.value());
                            },
                        }
                    }

                    // Cover Image URL field
                    AppInput {
                        name: "cover_image",
                        form,
                        label: "Cover Image URL",
                        placeholder: "https://example.com/image.jpg",
                    }

                    // Logo Image URL field
                    AppInput {
                        name: "logo_image",
                        form,
                        label: "Logo Image URL",
                        placeholder: "https://example.com/logo.jpg",
                    }

                    // Parent Category ID field
                    AppInput {
                        name: "parent_id",
                        form,
                        label: "Parent Category ID",
                        placeholder: "Parent category ID",
                        r#type: "number",
                    }

                    // Form actions
                    div { class: "flex justify-end gap-4 pt-4",
                        button { class: "btn", r#type: "button", "Cancel" }
                        button {
                            class: "btn btn-primary",
                            onclick: move |e| {
                                e.prevent_default();
                                let submit = props.on_submit.clone();
                                form.write().on_submit(move |val| { submit.call(val); });
                            },
                            {props.submit_label.clone().unwrap_or_else(|| "Save Category".to_string())}
                        }
                    }
                }
            }
        }
    }
}