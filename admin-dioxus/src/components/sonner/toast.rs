//! Sonner Single Toast View — Phase 2 minimal view

use crate::hooks::use_unique_id;
use dioxus::prelude::*;
use dioxus_time::sleep;
use std::time::Duration;

use super::types::{HeightT, ToastType};
use super::state::SonnerCtx;

#[derive(Props, Clone, PartialEq)]
pub struct SonnerToastProps {
    pub id: u64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub toast_type: ToastType,
    #[props(default = true)]
    pub close_button: bool,
    #[props(default = None)]
    pub duration_ms: Option<u64>,
    #[props(default = None)]
    pub on_auto_close: Option<Callback<u64>>,
    pub on_close: Callback<MouseEvent>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn SonnerToast(props: SonnerToastProps) -> Element {
    let ctx = use_context::<SonnerCtx>();
    let uid = use_unique_id();
    let id = use_memo(move || format!("sonner-toast-{uid}"));
    let label_id = format!("{id}-label");
    let description_id = props
        .description
        .as_ref()
        .map(|_| format!("{id}-description"));
    let aria_labelledby_val = label_id.clone();
    let aria_describedby_val = description_id.clone();

    // Phase 4: measure height on mount and when content changes
    {
        let heights = ctx.heights.clone();
        let position = ctx.defaults.position;
        let id_for_measure = format!("{id}");
        let toast_id_for_height = props.id;
        let _title_dep = props.title.clone();
        let _desc_dep = props.description.clone();
        use_effect(move || {
            let mut eval = dioxus::document::eval(&format!(
                "(() => {{ const el = document.getElementById('{}'); dioxus.send(el ? Math.round(el.getBoundingClientRect().height) : 0) }})()",
                id_for_measure
            ));
            let mut heights_sig = heights.clone();
            spawn(async move {
                if let Ok(h) = eval.recv::<i32>().await {
                    let mut vec = heights_sig.write();
                    if let Some(existing) = vec.iter_mut().find(|r| r.toast_id == toast_id_for_height) {
                        existing.height_px = h as i32;
                        existing.position = position;
                    } else {
                        vec.push(HeightT { height_px: h as i32, toast_id: toast_id_for_height, position });
                    }
                }
            });
        });
    }

    // Phase 3: timers with pause on hover/focus/interacting/hidden
    let mut hovered = use_signal(|| false);
    let mut focused = use_signal(|| false);
    let interacting = ctx.interacting;
    let hidden = ctx.hidden;
    let paused = use_memo(move || hovered() || focused() || interacting() || hidden());
    let mut remaining_ms = use_signal(|| props.duration_ms.unwrap_or(0));
    let toast_id_for_timer = props.id;
    let on_auto_close_cb = props.on_auto_close.clone();
    let dismiss = ctx.dismiss_toast;
    let has_duration = props.duration_ms.is_some();
    let mut started = use_signal(|| false);
    use_effect(move || {
        if has_duration && !started() {
            started.set(true);
            spawn(async move {
                let tick = 50u64; // ms
                loop {
                    if paused() {
                        sleep(Duration::from_millis(tick)).await;
                        continue;
                    }
                    let current = remaining_ms();
                    if current == 0 {
                        break;
                    }
                    let next = current.saturating_sub(tick);
                    remaining_ms.set(next);
                    if next == 0 {
                        break;
                    }
                    sleep(Duration::from_millis(tick)).await;
                }
                // Auto close
                dismiss.call(toast_id_for_timer);
                if let Some(cb) = on_auto_close_cb.clone() {
                    cb.call(toast_id_for_timer);
                }
            });
        }
    });

    rsx! {
        div {
            id,
            role: "alertdialog",
            aria_labelledby: "{aria_labelledby_val}",
            aria_describedby: aria_describedby_val,
            aria_modal: "false",
            tabindex: "0",
            class: "sonner-toast flex items-center justify-between gap-2 w-72 px-4 py-3 rounded-md border border-border bg-background text-foreground shadow-sm",
            "data-type": props.toast_type.as_str(),
            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),
            onfocus: move |_| focused.set(true),
            onblur: move |_| focused.set(false),
            ..props.attributes,

            div { class: "sonner-toast-content flex-1",
                role: "alert",
                aria_atomic: "true",

                if let Some(title) = &props.title {
                    div { id: label_id.clone(), class: "sonner-toast-title font-semibold", {title.clone()} }
                }

                if let Some(description) = &props.description {
                    div { id: description_id.clone(), class: "sonner-toast-description text-sm text-muted-foreground", {description.clone()} }
                }
            }

            if props.close_button {
                button {
                    class: "sonner-toast-close self-start p-0 m-0 border-0 bg-transparent text-[18px] leading-none cursor-pointer text-muted-foreground hover:text-foreground",
                    aria_label: "close",
                    onclick: move |e| props.on_close.call(e),
                    "×"
                }
            }
        }
    }
}
