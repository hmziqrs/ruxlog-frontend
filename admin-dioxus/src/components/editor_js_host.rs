use dioxus::prelude::*;
use gloo_console::error;
use wasm_bindgen::JsValue;

#[component]
pub fn EditorJsHost(initial_json: Option<String>) -> Element {
    let initial_json_clone = initial_json.clone();

    use_effect(move || {
        if let Some(window) = web_sys::window() {
            let window_js = JsValue::from(window.clone());
            let key = JsValue::from_str("__EDITOR_INITIAL_DATA_RAW");

            if let Some(raw_json) = initial_json_clone.clone() {
                let value = JsValue::from_str(&raw_json);

                if let Err(err) = js_sys::Reflect::set(&window_js, &key, &value) {
                    error!("[EditorJsHost] Failed setting initial data", err);
                }
            } else {
                if let Err(err) = js_sys::Reflect::set(&window_js, &key, &JsValue::UNDEFINED) {
                    error!("[EditorJsHost] Failed clearing initial data", err);
                }
            }
        }
    });

    rsx! {
        // SVG sprite for editorjs-hyperlink icons
        svg {
            style: "display: none;",
            defs {
                symbol {
                    id: "link",
                    "viewBox": "0 0 14 10",
                    path {
                        d: "M1.5 5h11M10 1.5l3.5 3.5L10 8.5",
                        stroke: "currentColor",
                        "stroke-width": "1.5",
                        fill: "none",
                        "stroke-linecap": "round",
                    }
                }
                symbol {
                    id: "unlink",
                    "viewBox": "0 0 15 11",
                    path {
                        d: "M1.5 5.5h5m3 0h5M10 2l3.5 3.5L10 9",
                        stroke: "currentColor",
                        "stroke-width": "1.5",
                        fill: "none",
                        "stroke-linecap": "round",
                    }
                    line {
                        x1: "6.5",
                        y1: "1",
                        x2: "8.5",
                        y2: "10",
                        stroke: "currentColor",
                        "stroke-width": "1.5",
                        "stroke-linecap": "round",
                    }
                }
            }
        }

        script { r#type: "module", src: asset!("/assets/editor.bundle.js") }

        div { id: "editorjs", class: "min-h-[300px] border rounded-md" }
    }
}
