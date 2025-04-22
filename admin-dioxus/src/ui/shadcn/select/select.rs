use dioxus::{prelude::*};
use hmziq_dioxus_free_icons::{icons::ld_icons::{LdChevronDown, LdCheck}, Icon};

use crate::ui::shadcn::{Button, ButtonVariant};
use crate::ui::custom::AppPortal;

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
                AppPortal {
                    onclick: move |_| {
                        state.write().is_open = false;
                    },
                }
                div {
                    class: "absolute bg-popover text-popover-foreground z-50 min-w-[15rem] overflow-hidden rounded-lg border shadow-md animate-in fade-in-0 zoom-in-95",
                    onclick: move |e| {
                        e.stop_propagation();
                    },
                    div { class: "select-content-inner p-1 max-h-[300px] overflow-y-auto",
                        for group in state.read().groups.clone().iter() {
                            div { class: "select-group py-1.5",
                                h3 { class: "select-group-label px-2 text-xs font-semibold text-muted-foreground",
                                    "{group.label}"
                                }
                                for item in group.clone().items.into_iter() {
                                    div {
                                        class: "relative flex cursor-default select-none items-center rounded-sm py-1.5 pl-2 pr-8 text-sm outline-none focus:bg-accent focus:text-accent-foreground hover:bg-accent hover:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
                                        onclick: move |_| {
                                            state.write().selected = Some(item.clone());
                                            state.write().toggle();
                                            if let Some(on_select) = props.on_select {
                                                on_select(item.clone());
                                            }
                                        },
                                        // Check mark for selected item
                                        if state.read().selected.as_ref() == Some(&item) {
                                            span { class: "absolute right-2 flex size-3.5 items-center justify-center",
                                                Icon {
                                                    icon: LdCheck,
                                                    class: "size-4",
                                                }
                                            }
                                        }
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