use dioxus::prelude::*;

use super::circles::{indices_to_px, CircleSignal, DIAMETER_PX, STEP_DURATION_MS};
use crate::components::animated_grid::provider::use_grid_context;

#[component]
pub fn AnimatedGridCircle(circle: CircleSignal) -> Element {
    let grid_ctx = use_grid_context();

    let (x, y, respawning, scale) = {
        let circle_state = circle.read();
        let grid = grid_ctx.grid_data.read();

        let Some((x, y)) = indices_to_px(circle_state.col, circle_state.row, &grid) else {
            return rsx! {};
        };
        (x, y, circle_state.respawning, circle_state.scale)
    };

    let transition_style = if respawning {
        "transition: none;"
    } else {
        "transition: transform linear;"
    };

    let style = format!(
        "transform: translate({x:.2}px, {y:.2}px) scale({scale:.2}); width: {DIAMETER_PX}px; height: {DIAMETER_PX}px; border-radius: 9999px; {transition_style} transition-duration: {STEP_DURATION_MS}ms;",
    );

    rsx! {
        div {
            class: "absolute will-change-transform pointer-events-none bg-primary shadow-[0_0_8px_1px_var(--primary)]",
            style: "{style}",
            ontransitionend: move |_| {
                circle.write().moving = false;
                super::circles::circle_step(circle, grid_ctx.clone());
            },
        }
    }
}
