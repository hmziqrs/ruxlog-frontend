use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn AnimatedBlob() -> Element {
    use_hook(move || {
        // Animate the canvas blob using web-sys and wasm-bindgen
        spawn(async move {
            use web_sys::{window as js_window, HtmlCanvasElement, CanvasRenderingContext2d, CanvasGradient};
            let window = js_window().unwrap();
            let document = window.document().unwrap();
            let canvas = document.get_element_by_id("animated-blob").unwrap();
            let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();
            let ctx = canvas
                .get_context("2d").unwrap().unwrap()
                .dyn_into::<CanvasRenderingContext2d>().unwrap();

            let mut time: f64 = 0.0;
            let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
            let g = f.clone();
            let closure = Closure::wrap(Box::new(move || {
                let width = canvas.width() as f64;
                let height = canvas.height() as f64;
                ctx.clear_rect(0.0, 0.0, width, height);
                let center_x = width / 2.0;
                let center_y = height / 2.0;
                // let grad = ctx.
                // ctx.
                let grad = ctx.create_radial_gradient(center_x, center_y, 0.0, center_x, center_y, width * 0.4).unwrap();
                grad.add_color_stop(0.0, "rgba(244,244,245,0.6)").unwrap(); // neutral-100
                grad.add_color_stop(0.5, "rgba(113,113,122,0.4)").unwrap(); // neutral-500
                grad.add_color_stop(1.0, "rgba(39,39,42,0)").unwrap(); // neutral-800
                ctx.set_fill_style(&grad);
                ctx.begin_path();
                let points = 8;
                let radius = width.min(height) * 0.3;
                for i in 0..=points {
                    let angle = (i as f64 / points as f64) * std::f64::consts::PI * 2.0;
                    let noise = ((time * 0.001) + (i as f64) * 0.5).sin() * (radius * 0.2);
                    let x = center_x + angle.cos() * (radius + noise);
                    let y = center_y + angle.sin() * (radius + noise);
                    if i == 0 {
                        ctx.move_to(x, y);
                    } else {
                        ctx.line_to(x, y);
                    }
                }
                ctx.close_path();
                ctx.fill();
                time += 1.0;
                // Schedule next frame
                window
                    // .unwrap()
                    .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                    .unwrap();
            }) as Box<dyn FnMut()>);
            *g.borrow_mut() = Some(closure);
            let w = js_window().unwrap();
                w.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        });
    });

    rsx! {
        div { class: "absolute inset-0 flex items-center justify-center pointer-events-none z-0",
            canvas { id: "animated-blob", class: "w-[400px] h-[400px]" }
        }
    }
}