use dioxus::prelude::*;

use super::form::{use_category_form, CategoryForm};
use crate::components::{AppInput, ColorPicker};
use crate::ui::shadcn::{Button, ButtonSize, ButtonVariant, Checkbox, Skeleton, Combobox, ComboboxItem};
use crate::store::use_category;

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
    let cats_state = use_category();

    // Fetch categories for parent selection on mount
    use_effect(move || {
        spawn(async move {
            cats_state.list().await;
        });
    });

    rsx! {
        div {
            if let Some(t) = props.title.clone() { h1 { class: "sr-only", {t} } }

            div { class: "grid grid-cols-1 gap-10 lg:grid-cols-3",
                // Main column
                div { class: "lg:col-span-2 space-y-8",
                    // Details card
                    div { class: "rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 shadow-sm",
                        div { class: "px-6 py-6",
                            h2 { class: "text-lg font-semibold", "Category details" }
                            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Basic information and metadata for your category." }
                        }
                        div { class: "px-6 py-6 space-y-6",
                            // Name
                            AppInput { name: "name", form, label: "Name", placeholder: "e.g. Tutorials" }

                            div { class: "h-px bg-zinc-200 dark:bg-zinc-800" }

                            // Slug with generate
                            div { class: "space-y-3",
                                div { class: "flex items-center justify-between",
                                    label { class: "block text-sm font-medium", "Slug" }
                                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm,
                                        onclick: move |_| {
                                            let name_value = form.peek().get_field("name").unwrap().value.clone();
                                            if !name_value.is_empty() {
                                                let sanitized = CategoryForm::sanitize_slug(&name_value);
                                                form.write().update_field("slug", sanitized);
                                            }
                                        },
                                        "Generate from name"
                                    }
                                }
                                AppInput { form, name: "slug", r#type: "text", placeholder: "tutorials" }
                                div { class: "flex items-center gap-2",
                                    span { class: "text-xs text-zinc-500 dark:text-zinc-400", "URL preview:" }
                                    code { class: "rounded bg-zinc-100 px-1.5 py-0.5 text-xs dark:bg-zinc-800",
                                        {
                                            let slug = form.read().data.slug.clone();
                                            let safe = if slug.trim().is_empty() { "your-slug".to_string() } else { slug };
                                            format!("/categories/{}", safe)
                                        }
                                    }
                                }
                            }

                            div { class: "h-px bg-zinc-200 dark:bg-zinc-800" }

                            // Description
                            div { class: "space-y-3",
                                label { class: "block text-sm font-medium", "Description" }
                                textarea {
                                    class: "w-full px-4 py-3 h-32 resize-none rounded-md border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 text-sm",
                                    placeholder: "Brief description of the category",
                                    value: form.read().data.description.clone(),
                                    onchange: move |event| { form.write().update_field("description", event.value()); }
                                }
                                p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Optional. Shown on the category page." }
                            }
                        }
                    }

                    // Branding card
                    div { class: "rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 shadow-sm",
                        div { class: "px-6 py-6",
                            h2 { class: "text-lg font-semibold", "Branding" }
                            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Color and images for the category." }
                        }
                        div { class: "px-6 py-6 space-y-6",
                            // Color picker + preview
                            div { class: "space-y-3",
                                label { class: "block text-sm font-medium", "Category color" }
                                ColorPicker { value: form.read().data.color.clone(), onchange: move |val| { form.write().update_field("color", val); } }
                                {
                                    let data = form.read().data.clone();
                                    let color = data.color.clone();
                                    rsx! {
                                        div { class: "inline-flex items-center gap-2 rounded-full border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-950 px-3 py-1.5 text-sm",
                                            span { class: "h-2.5 w-2.5 rounded-full ring-1 ring-zinc-200 dark:ring-zinc-800", style: format!("background-color: {}", color) }
                                            code { class: "text-xs border rounded px-1.5 py-0.5 border-zinc-200 dark:border-zinc-800", {color.clone()} }
                                            span { class: "text-xs opacity-70", if data.custom_text_color { "Using custom text color." } else { "Text color auto-adjusts for readability." } }
                                        }
                                    }
                                }
                            }

                            // Optional custom text color toggle
                            div { class: "space-y-2",
                                div { class: "flex items-center justify-between",
                                    div { class: "space-y-0.5",
                                        label { class: "block text-sm font-medium", "Use custom text color" }
                                        p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Override automatic contrast with your own text color." }
                                    }
                                    Checkbox { class: None, checked: form.read().data.custom_text_color, onchange: move |checked: bool| { form.write().update_field("custom_text_color", checked.to_string()); } }
                                }
                                if form.read().data.custom_text_color {
                                    ColorPicker { value: form.read().data.text_color.clone(), onchange: move |val| { form.write().update_field("text_color", val); } }
                                }
                            }

                            // Logo image URL
                            AppInput { name: "logo_image", form, label: "Logo Image URL", placeholder: "https://example.com/logo.jpg" }

                            // Cover image URL
                            AppInput { name: "cover_image", form, label: "Cover Image URL", placeholder: "https://example.com/cover.jpg" }
                        }
                    }
                }

                // Sidebar column
                div { class: "space-y-8",
                    // Visibility card
                    div { class: "rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 shadow-sm",
                        div { class: "px-6 pt-6",
                            h2 { class: "text-lg font-semibold", "Visibility" }
                            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Control whether this category is available publicly." }
                        }
                        div { class: "px-6 py-6 space-y-6",
                            div { class: "flex items-center justify-between",
                                div { class: "space-y-0.5",
                                    label { class: "block text-sm font-medium", "Active" }
                                    p { class: "text-xs text-zinc-500 dark:text-zinc-400",
                                        if form.read().data.active { "This category will be visible across your site." } else { "This category will be hidden and unavailable for selection." }
                                    }
                                }
                                Checkbox { class: None, checked: form.read().data.active, onchange: move |checked: bool| { form.write().update_field("active", checked.to_string()); } }
                            }
                        }
                    }

                    // Parent category selector
                    div { class: "rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 shadow-sm",
                        div { class: "px-6 pt-6",
                            h2 { class: "text-lg font-semibold", "Parent Category" }
                            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Optional. Select a parent to create a hierarchy." }
                        }
                        div { class: "px-6 py-6",
                            {
                                let list = cats_state.list.read();
                                let is_loading = list.is_loading();
                                let items: Vec<ComboboxItem> = if let Some(page) = list.data.clone() {
                                    page.data.into_iter().map(|c| ComboboxItem { value: c.id.to_string(), label: c.name }).collect()
                                } else { vec![] };
                                let current_val = {
                                    let pid = form.read().data.parent_id.clone();
                                    if pid.trim().is_empty() { None } else { Some(pid) }
                                };
                                rsx! {
                                    if is_loading && items.is_empty() {
                                        Skeleton { class: Some("h-10 w-full".to_string()) }
                                    } else {
                                        Combobox {
                                            items,
                                            placeholder: "Select parent...".to_string(),
                                            value: current_val,
                                            width: "w-full".to_string(),
                                            onvaluechange: Some(EventHandler::new(move |val: Option<String>| {
                                                let v = val.unwrap_or_default();
                                                form.write().update_field("parent_id", v);
                                            })),
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Actions
                    div { class: "flex gap-3",
                        Button { class: "flex-1 w-auto", variant: ButtonVariant::Outline, "Cancel" }
                        Button { class: "flex-1 w-auto",
                            onclick: move |_| {
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
