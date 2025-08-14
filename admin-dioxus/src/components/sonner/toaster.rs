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

    // Compute container attributes and stacking offsets
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

    // Stacking calculations (Phase 4): absolute-position each toast with offsets
    let gap = props.defaults.gap_px.max(0) as i32;
    let fallback_h = 64i32; // px estimate if not yet measured

    let heights_vec = heights.read().clone();
    let heights_px: Vec<i32> = toasts_vec
        .iter()
        .map(|t| {
            heights_vec
                .iter()
                .find(|r| r.toast_id == t.id)
                .map(|r| r.height_px)
                .unwrap_or(fallback_h)
        })
        .collect();

    let mut offsets: Vec<i32> = Vec::with_capacity(toasts_vec.len());
    match props.defaults.position {
        Position::TopLeft | Position::TopRight | Position::TopCenter => {
            let mut cursor = 0i32;
            for h in &heights_px {
                offsets.push(cursor);
                cursor += *h + gap;
            }
        }
        Position::BottomLeft | Position::BottomRight | Position::BottomCenter => {
            // Compute from bottom upwards
            let mut cursor = 0i32;
            let mut tmp: Vec<i32> = Vec::with_capacity(toasts_vec.len());
            for h in heights_px.iter().rev() {
                tmp.push(cursor);
                cursor += *h + gap;
            }
            tmp.reverse();
            offsets = tmp;
        }
    }

    // Visible count handling (expand = show all)
    let requested_visible = props.defaults.visible_toasts.max(1);
    let max_visible = if props.defaults.expand { usize::MAX } else { requested_visible };
    let visible_count = count.min(max_visible);

    // Container height from visible toasts only
    let visible_height = if visible_count == 0 { 0 } else {
        match props.defaults.position {
            Position::TopLeft | Position::TopRight | Position::TopCenter => {
                let slice = &heights_px[0..visible_count];
                let sum: i32 = slice.iter().sum();
                sum + gap * ((visible_count as i32) - 1)
            }
            Position::BottomLeft | Position::BottomRight | Position::BottomCenter => {
                let start = count - visible_count;
                let slice = &heights_px[start..count];
                let sum: i32 = slice.iter().sum();
                sum + gap * ((visible_count as i32) - 1)
            }
        }
    };

    let container_style = format!("position: relative; height: {}px;", visible_height.max(0));

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
                style: container_style,

                // Visible list with stacking/offsets
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
                        style: {match props.defaults.position {
                            Position::TopLeft | Position::TopRight | Position::TopCenter => {
                                // Clamp overflow items to last visible offset
                                if visible_count == 0 {
                                    "position:absolute; left:0; right:0; top:0px; opacity:0;".to_string()
                                } else if i < visible_count {
                                    format!(
                                        "position:absolute; left:0; right:0; top:{}px; z-index:{};",
                                        offsets.get(i).cloned().unwrap_or(0),
                                        1000 - i as i32
                                    )
                                } else {
                                    let cutoff = visible_count - 1;
                                    let overflow_index = i - cutoff; // 1 for first overflow
                                    let scale = (1.0 - (overflow_index as f32) * 0.06).max(0.82);
                                    let opacity = (1.0 - (overflow_index as f32) * 0.15).max(0.4);
                                    format!(
                                        "position:absolute; left:0; right:0; top:{}px; transform: scale({:.3}); opacity: {:.3}; pointer-events: none; z-index:{};",
                                        offsets.get(cutoff).cloned().unwrap_or(0),
                                        scale,
                                        opacity,
                                        1000 - cutoff as i32
                                    )
                                }
                            }
                            Position::BottomLeft | Position::BottomRight | Position::BottomCenter => {
                                if visible_count == 0 {
                                    "position:absolute; left:0; right:0; bottom:0px; opacity:0;".to_string()
                                } else {
                                    let visible_start = count - visible_count;
                                    if i >= visible_start {
                                        format!(
                                            "position:absolute; left:0; right:0; bottom:{}px; z-index:{};",
                                            offsets.get(i).cloned().unwrap_or(0),
                                            1000 - (count - i) as i32
                                        )
                                    } else {
                                        let overflow_index = visible_start - i; // 1 for first overflow above visible cluster
                                        let scale = (1.0 - (overflow_index as f32) * 0.06).max(0.82);
                                        let opacity = (1.0 - (overflow_index as f32) * 0.15).max(0.4);
                                        format!(
                                            "position:absolute; left:0; right:0; bottom:{}px; transform: scale({:.3}); opacity: {:.3}; pointer-events: none; z-index:{};",
                                            offsets.get(visible_start).cloned().unwrap_or(0),
                                            scale,
                                            opacity,
                                            1000 - (count - visible_start) as i32
                                        )
                                    }
                                }
                            }
                        }},
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
