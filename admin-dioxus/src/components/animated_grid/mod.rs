pub mod circle;
pub mod provider;

pub use circle::AnimatedGridCircle;
pub use provider::{GridContext, GridData, use_grid_context};

use dioxus::prelude::*;

#[component]
pub fn AnimatedGridBackground() -> Element {
    let mut ctx = use_grid_context();
    let grid_data_signal = ctx.grid_data;
    let grid_data = grid_data_signal.read();

    rsx! {
        div {
            class: "pointer-events-none absolute inset-0 -z-10 bg-transparent",
            aria_hidden: "true",
            onmount: move |event| {
                ctx.container_ref.set(Some(event.data()));
                ctx.update_dimensions();
            },

            // Vertical lines
            {grid_data.vertical_lines.iter().map(|pos| {
                let offset = format!("{pos:.2}px");
                rsx! {
                    div {
                        key: "v-{pos}",
                        class: "absolute inset-y-0 border-l border-border",
                        style: format!("left: {offset}; opacity: 0.15;"),
                    }
                }
            })},

            // Horizontal lines
            {grid_data.horizontal_lines.iter().map(|pos| {
                let offset = format!("{pos:.2}px");
                rsx! {
                    div {
                        key: "h-{pos}",
                        class: "absolute inset-x-0 border-t border-border",
                        style: format!("top: {offset}; opacity: 0.15;"),
                    }
                }
            })},
        }
    }
}
