use std::sync::atomic::Ordering;

use dioxus::prelude::*;

use super::circle::AnimatedGridCircle;
use super::*;
use crate::components::animated_grid::provider::use_grid_context;


#[component]
pub fn AnimatedGridCircles(#[props(optional)] count: Option<usize>) -> Element {
    let grid_ctx = use_grid_context();
    let circle_count = count.unwrap_or(DEFAULT_CIRCLE_COUNT);

    let mut circles = use_signal(|| Vec::new());

    use_effect(move || {
        if !circles.read().is_empty() {
            return;
        }

        let grid = grid_ctx.grid_data.read();
        if grid.vertical_lines.len() < 2 || grid.horizontal_lines.len() < 2 {
            return;
        }

        let new_circles = (0..circle_count)
            .map(|_| {
                let id = NEXT_CIRCLE_ID.fetch_add(1, Ordering::Relaxed);
                Signal::new(spawn_circle_state(id, &grid))
            })
            .collect::<Vec<_>>();

        circles.set(new_circles);
    });

    rsx! {
        div { class: "absolute inset-0 pointer-events-none",
            {
                circles
                    .read()
                    .iter()
                    .map(|circle_sig| {
                        let id = circle_sig.read().id;
                        rsx! {
                            AnimatedGridCircle { key: "{id}", circle: *circle_sig }
                        }
                    })
            }
        }
    }
}
