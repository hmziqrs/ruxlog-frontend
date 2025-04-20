use dioxus::prelude::*;
use super::state::*;

#[component]
pub fn Cmdk(props: CommandListProps) -> Element {
    let mut  state = use_signal(|| CommandContext::new(props.groups.clone(), props.data.clone(), None));

    

    rsx! {
        div { class: "cmdk",
            div { class: "cmdk-input",
                input {
                    oninput: move |e| {
                        let value = e.value();
                        state.write().set_search(value);
                    },
                }
            }
            div { class: "cmdk-list" }
        }
    }
}