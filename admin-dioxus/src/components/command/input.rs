#![allow(non_snake_case)]
use crate::components::command::context::use_command_context;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CommandInputProps {
    #[props(default)]
    value: Option<String>, // Controlled value
    #[props(default)]
    on_value_change: Option<Callback<String>>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute<'_>>,
}

pub fn CommandInput(props: CommandInputProps) -> Element<'_> {
    let cx = use_hook_context();
    let cmdk_context = use_command_context();
    let search_signal = cmdk_context.state.search;
    let is_controlled = props.value.is_some();
    let current_value = if is_controlled {
        props.value.clone().unwrap_or_default()
    } else {
        search_signal.read().clone()
    };

    let handle_input = move |ev: FormEvent| {
        let new_value = ev.value();
        if !is_controlled {
            cmdk_context.set_search.call(new_value.clone());
        }
        if let Some(cb) = &props.on_value_change {
            cb.call(new_value);
        }
    };

    rsx! {
        input {
            ..props.attributes,
            "cmdk-input": "",
            autocomplete: "off",
            autocorrect: "off",
            spellcheck: "false",
            "aria-autocomplete": "list",
            role: "combobox",
            "aria-expanded": "true", // Assuming list is always expandable
            "aria-controls": "{cmdk_context.list_id.read()}",
            "aria-labelledby": "{cmdk_context.label_id.read()}",
            "aria-activedescendant": "{cmdk_context.state.selected_item_id.read().as_deref().unwrap_or(\"\")}",
            id: "{cmdk_context.input_id.read()}",
            "type": "text",
            value: "{current_value}",
            oninput: handle_input,
        }
    }
}
