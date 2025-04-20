use dioxus::{logger::tracing, prelude::*};
use super::state::*;

#[component]
pub fn Cmdk(props: CommandListProps) -> Element {
    let mut  state = use_signal(|| CommandContext::new(props.groups.clone(), props.data.clone(), None));

    let read = state.read();
    let groups = read.groups.clone();

    tracing::info!("RENDER");

    rsx! {
        div { class: "cmdk",
            div { class: "cmdk-input",
                input {
                    value: state.read().search.clone(),
                    oninput: move |e| {
                        let value = e.value();
                        tracing::info!("onchange: {}", value);
                        state.write().set_search(value);
                    },
                }
            }
            if read.is_empty {
                div { class: "cmdk-empty", "No results found" }
            }
            div { class: "cmdk-list",
                {
                    groups
                        .into_iter()
                        .map(|group| {
                            rsx! {
                                div { class: "cmdk-group",
                                    div { class: "cmdk-group-label", "{group.label}" }
                                    {
                                        group
                                            .items
                                            .into_iter()
                                            .map(|item| {
                                                rsx! {
                                                    div { class: "cmdk-group-item", "data-value": item.value, {item.label} }
                                                }
                                            })
                                    }
                                }
                            }
                        })
                }
            }
        }
    }
}