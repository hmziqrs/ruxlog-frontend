use dioxus::prelude::*;

use super::*;
use crate::components::animated_grid::provider::use_grid_context;

#[component]
pub fn AnimatedGridCircle(circle: CircleSignal) -> Element {
    let grid_ctx = use_grid_context();

    let (x, y, circle_state, is_scaling_in) = {
        let state = circle.read();
        let grid = grid_ctx.grid_data.read();

        let Some((x, y)) = indices_to_px(state.col, state.row, &grid) else {
            return rsx! {};
        };
        let is_scaling_in = state.is_scaling_in_active(&grid);
        (x, y, state.clone(), is_scaling_in)
    };

    let style = if circle_state.is_respawning() {
        web_sys::console::log_1(
            &format!(
                "Circle {} RESPAWNING: scale={:.2}, opacity={:.2}, transition=NONE",
                circle_state.id, circle_state.scale, circle_state.opacity
            )
            .into(),
        );
        format!(
            "transform: translate({x:.2}px, {y:.2}px) scale({:.2}); width: {DIAMETER_PX}px; height: {DIAMETER_PX}px; border-radius: 9999px; opacity: {:.2}; transition: none;",
            circle_state.scale, circle_state.opacity
        )
    } else if is_scaling_in || circle_state.is_scaling_out_active() {
        web_sys::console::log_1(
            &format!(
                "Circle {} SCALING: scale={:.2}, opacity={:.2}, duration={}ms",
                circle_state.id, circle_state.scale, circle_state.opacity, SCALE_DURATION_MS
            )
            .into(),
        );
        format!(
            "transform: translate({x:.2}px, {y:.2}px) scale({:.2}); width: {DIAMETER_PX}px; height: {DIAMETER_PX}px; border-radius: 9999px; opacity: {:.2}; transition: transform {SCALE_DURATION_MS}ms ease-in, opacity {SCALE_DURATION_MS}ms ease-in;",
            circle_state.scale, circle_state.opacity
        )
    } else {
        web_sys::console::log_1(
            &format!(
                "Circle {} MOVING: scale={:.2}, opacity={:.2}, duration={}ms",
                circle_state.id, circle_state.scale, circle_state.opacity, STEP_DURATION_MS
            )
            .into(),
        );
        format!(
            "transform: translate({x:.2}px, {y:.2}px) scale({:.2}); width: {DIAMETER_PX}px; height: {DIAMETER_PX}px; border-radius: 9999px; opacity: {:.2}; transition: transform {STEP_DURATION_MS}ms linear;",
            circle_state.scale, circle_state.opacity
        )
    };

    rsx! {
        div {
            class: "absolute will-change-transform pointer-events-none bg-primary shadow-[0_0_8px_1px_var(--primary)]",
            style: "{style}",
            ontransitionend: move |_| {
                handle_transition_end(circle, grid_ctx.clone());
            },
        }
    }
}
