use dioxus::prelude::*;
use hmziq_dioxus_free_icons::icons::ld_icons::LdSearch;
use hmziq_dioxus_free_icons::Icon;

use crate::ui::shadcn::{Select, SelectGroup};

#[derive(Props, PartialEq, Clone)]
pub struct ListToolbarProps {
    /// Current search value
    pub search_value: String,
    /// Placeholder for the search input
    #[props(default = "Search".to_string())]
    pub search_placeholder: String,
    /// Disable interaction when true
    #[props(default = false)]
    pub disabled: bool,
    /// Called with new search value on input
    pub on_search_input: EventHandler<String>,

    /// Current status selected value (e.g., "all" | "active" | "inactive")
    pub status_selected: String,
    /// Called when status is selected
    pub on_status_select: EventHandler<String>,
}

/// Generic list toolbar with a search input and a status select.
#[component]
pub fn ListToolbar(props: ListToolbarProps) -> Element {
    rsx! {
        div { class: "bg-transparent",
            div { class: "flex flex-col gap-3 md:flex-row md:items-center md:justify-between",
                // Search
                div { class: "w-full md:w-96",
                    label { class: "sr-only", r#for: "search", "Search" }
                    div { class: "relative",
                        div { class: "pointer-events-none absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground", Icon { icon: LdSearch {} } }
                        input {
                            id: "search",
                            r#type: "search",
                            class: "pl-8 w-full h-9 rounded-md border border-zinc-200 dark:border-zinc-800 bg-transparent px-3 text-sm",
                            placeholder: "{props.search_placeholder}",
                            value: props.search_value.clone(),
                            disabled: props.disabled,
                            oninput: move |e| {
                                props.on_search_input.call(e.value());
                            },
                        }
                    }
                }

                // Status filter
                div { class: "flex w-full items-center gap-2 md:w-auto",
                    div { class: "w-full md:w-48 relative",
                        label { class: "sr-only", r#for: "status", "Status" }
                        Select {
                            groups: vec![SelectGroup::new(
                                "Status".to_string(),
                                vec!["All".to_string(), "Active".to_string(), "Inactive".to_string()],
                            )],
                            selected: Some(props.status_selected.clone()),
                            placeholder: "All status".to_string(),
                            on_select: move |value| {
                                props.on_status_select.call(value);
                            }
                        }
                        if props.disabled { div { class: "absolute inset-0 z-10 cursor-not-allowed bg-transparent" } }
                    }
                }
            }
        }
    }
}
