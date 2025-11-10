mod circle;
mod provider;

pub use circle::*;
pub use provider::*;

use provider::use_grid_context;

use dioxus::prelude::*;

#[component]
pub fn AnimatedGridBackground() -> Element {
    let mut ctx = use_grid_context();

    rsx! {
        div {
            class: "pointer-events-none absolute inset-0 -z-10 bg-transparent",
            aria_hidden: "true",
            onmount: move |event| {
                ctx.container_ref.set(Some(event.data()));
                ctx.update_dimensions();
            },

            // Vertical lines
            {ctx.grid_data.read().vertical_lines.iter().map(|pos| {
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
            {ctx.grid_data.read().horizontal_lines.iter().map(|pos| {
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
