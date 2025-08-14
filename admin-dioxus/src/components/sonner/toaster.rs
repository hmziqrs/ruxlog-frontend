//! Sonner Toaster Provider — Phase 2 (basic render, no animations)

use crate::components::portal_v2::{use_portal, PortalIn, PortalOut};
use dioxus::prelude::*;
use std::collections::VecDeque;

use super::state::SonnerCtx;
use super::toast::SonnerToast;
use super::types::{HeightT, Position, TextDirection, ToasterProps, ToastT};

#[derive(Props, Clone)]
pub struct SonnerToasterProps {
    #[props(default = ToasterProps::default())]
    pub defaults: ToasterProps,
    #[props(default = None)]
    pub children: Option<Element>,
}

impl PartialEq for SonnerToasterProps {
    fn eq(&self, _other: &Self) -> bool {
        // Force re-render checks to consider props changed. This avoids requiring
        // ToasterProps (which contains callbacks) to implement PartialEq.
        false
    }
}

#[component]
pub fn SonnerToaster(props: SonnerToasterProps) -> Element {
    // Toasts state (Phase 2: simple list, no stacking/measurements yet)
    let toasts = use_signal(|| VecDeque::<ToastT>::new());
    let heights = use_signal(|| Vec::<HeightT>::new());
    let mut interacting = use_signal(|| false);
    let hidden = use_signal(|| false);

    // Create callbacks for context
    let add_toast = {
        let mut toasts = toasts.clone();
        let default_duration = props.defaults.duration_ms;
        use_callback(move |mut toast: ToastT| {
            if toast.duration_ms.is_none() {
                toast.duration_ms = Some(default_duration);
            }
            let mut list = toasts.write();
            list.push_back(toast);
        })
    };

    let update_toast = {
        let mut toasts = toasts.clone();
        use_callback(move |toast: ToastT| {
            let mut list = toasts.write();
            if let Some(pos) = list.iter().position(|t| t.id == toast.id) {
                list[pos] = toast;
            }
        })
    };

    let dismiss_toast = {
        let mut toasts = toasts.clone();
        use_callback(move |id: u64| {
            let mut list = toasts.write();
            if let Some(pos) = list.iter().position(|t| t.id == id) {
                list.remove(pos);
            }
        })
    };

    let delete_toast = {
        let mut toasts = toasts.clone();
        use_callback(move |id: u64| {
            let mut list = toasts.write();
            if let Some(pos) = list.iter().position(|t| t.id == id) {
                list.remove(pos);
            }
        })
    };

    use_context_provider(|| SonnerCtx {
        // signals/defaults
        toasts: toasts.clone(),
        heights: heights.clone(),
        interacting: interacting.clone(),
        hidden: hidden.clone(),
        defaults: props.defaults.clone(),
        // callbacks
        add_toast,
        update_toast,
        dismiss_toast: dismiss_toast.clone(),
        delete_toast,
    });

    let portal = use_portal();

    // Listen for document visibility changes to pause timers when hidden
    use_effect(move || {
        let mut eval = dioxus::document::eval(
            "document.addEventListener('visibilitychange', () => { dioxus.send(document.hidden) })",
        );
        let mut hidden_sig = hidden.clone();
        spawn(async move {
            while let Ok(flag) = eval.recv().await {
                hidden_sig.set(flag);
            }
        });
    });

    // Compute container attributes
    let toasts_vec = {
        let list = toasts.read();
        list.iter().cloned().collect::<Vec<_>>()
    };
    let count = toasts_vec.len();
    let container_label = props
        .defaults
        .container_aria_label
        .clone()
        .unwrap_or_else(|| format!("{} notifications", count));

    // Position -> simple class name for now
    let position_class = match props.defaults.position {
        Position::TopLeft => "sonner-top-left",
        Position::TopRight => "sonner-top-right",
        Position::BottomLeft => "sonner-bottom-left",
        Position::BottomRight => "sonner-bottom-right",
        Position::TopCenter => "sonner-top-center",
        Position::BottomCenter => "sonner-bottom-center",
    };

    let dir_attr = match props.defaults.dir {
        TextDirection::Ltr => "ltr",
        TextDirection::Rtl => "rtl",
        TextDirection::Auto => "auto",
    };

    rsx! {
        {props.children}

        PortalIn { portal,
            // Container region
            div { 
                role: "region",
                aria_label: "{container_label}",
                tabindex: "-1",
                dir: "{dir_attr}",
                class: "sonner-container {position_class}",
                onmouseenter: move |_| interacting.set(true),
                onmouseleave: move |_| interacting.set(false),

                // Visible list (no stacking/measurements yet)
                for (i, toast) in toasts_vec.iter().enumerate() {
                    SonnerToast {
                        key: "{toast.id}-{i}",
                        id: toast.id,
                        title: toast.title.clone(),
                        description: toast.description.clone(),
                        toast_type: toast.toast_type,
                        close_button: toast.close_button,
                        duration_ms: toast.duration_ms,
                        on_auto_close: toast.on_auto_close.clone(),
                        on_close: {
                            let dismiss_toast = dismiss_toast.clone();
                            let id = toast.id;
                            Callback::new(move |_| dismiss_toast.call(id))
                        },
                    }
                }
            }
        }
        PortalOut { portal }
    }
}
