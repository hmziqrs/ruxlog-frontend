#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CommandLoadingProps {
    children: Element<'_>,
    #[props(default)]
    progress: Option<u8>, // Value between 0 and 100
    #[props(default="Loading...".to_string())]
    label: String,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute<'_>>,
}

pub fn CommandLoading(props: CommandLoadingProps) -> Element<'_> {
    // This component is conditionally rendered by the parent based on loading state
    rsx! {
        div {
            ..props.attributes,
            "cmdk-loading": "",
            role: "progressbar",
            "aria-label": "{props.label}",
            "aria-valuenow": if let Some(p) = props.progress { format!("{}", p) } else { "".to_string() },
            "aria-valuemin": "0",
            "aria-valuemax": "100",

            div { // Inner div for styling/hiding visual progress if needed
                "aria-hidden": "true",
                &props.children
            }
        }
    }
}

