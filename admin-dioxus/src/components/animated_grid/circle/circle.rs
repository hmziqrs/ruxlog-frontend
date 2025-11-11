use dioxus::prelude::*;

use super::circles::{indices_to_px, CirclesSignal};
use super::state::GridCircle;
use crate::components::animated_grid::provider::GridContext;

#[derive(Props, Clone)]
pub struct AnimatedGridCircleProps {
    pub index: usize,
    pub circle: GridCircle,
    pub circles: CirclesSignal,
    pub grid_ctx: GridContext,
}

impl PartialEq for AnimatedGridCircleProps {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.circle == other.circle
    }
}

#[component]
pub fn AnimatedGridCircle(props: AnimatedGridCircleProps) -> Element {
    let AnimatedGridCircleProps {
        index,
        circle,
        circles,
        grid_ctx,
    } = props;

    let grid = grid_ctx.grid_data.read().clone();
    let Some((x, y)) = indices_to_px(circle.col, circle.row, &grid, circle.diameter_px) else {
        return rsx! {};
    };

    let transition_style = if circle.respawning {
        "transition: none;".to_string()
    } else {
        format!("transition: transform linear {}ms;", circle.step_ms)
    };

    let style = format!(
        "transform: translate({x:.2}px, {y:.2}px); width: {d:.2}px; height: {d:.2}px; border-radius: 9999px; {transition_style}",
        d = circle.diameter_px,
    );

    rsx! {
        div {
            class: "absolute will-change-transform pointer-events-none bg-primary shadow-[0_0_8px_1px_var(--primary)]",
            style: "{style}",
            ontransitionend: move |_| {
                handle_transition_end(index, circles.clone(), grid_ctx.clone());
            },
        }
    }
}

pub fn handle_transition_end(index: usize, mut circles: CirclesSignal, grid_ctx: GridContext) {
    {
        let mut all = circles.write();
        if let Some(circle) = all.get_mut(index) {
            circle.moving = false;
        } else {
            return;
        }
    }

    super::circles::circle_step(index, circles, grid_ctx);
}
