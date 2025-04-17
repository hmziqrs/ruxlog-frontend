#![allow(non_snake_case)]
use crate::components::command::context::use_command_context;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CommandSeparatorProps {
    #[props(default)]
    always_render: bool,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute<'_>>,
}

#[component]
pub fn CommandSeparator(props: CommandSeparatorProps) -> Element {
    let cmdk_context = use_command_context();

    let is_visible = use_memo(&cmdk_context.state.search, |search| {
        props.always_render || search.read().is_empty()
    });

    if !*is_visible {
        return None;
    }

    rsx! {
        div {
            ..props.attributes,
            "cmdk-separator": "",
            role: "separator",
        }
    }
}

