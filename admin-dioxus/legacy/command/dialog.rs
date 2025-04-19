#![allow(non_snake_case)]
use crate::components::command::Command; // Assuming CommandProps is accessible
use dioxus::prelude::*;

// Re-export CommandProps for convenience if needed elsewhere
pub use crate::components::command::CommandProps;

#[derive(Props, Clone, PartialEq)]
pub struct CommandDialogProps {
    // Props for controlling the dialog visibility
    #[props(default)]
    open: bool,
    on_open_change: Option<Callback<bool>>,

    // Command component props are nested
    #[props(inner)]
    command_props: CommandProps,

    // Styling classes (optional)
    #[props(default)]
    overlay_class: String,
    #[props(default)]
    content_class: String,

    // Container for portal (advanced, maybe omit initially)
    // container: Option<web_sys::HtmlElement>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute<'_>>, // Attributes for the content container
}

#[component]
pub fn CommandDialog(props: CommandDialogProps) -> Element {
    if !props.open {
        return None; // Don't render anything if not open
    }

    // Simulate dialog structure with divs
    // A real implementation would use portals and better focus management
    let handle_overlay_click = move |_| {
        if let Some(cb) = &props.on_open_change {
            cb.call(false);
        }
    };

    rsx! {
        // Portal would go here in a more complete implementation
        div { // Simulate Overlay
            "cmdk-overlay": "",
            class: "{props.overlay_class}",
            style: "position: fixed; inset: 0; background-color: rgba(0,0,0,0.5); z-index: 40;", // Basic overlay style
            onclick: handle_overlay_click,
        }
        div { // Simulate Content container
            ..props.attributes,
            "cmdk-dialog": "",
            class: "{props.content_class}",
            style: "position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); background-color: white; padding: 1rem; border-radius: 0.5rem; z-index: 50;", // Basic dialog style
            "aria-label": "{props.command_props.label.as_deref().unwrap_or(\"Command Menu\")}",
            role: "dialog",
            "aria-modal": "true",

            // Render the actual Command component with its props
            Command {
                ..props.command_props.clone(), // Pass through command props
            }
        }
    }
}
