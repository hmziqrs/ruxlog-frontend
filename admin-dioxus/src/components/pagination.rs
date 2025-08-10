use dioxus::prelude::*;
use crate::ui::shadcn::{Button, ButtonVariant};

#[derive(Props, PartialEq, Clone)]
pub struct PaginationProps {
    /// Label shown on the left (hidden on small screens), e.g. "Page 1 • 42 items"
    pub label: String,
    /// Disable all interactions (loading state)
    #[props(default = false)]
    pub disabled: bool,
    /// Whether previous button should be disabled (e.g., no previous page)
    pub prev_disabled: bool,
    /// Whether next button should be disabled (e.g., no next page)
    pub next_disabled: bool,
    /// Callback when previous is clicked
    pub on_prev: EventHandler<()>,
    /// Callback when next is clicked
    pub on_next: EventHandler<()>,
}

#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let prev_is_disabled = props.disabled || props.prev_disabled;
    let next_is_disabled = props.disabled || props.next_disabled;

    rsx! {
        div { class: "flex items-center justify-between border-t border-border/60 p-3 text-sm text-muted-foreground",
            div { class: "hidden md:block", "{props.label}" }
            div { class: "flex items-center gap-2 ml-auto",
                Button { variant: ButtonVariant::Outline, disabled: prev_is_disabled, onclick: move |_| { props.on_prev.call(()); }, "Previous" }
                Button { disabled: next_is_disabled, onclick: move |_| { props.on_next.call(()); }, "Next" }
            }
        }
    }
}
