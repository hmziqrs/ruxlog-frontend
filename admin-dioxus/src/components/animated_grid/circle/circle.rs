use dioxus::prelude::*;

use super::circles::{
    indices_to_px, CircleSignal, DIAMETER_PX, SCALE_DURATION_MS, STEP_DURATION_MS,
};
use crate::components::animated_grid::provider::use_grid_context;

#[component]
pub fn AnimatedGridCircle(circle: CircleSignal) -> Element {
    let grid_ctx = use_grid_context();

    let (x, y, respawning, scale, opacity) = {
        let circle_state = circle.read();
        let grid = grid_ctx.grid_data.read();

        let Some((x, y)) = indices_to_px(circle_state.col, circle_state.row, &grid) else {
            return rsx! {};
        };
        (
            x,
            y,
            circle_state.respawning,
            circle_state.scale,
            circle_state.opacity,
        )
    };

    let style = if respawning {
        // No transition when respawning (instant position change)
        format!(
            "transform: translate({x:.2}px, {y:.2}px) scale({scale:.2}); width: {DIAMETER_PX}px; height: {DIAMETER_PX}px; border-radius: 9999px; opacity: {opacity:.2}; transition: none;",
        )
    } else if scale != 1.0 {
        // During scale transitions: only animate scale/opacity with ease-in, keep position fixed
        format!(
            "transform: translate({x:.2}px, {y:.2}px) scale({scale:.2}); width: {DIAMETER_PX}px; height: {DIAMETER_PX}px; border-radius: 9999px; opacity: {opacity:.2}; transition: transform {SCALE_DURATION_MS}ms ease-in, opacity {SCALE_DURATION_MS}ms ease-in;",
        )
    } else {
        // During movement: only animate position with linear, keep scale/opacity fixed
        format!(
            "transform: translate({x:.2}px, {y:.2}px) scale({scale:.2}); width: {DIAMETER_PX}px; height: {DIAMETER_PX}px; border-radius: 9999px; opacity: {opacity:.2}; transition: transform {STEP_DURATION_MS}ms linear;",
        )
    };

    rsx! {
        div {
            class: "absolute will-change-transform pointer-events-none bg-primary shadow-[0_0_8px_1px_var(--primary)]",
            style: "{style}",
            ontransitionend: move |_| {
                super::circles::handle_transition_end(circle, grid_ctx.clone());
            },
        }
    }
}
