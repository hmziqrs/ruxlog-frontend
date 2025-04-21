use dioxus::prelude::*;
use hmziq_dioxus_free_icons::{Icon, icons::ld_icons::{LdCheck, LdChevronsUpDown}};

use crate::ui::{
    custom::{Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList},
    shadcn::{Popover, PopoverContent, PopoverTrigger}
};

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
    let mut open = use_signal(|| false);
    let mut value = use_signal(|| props.value.clone());
    
    // Handle incoming prop changes
    use_effect(move || {
        if props.value != *value.peek() {
            value.set(props.value.clone());
        }
    });

    // Find the selected item to display in the trigger
    let selected_item = props.items.iter()
        .find(|item| Some(&item.value) == value.as_ref());

    let display_text = selected_item
        .map(|item| item.label.clone())
        .unwrap_or(props.placeholder.clone());

    let mut class = vec!["relative".to_string()];
    
    if let Some(custom_class) = &props.class {
        class.push(custom_class.clone());
    }

    rsx! {
        div { class: class.join(" "),
            Popover {
                PopoverTrigger { class: Some(format!("{} flex items-center justify-between", props.width)),
                    button {
                        class: "bg-background hover:bg-accent hover:text-accent-foreground inline-flex h-10 w-full items-center justify-between rounded-md border border-input px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
                        role: "combobox",
                        "aria-expanded": if *open.read() { "true" } else { "false" },
                        span { class: "truncate", "{display_text}" }
                        Icon {
                            icon: LdChevronsUpDown,
                            class: "ml-2 h-4 w-4 shrink-0 opacity-50",
                        }
                    }
                }
                PopoverContent {
                    class: Some(format!("{} p-0", props.width)),
                    align: String::from("start"),
                    hide_close_button: true,
                    side_offset: 4,
                    Command {
                        CommandInput {
                            placeholder: props.search_placeholder.clone(),
                            class: Some("h-9".to_string()),
                        }
                        CommandList {
                            CommandEmpty { "{props.empty_message}" }
                            CommandGroup {
                                {
                                    props
                                        .items
                                        .iter()
                                        .map(|item| {
                                            let item_value = item.value.clone();
                                            let item_label = item.label.clone();
                                            let is_selected = Some(&item_value) == value.as_ref();
                                            rsx! {
                                                CommandItem {
                                                    value: item_value.clone(),
                                                    onselect: move |selected_value| {
                                                        let new_value = if Some(&selected_value) == value.as_ref() {
                                                            None
                                                        } else {
                                                            Some(selected_value)
                                                        };
                                                        value.set(new_value.clone());
                                                        if let Some(handler) = &props.onvaluechange {
                                                            handler.call(new_value);
                                                        }
                                                        open.set(false);
                                                    },
                                                    class: Some("flex justify-between".to_string()),
                                                    {item_label}
                                                    if is_selected {
                                                        Icon { icon: LdCheck, class: "ml-auto h-4 w-4" }
                                                    }
                                                }
                                            }
                                        })
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}