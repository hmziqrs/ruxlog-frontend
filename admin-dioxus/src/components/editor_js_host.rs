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
        script { r#type: "module", src: asset!("/assets/editor.js") }

        div { id: "editorjs", class: "min-h-[300px] border rounded-md" }
    }
}
