#![allow(non_snake_case)]
use crate::components::command::context::use_command_context;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CommandListProps {
    children: Element<'_>,
    #[props(default="Suggestions".to_string())]
    label: String,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute<'_>>,
}

pub fn CommandList(props: CommandListProps) -> Element<'_> {
    let cx = use_hook_context();
    let cmdk_context = use_command_context();
    let list_inner_ref_signal = cmdk_context.list_inner_ref; // Get the signal itself

    // TODO: Implement height animation using ResizeObserver if needed.
    // This is complex and might require JS interop or a different approach in Dioxus.
    // For now, we omit the dynamic height setting via CSS variable.

    rsx! {
        div {
            ..props.attributes,
            "cmdk-list": "",
            role: "listbox",
            // tabindex: "-1", // Usually focus stays on input
            "aria-label": "{props.label}",
            id: "{cmdk_context.list_id.read()}",
            "aria-activedescendant": "{cmdk_context.state.selected_item_id.read().as_deref().unwrap_or(\"\")}",

            // The inner div that contains the items and whose ref is stored
            div {
                "cmdk-list-sizer": "", // Use for potential height calculations later
                onmounted: move |cx| {
                    // Set the MountedElement in the context's signal
                    list_inner_ref_signal.set(Some(cx.inner().clone()));
                },
                &props.children
            }
        }
    }
}
