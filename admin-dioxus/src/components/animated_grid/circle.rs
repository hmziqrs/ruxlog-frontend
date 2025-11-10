use dioxus::prelude::*;

use super::provider::use_grid_context;

const CIRCLE_KEYFRAMES: &str = r#"
    @keyframes gridCircleMove {
        from { left: -32px; }
        to { left: calc(100vw + 32px); }
    }
"#;

#[component]
pub fn AnimatedGridCircle() -> Element {
    let ctx = use_grid_context();

    rsx! {
        Fragment {
            style {
                dangerous_inner_html: CIRCLE_KEYFRAMES,
            }
            div {
                class: "absolute pointer-events-none",
                style: format!(
                    "top: {}px; animation: gridCircleMove 25s linear infinite;",
                    ctx.grid_data.read().middle_line
                ),
                div {
                    class: "bg-primary size-[6px] rounded-full -translate-x-1/2 -translate-y-1/2 shadow-[0_0_7px_1px_var(--primary)]",
                }
            }
        }
    }
}
