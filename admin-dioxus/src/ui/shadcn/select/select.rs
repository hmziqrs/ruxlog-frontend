use dioxus::{prelude::*};
use hmziq_dioxus_free_icons::{icons::ld_icons::LdChevronDown, Icon};

use crate::ui::shadcn::{Button, ButtonVariant};

use super::state::{SelectContext, SelectProps};

#[component]
pub fn Select(props: SelectProps) -> Element {
    let mut state = use_signal(|| SelectContext::new(props.groups.clone(), props.selected));

    rsx! {
        div { class: "select-root relative max-w-3xs",
            Button {
                variant: ButtonVariant::Outline,
                class: format!(
                    "w-full justify-between {}",
                    if state.read().selected.is_some() {
                        "text-foreground"
                    } else {
                        "text-muted-foreground"
                    },
                ),
                onclick: move |_| {
                    state.write().toggle();
                },
                label {
                    if let Some(selected) = state.read().selected.clone() {
                        {selected}
                    } else {
                        {props.placeholder}
                    }
                }
                Icon { icon: LdChevronDown }
            }
            if state.read().is_open {
                div { class: "select-content",
                    div { class: "select-content-inner",
                        for group in state.read().groups.iter() {
                            div { class: "select-group",
                                h3 { class: "select-group-label", "{group.label}" }
                                for item in group.items.iter() {
                                    div {
                                        class: "select-item",
                                        onclick: move |_| {},
                                        "{item}"
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