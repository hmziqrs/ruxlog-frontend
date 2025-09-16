use dioxus::prelude::*;
use hmziq_dioxus_free_icons::{
    icons::ld_icons::{LdCheck, LdChevronsUpDown},
    Icon,
};

use crate::ui::shadcn::{Popover, PopoverClose, PopoverContent, PopoverTrigger};

#[derive(Debug, Clone, PartialEq)]
pub struct ComboboxItem {
    pub value: String,
    pub label: String,
}

#[derive(Props, PartialEq, Clone)]
pub struct ComboboxProps {
    /// List of items to display in the combobox
    pub items: Vec<ComboboxItem>,
    /// Placeholder text for the combobox trigger
    #[props(default = String::from("Select an option..."))]
    pub placeholder: String,
    /// Placeholder text for the search input
    #[props(default = String::from("Search..."))]
    pub search_placeholder: String,
    /// Empty message when no items match the search
    #[props(default = String::from("No items found."))]
    pub empty_message: String,
    /// The currently selected value
    #[props(default)]
    pub value: Option<String>,
    /// Callback when the value changes
    #[props(default)]
    pub onvaluechange: Option<EventHandler<Option<String>>>,
    /// Width of the combobox
    #[props(default = String::from("w-[200px]"))]
    pub width: String,
    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,
}

#[component]
pub fn Combobox(props: ComboboxProps) -> Element {
    let mut value = use_signal(|| props.value.clone());
    let mut query = use_signal(|| String::new());

    // Handle incoming prop changes
    use_effect(move || {
        if props.value != *value.peek() {
            value.set(props.value.clone());
        }
    });

    // Find the selected item to display in the trigger
    let selected_item = props
        .items
        .iter()
        .find(|item| Some(&item.value) == value.read().as_ref());

    let display_text = selected_item
        .map(|item| item.label.clone())
        .unwrap_or(props.placeholder.clone());

    let mut class = vec!["relative".to_string()];

    if let Some(custom_class) = &props.class {
        class.push(custom_class.clone());
    }

    // Precompute filtered items based on query
    let q = query.read().to_lowercase();
    let filtered: Vec<ComboboxItem> = props
        .items
        .iter()
        .filter(|item| {
            q.is_empty()
                || item.label.to_lowercase().contains(&q)
                || item.value.to_lowercase().contains(&q)
        })
        .cloned()
        .collect();

    let mapped: Vec<(String, String, bool)> = filtered
        .iter()
        .map(|item| {
            let val = item.value.clone();
            let label = item.label.clone();
            let sel = Some(&val) == value.read().as_ref();
            (val, label, sel)
        })
        .collect();

    rsx! {
        div { class: class.join(" "),
            Popover {
                PopoverTrigger { class: Some(format!("{} flex items-center justify-between", props.width)),
                    button {
                        class: "bg-background hover:bg-accent hover:text-accent-foreground inline-flex h-10 w-full items-center justify-between rounded-md border border-input px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
                        role: "combobox",
                        "aria-expanded": "false",
                        span { class: "truncate", "{display_text}" }
                        Icon {
                            icon: LdChevronsUpDown,
                            class: "ml-2 h-4 w-4 shrink-0 opacity-50",
                        }
                    }
                }
                PopoverContent {
                    class: Some(format!("{} p-0", props.width)),
                    div { class: "p-2",
                        input {
                            class: "mb-2 h-9 w-full rounded-md border border-input bg-background px-3 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring",
                            r#type: "text",
                            placeholder: props.search_placeholder.clone(),
                            value: query.read().clone(),
                            oninput: move |e| {
                                query.set(e.value());
                            }
                        }
                        div { class: "max-h-64 overflow-auto",
                            if mapped.is_empty() {
                                div { class: "px-3 py-2 text-sm text-muted-foreground", "{props.empty_message}" }
                            } else {
                                for (item_value, item_label, is_selected) in mapped {
                                    PopoverClose {
                                        class: Some("w-full".to_string()),
                                        button { class: "flex w-full items-center justify-between rounded-sm px-3 py-2 text-sm hover:bg-accent hover:text-accent-foreground",
                                            onclick: move |_| {
                                                let new_value = if is_selected { None } else { Some(item_value.clone()) };
                                                value.set(new_value.clone());
                                                if let Some(handler) = &props.onvaluechange {
                                                    handler.call(new_value);
                                                }
                                            },
                                            span { class: "truncate", "{item_label}" }
                                            if is_selected { Icon { icon: LdCheck, class: "ml-2 h-4 w-4" } }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
