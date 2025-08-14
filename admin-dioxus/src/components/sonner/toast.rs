//! Sonner Single Toast View — Phase 2 minimal view

use crate::hooks::use_unique_id;
use dioxus::prelude::*;

use super::types::ToastType;

#[derive(Props, Clone, PartialEq)]
pub struct SonnerToastProps {
    pub id: u64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub toast_type: ToastType,
    #[props(default = true)]
    pub close_button: bool,
    pub on_close: Callback<MouseEvent>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn SonnerToast(props: SonnerToastProps) -> Element {
    let uid = use_unique_id();
    let id = use_memo(move || format!("sonner-toast-{uid}"));
    let label_id = format!("{id}-label");
    let description_id = props
        .description
        .as_ref()
        .map(|_| format!("{id}-description"));
    let aria_labelledby_val = label_id.clone();
    let aria_describedby_val = description_id.clone();

    rsx! {
        div {
            id,
            role: "alertdialog",
            aria_labelledby: "{aria_labelledby_val}",
            aria_describedby: aria_describedby_val,
            aria_modal: "false",
            tabindex: "0",
            class: "sonner-toast flex items-center justify-between gap-2 w-72 px-4 py-3 rounded-md border border-border bg-background text-foreground shadow-sm",
            "data-type": props.toast_type.as_str(),
            ..props.attributes,

            div { class: "sonner-toast-content flex-1",
                role: "alert",
                aria_atomic: "true",

                if let Some(title) = &props.title {
                    div { id: label_id.clone(), class: "sonner-toast-title font-semibold", {title.clone()} }
                }

                if let Some(description) = &props.description {
                    div { id: description_id.clone(), class: "sonner-toast-description text-sm text-muted-foreground", {description.clone()} }
                }
            }

            if props.close_button {
                button {
                    class: "sonner-toast-close self-start p-0 m-0 border-0 bg-transparent text-[18px] leading-none cursor-pointer text-muted-foreground hover:text-foreground",
                    aria_label: "close",
                    onclick: move |e| props.on_close.call(e),
                    "×"
                }
            }
        }
    }
}
