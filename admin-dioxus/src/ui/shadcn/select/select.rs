use dioxus::{html::label, prelude::*};
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
                    if let Some(selected) = state.read().selected.as_ref() {
                        {selected}
                    } else {
                        {props.placeholder}
                    }
                }
                Icon { icon: LdChevronDown }
            }
        }
    }
}