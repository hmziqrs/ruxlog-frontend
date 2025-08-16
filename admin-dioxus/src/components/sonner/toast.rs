//! Sonner Single Toast View — Phase 2 minimal view + Phase 7 swipe/drag to dismiss

use crate::hooks::use_unique_id;
use dioxus::prelude::*;
use dioxus_time::sleep;
use std::time::Duration;

use super::types::{HeightT, ToastType, SwipeDirection, Position, DEFAULT_SWIPE_THRESHOLD_PX};
use super::icons::{icon_close, icon_error, icon_info, icon_success, icon_warning, loader_spinner};
use super::state::SonnerCtx;

#[derive(Props, Clone, PartialEq)]
pub struct SonnerToastProps {
    pub id: u64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub toast_type: ToastType,
    #[props(default = None)]
    pub icon: Option<String>,
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

    // Phase 7: swipe/drag to dismiss
    // Determine allowed swipe directions: use defaults if provided, else infer from position
    let allowed_dirs = {
        let cfg = ctx.defaults.swipe_directions.clone();
        if !cfg.is_empty() {
            cfg
        } else {
            match ctx.defaults.position {
                Position::TopLeft | Position::BottomLeft => vec![SwipeDirection::Left],
                Position::TopRight | Position::BottomRight => vec![SwipeDirection::Right],
                Position::TopCenter => vec![SwipeDirection::Top],
                Position::BottomCenter => vec![SwipeDirection::Bottom],
            }
        }
    };

    let drag_dx = use_signal(|| 0.0f64);
    let drag_dy = use_signal(|| 0.0f64);
    let dragging = use_signal(|| false);
    let snapping = use_signal(|| false);

    // Attach JS pointer/touch listeners to track drag deltas
    {
        let _id_for_drag = format!("{id}");
        let mut dx_sig = drag_dx.clone();
        let mut dy_sig = drag_dy.clone();
        let mut dragging_sig = dragging.clone();
        let mut snapping_sig = snapping.clone();
        let dismiss = ctx.dismiss_toast.clone();
        let toast_id = props.id;
        let allowed = allowed_dirs.clone();
        use_effect(move || {
            let script = format!(r#"(function() {{
                const el = document.getElementById('{id}');
                if (!el) {{ dioxus.send(['noop',0,0]); return; }}
                if (el.dataset.dragBound === '1') {{ return; }}
                el.dataset.dragBound = '1';
                let startX=0, startY=0, active=false;
                function onDown(ev) {{
                    const p = ev.touches ? ev.touches[0] : ev;
                    startX = p.clientX; startY = p.clientY; active = true;
                    dioxus.send(['start', 0, 0]);
                    window.addEventListener('mousemove', onMove);
                    window.addEventListener('touchmove', onMove, {{ passive: false }});
                    window.addEventListener('mouseup', onUp);
                    window.addEventListener('touchend', onUp);
                    window.addEventListener('touchcancel', onUp);
                }}
                function onMove(ev) {{
                    if (!active) return;
                    const p = ev.touches ? ev.touches[0] : ev;
                    const dx = p.clientX - startX; const dy = p.clientY - startY;
                    dioxus.send(['move', dx, dy]);
                    if (ev.cancelable) ev.preventDefault();
                }}
                function onUp(ev) {{
                    if (!active) return;
                    active = false;
                    const p = (ev.changedTouches && ev.changedTouches[0]) ? ev.changedTouches[0] : ev;
                    const dx = p.clientX - startX; const dy = p.clientY - startY;
                    dioxus.send(['end', dx, dy]);
                    window.removeEventListener('mousemove', onMove);
                    window.removeEventListener('touchmove', onMove);
                    window.removeEventListener('mouseup', onUp);
                    window.removeEventListener('touchend', onUp);
                    window.removeEventListener('touchcancel', onUp);
                }}
                el.addEventListener('mousedown', onDown);
                el.addEventListener('touchstart', onDown, {{ passive: true }});
            }})()"#);
            let mut eval = dioxus::document::eval(&script);
            let allowed_for_task = allowed.clone();
            spawn(async move {
                while let Ok((typ, dx, dy)) = eval.recv::<(String, f64, f64)>().await {
                    match typ.as_str() {
                        "start" => { dragging_sig.set(true); snapping_sig.set(false); }
                        "move" => { dx_sig.set(dx); dy_sig.set(dy); }
                        "end" => {
                            dragging_sig.set(false);
                            // Project movement onto allowed axis and decide dismissal with sign check
                            let ax_h = allowed_for_task.iter().any(|d| matches!(d, SwipeDirection::Left | SwipeDirection::Right));
                            let ax_v = allowed_for_task.iter().any(|d| matches!(d, SwipeDirection::Top | SwipeDirection::Bottom));
                            let use_h = ax_h && (!ax_v || dx.abs() >= dy.abs());
                            let threshold = DEFAULT_SWIPE_THRESHOLD_PX as f64;
                            let mut should_dismiss = false;
                            if use_h {
                                if dx <= -threshold && allowed_for_task.iter().any(|d| matches!(d, SwipeDirection::Left)) { should_dismiss = true; }
                                if dx >=  threshold && allowed_for_task.iter().any(|d| matches!(d, SwipeDirection::Right)) { should_dismiss = true; }
                            } else if ax_v {
                                if dy <= -threshold && allowed_for_task.iter().any(|d| matches!(d, SwipeDirection::Top)) { should_dismiss = true; }
                                if dy >=  threshold && allowed_for_task.iter().any(|d| matches!(d, SwipeDirection::Bottom)) { should_dismiss = true; }
                            }
                            if should_dismiss {
                                // Dismiss immediately
                                dismiss.call(toast_id);
                                // reset deltas to avoid lingering transform
                                dx_sig.set(0.0); dy_sig.set(0.0);
                            } else {
                                // Snap back with transition
                                snapping_sig.set(true);
                                dx_sig.set(0.0); dy_sig.set(0.0);
                            }
                        }
                        _ => {}
                    }
                }
            });
        });
    }

    // Compute drag style for inner wrapper
    let dx = drag_dx();
    let dy = drag_dy();
    let ax_h = allowed_dirs.iter().any(|d| matches!(d, SwipeDirection::Left | SwipeDirection::Right));
    let ax_v = allowed_dirs.iter().any(|d| matches!(d, SwipeDirection::Top | SwipeDirection::Bottom));
    let use_h = ax_h && (!ax_v || dx.abs() >= dy.abs());
    let (tx, ty) = if use_h { (dx, 0.0) } else if ax_v { (0.0, dy) } else { (0.0, 0.0) };
    let ratio = ((if use_h { dx.abs() } else { dy.abs() }) / (DEFAULT_SWIPE_THRESHOLD_PX as f64)).min(1.0);
    let opacity = 1.0 - 0.3 * ratio;
    let transition = if snapping() && !dragging() { "transform 180ms ease-out, opacity 180ms ease-out" } else { "none" };
    let drag_style = format!("transform: translate({:.2}px, {:.2}px); opacity: {:.3}; transition: {};", tx, ty, opacity, transition);

    // Phase 8: resolve icon element based on per-toast and global overrides
    let icon_el: Option<Element> = {
        // Prefer per-toast keyword, else provider defaults by type
        let kw_opt = props
            .icon
            .clone()
            .or_else(|| match props.toast_type {
                ToastType::Success => ctx.defaults.icons.success.clone(),
                ToastType::Info => ctx.defaults.icons.info.clone(),
                ToastType::Warning => ctx.defaults.icons.warning.clone(),
                ToastType::Error => ctx.defaults.icons.error.clone(),
                ToastType::Loading => ctx.defaults.icons.loading.clone(),
            });

        if let Some(kw) = kw_opt {
            let k = kw.to_lowercase();
            match k.as_str() {
                "none" => None,
                "success" => Some(icon_success(None)),
                "info" => Some(icon_info(None)),
                "warning" => Some(icon_warning(None)),
                "error" => Some(icon_error(None)),
                "loading" | "loader" => Some(loader_spinner(None)),
                "close" => Some(icon_close(None)),
                _ => {
                    // Fallback to default for this toast type
                    match props.toast_type {
                        ToastType::Success => Some(icon_success(None)),
                        ToastType::Info => Some(icon_info(None)),
                        ToastType::Warning => Some(icon_warning(None)),
                        ToastType::Error => Some(icon_error(None)),
                        ToastType::Loading => Some(loader_spinner(None)),
                    }
                }
            }
        } else {
            // Default for this toast type
            match props.toast_type {
                ToastType::Success => Some(icon_success(None)),
                ToastType::Info => Some(icon_info(None)),
                ToastType::Warning => Some(icon_warning(None)),
                ToastType::Error => Some(icon_error(None)),
                ToastType::Loading => Some(loader_spinner(None)),
            }
        }
    };

    rsx! {
        div {
            id,
            role: "alertdialog",
            aria_labelledby: "{aria_labelledby_val}",
            aria_describedby: aria_describedby_val,
            aria_modal: "false",
            tabindex: "0",
            class: "sonner-toast w-72 rounded-md border border-border bg-background text-foreground shadow-sm",
            "data-type": props.toast_type.as_str(),
            // Apply drag transform to the OUTER container so the whole toast (including border/bg) moves
            style: "{drag_style}",
            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),
            onfocus: move |_| focused.set(true),
            onblur: move |_| focused.set(false),
            ..props.attributes,

            // Inner wrapper is static content container
            div { class: "sonner-toast-inner flex items-center justify-between gap-2 px-4 py-3",
                // Left icon (if any)
                if let Some(icon) = icon_el {
                    div { class: "sonner-toast-icon shrink-0", {icon} }
                }

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
                        class: "sonner-toast-close self-start p-0 m-0 border-0 bg-transparent leading-none cursor-pointer text-muted-foreground hover:text-foreground",
                        aria_label: "close",
                        onclick: move |e| props.on_close.call(e),
                        { icon_close(None) }
                    }
                }
            }
        }
    }
}
