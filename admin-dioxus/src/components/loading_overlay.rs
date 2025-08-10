use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct LoadingOverlayProps {
    #[props(default = false)]
    pub visible: bool,
}

/// Full-cover loading overlay with a small spinner. Place inside a relatively-positioned container.
#[component]
pub fn LoadingOverlay(props: LoadingOverlayProps) -> Element {
    if !props.visible {
        return rsx! { Fragment {} };
    }

    rsx! {
        div { class: "absolute inset-0 z-10 bg-background/50 backdrop-blur-[1px] flex items-center justify-center",
            div { class: "h-6 w-6 rounded-full border-2 border-zinc-300 border-t-zinc-700 animate-spin" }
        }
    }
}
