use dioxus::prelude::*;
use dioxus_time::sleep;
use std::time::Duration;
use wasm_bindgen::JsCast;

use crate::utils::grid_calculator::GridCalculator;

const MIN_CELL_SIZE: f64 = 60.0;
const MAX_CELL_SIZE: f64 = 80.0;

#[component]
pub fn AnimatedGridBackground() -> Element {
    let mut container_ref = use_signal(|| None as Option<std::rc::Rc<MountedData>>);
    let mut dimensions = use_signal(|| (0.0, 0.0)); // (width, height)
    let mut debounce_timer = use_signal(|| 0u64);

    // Calculate optimal grid based on current dimensions
    let calculate_grid = move || {
        let (width, height) = dimensions();
        let (_cell_size, vertical_lines, horizontal_lines) =
            GridCalculator::calculate_optimal_grid(width, height, MIN_CELL_SIZE, MAX_CELL_SIZE);
        (vertical_lines, horizontal_lines)
    };

    // Update dimensions helper
    let mut update_dimensions = move || {
        let timer_id = debounce_timer() + 1;
        debounce_timer.set(timer_id);

        spawn(async move {
            sleep(Duration::from_millis(50)).await;

            if debounce_timer() != timer_id {
                return;
            }

            let rect = container_ref.peek();
            if rect.is_none() {
                return;
            }
            let data = rect.as_ref().unwrap();
            let rect = data.get_client_rect().await;
            if rect.is_err() {
                return;
            }
            let rect = rect.unwrap();

            dimensions.set((rect.size.width, rect.size.height));
        });
    };

    // Window resize listener
    use_effect(move || {
        let window = web_sys::window();
        if window.is_none() {
            return;
        }
        let window = window.unwrap();

        let closure = {
            let mut update_fn = update_dimensions.clone();
            wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                update_fn();
            }) as Box<dyn FnMut(_)>)
        };

        let _ = window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());

        // Keep closure alive
        closure.forget();
    });

    let (vertical_lines, horizontal_lines) = calculate_grid();

    rsx! {
        div {
            class: "pointer-events-none absolute inset-0 -z-10 bg-transparent",
            aria_hidden: "true",
            onmount: move |event| {
                container_ref.set(Some(event.data()));
                update_dimensions();
            },
            {vertical_lines.iter().map(|pos| {
                let offset = format!("{pos:.2}px");
                rsx! {
                    div {
                        key: "v-{pos}",
                        class: "absolute inset-y-0 border-l border-border",
                        style: format!("left: {offset}; opacity: 0.15;"),
                    }
                }
            })},
            {horizontal_lines.iter().map(|pos| {
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
