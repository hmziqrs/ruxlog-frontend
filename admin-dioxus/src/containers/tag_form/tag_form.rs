use dioxus::{logger::tracing, prelude::*};

use super::form::{use_tag_form, TagForm};
use crate::components::{AppInput, ColorPicker};
use crate::ui::shadcn::Checkbox;
use crate::utils::colors::get_contrast_yiq;

#[derive(Props, PartialEq, Clone)]
pub struct TagFormContainerProps {
    #[props(default)]
    pub initial: Option<TagForm>,
    pub on_submit: EventHandler<TagForm>,
    #[props(default)]
    pub title: Option<String>,
    #[props(default)]
    pub submit_label: Option<String>,
}

#[component]
pub fn TagFormContainer(props: TagFormContainerProps) -> Element {
    let initial_tag_form = props.initial.clone().unwrap_or_else(TagForm::new);
    let tag_form_hook = use_tag_form(initial_tag_form);
    let mut form = tag_form_hook.form;
    let mut auto_slug = tag_form_hook.auto_slug;
    

    rsx! {
        div { class: "p-8 max-w-4xl mx-auto",
            div { class: "bg-base-100 shadow-2xl rounded-xl p-8",
                h1 { class: "text-2xl font-bold mb-6 text-primary", {props.title.clone().unwrap_or_else(|| "New Tag".to_string())} }
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

                    // Appearance: color picker and preview
                    div { class: "space-y-3",
                        h2 { class: "text-sm font-semibold text-primary", "Appearance" }
                        div { class: "space-y-2",
                            label { class: "block text-sm font-medium", "Tag color" }
                            ColorPicker {
                                value: form.read().data.color.clone(),
                                onchange: move |val| {
                                    form.write().update_field("color", val);
                                },
                            }
                        }
                        // Text color override
                        div { class: "space-y-2",
                            div { class: "flex items-center justify-between",
                                div { class: "space-y-0.5",
                                    label { class: "block text-sm font-medium", "Text color" }
                                    p { class: "text-xs opacity-70",
                                        "Enable to choose a custom text color. Otherwise it auto-adjusts for readability."
                                    }
                                }
                                Checkbox {
                                    class: Some("size-6 rounded".to_string()),
                                    checked: form.read().data.custom_text_color,
                                    onchange: move |checked: bool| {
                                        form.write().update_field("custom_text_color", checked.to_string());
                                    },
                                }
                            }
                            if form.read().data.custom_text_color {
                                ColorPicker {
                                    value: form.read().data.text_color.clone(),
                                    onchange: move |val| {
                                        form.write().update_field("text_color", val);
                                    },
                                }
                            }
                        }
                        // Preview chip
                        {
                            let data = form.read().data.clone();
                            let color = data.color.clone();
                            let text_color = if data.custom_text_color && !data.text_color.trim().is_empty() {
                                data.text_color.clone()
                            } else {
                                get_contrast_yiq(&color).to_string()
                            };
                            let style = format!(
                                "background-color: {}; color: {}; border-color: rgba(0,0,0,0.06);",
                                color,
                                text_color
                            );
                            rsx! {
                                div { class: "space-y-1",
                                    label { class: "block text-sm font-medium", "Preview" }
                                    div { class: "flex items-center gap-3",
                                        span { class: "inline-flex items-center rounded-md px-2.5 py-1.5 text-sm font-medium shadow-sm ring-1 ring-inset",
                                            style: style,
                                            span { class: "mr-2 inline-block w-2.5 h-2.5 rounded-full", style: "background-color: rgba(255,255,255,0.4);" }
                                            {
                                                let name = form.read().data.name.clone();
                                                if name.is_empty() { "Tag preview".to_string() } else { name }
                                            }
                                        }
                                        code { class: "text-xs border rounded px-1.5 py-0.5", {color} }
                                    }
                                    p { class: "text-xs opacity-70", 
                                        if form.read().data.custom_text_color { 
                                            "Using custom text color."
                                        } else { 
                                            "Text color auto-adjusts for readability." 
                                        } 
                                    }
                                }
                            }
                        }
                    }

                    // Visibility: active toggle
                    div { class: "space-y-2",
                        h2 { class: "text-sm font-semibold text-primary", "Visibility" }
                        div { class: "flex items-center justify-between",
                            div { class: "space-y-0.5",
                                label { class: "block text-sm font-medium", "Active" }
                                p { class: "text-xs opacity-70",
                                    if form.read().data.active { "This tag will be visible across your site." } else { "This tag will be hidden and unavailable for selection." }
                                }
                            }
                            Checkbox {
                                class: Some("size-6 rounded".to_string()),
                                checked: form.read().data.active,
                                onchange: move |checked: bool| {
                                    form.write().update_field("active", checked.to_string());
                                },
                            }
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
                                let submit = props.on_submit.clone();
                                form.write().on_submit(move |val| {
                                    tracing::info!("Tag form submitted: {:?}", val);
                                    submit.call(val);
                                });
                            },
                            {props.submit_label.clone().unwrap_or_else(|| "Create Tag".to_string())}
                        }
                    }
                }
            }
        }
    }
}
