use dioxus::prelude::*;

use super::form::{use_tag_form, TagForm};
use crate::components::{AppInput, ColorPicker};
use crate::ui::shadcn::{Button, ButtonSize, ButtonVariant, Checkbox};
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
    // auto_slug behavior remains in hook effect; we expose a manual generate button in UI
    
    rsx! {
        // Grid layout: 2/3 main, 1/3 sidebar
        div {
            // Title (optional)
            if let Some(t) = props.title.clone() { h1 { class: "sr-only", {t} } }

            div { class: "grid grid-cols-1 gap-10 lg:grid-cols-3",
                // Main column
                div { class: "lg:col-span-2 space-y-8",
                    // Tag details card
                    div { class: "rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 shadow-sm",
                        div { class: "px-6 py-6",
                            h2 { class: "text-lg font-semibold", "Tag details" }
                            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Basic information and metadata for your tag." }
                        }
                        div { class: "px-6 py-6 space-y-6",
                            // Name
                            AppInput { name: "name", form, label: "Name", placeholder: "e.g. Product Updates" }

                            // Separator
                            div { class: "h-px bg-zinc-200 dark:bg-zinc-800" }

                            // Slug with generate button
                            div { class: "space-y-3",
                                div { class: "flex items-center justify-between",
                                    label { class: "block text-sm font-medium", "Slug" }
                                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm,
                                        onclick: move |_| {
                                            let name_value = form.peek().get_field("name").unwrap().value.clone();
                                            if !name_value.is_empty() {
                                                let sanitized = TagForm::sanitize_slug(&name_value);
                                                form.write().update_field("slug", sanitized);
                                            }
                                        },
                                        "Generate from name"
                                    }
                                }
                                AppInput { form, name: "slug", r#type: "text", placeholder: "product-updates" }
                                // URL preview
                                div { class: "flex items-center gap-2",
                                    span { class: "text-xs text-zinc-500 dark:text-zinc-400", "URL preview:" }
                                    code { class: "rounded bg-zinc-100 px-1.5 py-0.5 text-xs dark:bg-zinc-800",
                                        {
                                            let slug = form.read().data.slug.clone();
                                            let safe = if slug.trim().is_empty() { "your-tag-slug".to_string() } else { slug };
                                            format!("/tags/{}", safe)
                                        }
                                    }
                                }
                            }

                            // Separator
                            div { class: "h-px bg-zinc-200 dark:bg-zinc-800" }

                            // Description
                            div { class: "space-y-3",
                                label { class: "block text-sm font-medium", "Description" }
                                textarea {
                                    class: "w-full px-4 py-3 h-32 resize-none rounded-md border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 text-sm",
                                    placeholder: "Briefly describe what posts belong in this tag.",
                                    value: form.read().data.description.clone(),
                                    oninput: move |event| { form.write().update_field("description", event.value()); }
                                }
                            }
                        }
                    }

                    // Design tip alert
                    div { class: "flex gap-3 items-start rounded-lg border border-zinc-200 dark:border-zinc-800 bg-white/60 dark:bg-zinc-900/60 p-5",
                        // simple info dot
                        div { class: "mt-0.5 w-4 h-4 rounded-full bg-zinc-200 dark:bg-zinc-700" }
                        div { class: "space-y-1",
                            p { class: "font-medium text-sm", "Design tip" }
                            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Use contrasting colors to ensure the tag remains legible in both light and dark themes." }
                        }
                    }
                }

                // Sidebar column
                div { class: "space-y-8 lg:sticky lg:top-28 h-fit",
                    // Appearance card
                    div { class: "rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 shadow-sm",
                        div { class: "px-6 pt-6",
                            h2 { class: "text-lg font-semibold", "Appearance" }
                            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Choose a color and preview the tag." }
                        }
                        div { class: "px-6 py-6 space-y-6",
                            div { class: "space-y-3",
                                label { class: "block text-sm font-medium", "Tag color" }
                                ColorPicker { value: form.read().data.color.clone(), onchange: move |val| { form.write().update_field("color", val); } }
                            }
                            div { class: "space-y-3",
                                div { class: "flex items-center justify-between",
                                    div { class: "space-y-0.5",
                                        label { class: "block text-sm font-medium", "Text color" }
                                        p { class: "text-xs opacity-70", "Enable to choose a custom text color. Otherwise it auto-adjusts for readability." }
                                    }
                                    Checkbox { checked: form.read().data.custom_text_color, onchange: move |checked: bool| { form.write().update_field("custom_text_color", checked.to_string()); } }
                                }
                                if form.read().data.custom_text_color {
                                    ColorPicker { value: form.read().data.text_color.clone(), onchange: move |val| { form.write().update_field("text_color", val); } }
                                }
                            }
                            {
                                let data = form.read().data.clone();
                                let color = data.color.clone();
                                let text_color = if data.custom_text_color && !data.text_color.trim().is_empty() { data.text_color.clone() } else { get_contrast_yiq(&color).to_string() };
                                let style = format!("background-color: {}; color: {}; border-color: rgba(0,0,0,0.06);", color, text_color);
                                rsx! {
                                    div { class: "space-y-3",
                                        label { class: "block text-sm font-medium", "Preview" }
                                        div { class: "flex items-center gap-3",
                                            span { class: "inline-flex items-center rounded-md px-2.5 py-1.5 text-sm font-medium shadow-sm ring-1 ring-inset", style: style,
                                                span { class: "mr-2 inline-block w-2.5 h-2.5 rounded-full", style: "background-color: rgba(255,255,255,0.4);" }
                                                { if form.read().data.name.is_empty() { "Tag preview".to_string() } else { form.read().data.name.clone() } }
                                            }
                                            code { class: "text-xs border rounded px-1.5 py-0.5 border-zinc-200 dark:border-zinc-800", {color} }
                                        }
                                        p { class: "text-xs opacity-70", if form.read().data.custom_text_color { "Using custom text color." } else { "Text color auto-adjusts for readability." } }
                                    }
                                }
                            }
                        }
                    }

                    // Visibility card
                    div { class: "rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/60 shadow-sm",
                        div { class: "px-6 pt-6",
                            h2 { class: "text-lg font-semibold", "Visibility" }
                            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Control whether this tag is available publicly." }
                        }
                        div { class: "px-6 py-6 space-y-6",
                            div { class: "flex items-center justify-between",
                                div { class: "space-y-0.5",
                                    label { class: "block text-sm font-medium", "Active" }
                                    p { class: "text-xs text-zinc-500 dark:text-zinc-400", if form.read().data.active { "This tag will be visible across your site." } else { "This tag will be hidden and unavailable for selection." } }
                                }
                                Checkbox { class: None, checked: form.read().data.active, onchange: move |checked: bool| { form.write().update_field("active", checked.to_string()); } }
                            }
                        }
                    }

                    // Actions
                    div { class: "flex gap-3 pt-4",
                        Button { class: "flex-1 w-auto", variant: ButtonVariant::Outline, "Cancel" }
                        Button { class: "flex-1 w-auto",
                            onclick: move |_| {
                                // e.prevent_default();
                                let submit = props.on_submit.clone();
                                form.write().on_submit(move |val| { submit.call(val); });
                            },
                            {props.submit_label.clone().unwrap_or_else(|| "Save Tag".to_string())}
                        }
                    }
                }
            }
        }
    }
}
