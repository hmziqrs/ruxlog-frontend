#![allow(non_snake_case)]
use crate::components::command::context::use_command_context;
use dioxus::prelude::*;
use dioxus_signals::*;

#[derive(Props, Clone, PartialEq)]
pub struct CommandEmptyProps {
    children: Element<'_>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute<'_>>,
}

pub fn CommandEmpty(props: CommandEmptyProps) -> Element<'_> {
    let cx = use_hook_context();
    let cmdk_context = use_command_context();

    let is_visible = use_memo(cx, &cmdk_context.state.filtered, |filtered| {
        filtered.read().count == 0
    });

    if !*is_visible {
        return None;
    }

    rsx! {
        div {
            ..props.attributes,
            "cmdk-empty": "",
            role: "presentation",
            &props.children
        }
    }
}

