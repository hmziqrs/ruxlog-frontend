use crate::store::use_image_editor;
use crate::ui::shadcn::Button;
use dioxus::prelude::*;

/// Crop tool controls
#[component]
pub fn CropTool() -> Element {
    let editor = use_image_editor();
    let mut crop_region = editor.crop_region;
    let is_processing = *editor.is_processing.read();

    let handle_apply = move |_| {
        spawn(async move {
            let _ = editor.apply_crop().await;
        });
    };

    rsx! {
        div {
            class: "space-y-4 p-4 bg-white dark:bg-gray-800 rounded-lg border",

            h3 { class: "text-sm font-semibold mb-2", "Crop Settings" }

            div { class: "grid grid-cols-2 gap-3",
                // X position
                div {
                    label { class: "block text-xs text-gray-600 dark:text-gray-400 mb-1", "X Position" }
                    input {
                        r#type: "number",
                        class: "w-full px-2 py-1 text-sm border rounded",
                        value: "{crop_region().x}",
                        min: 0,
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<u32>() {
                                crop_region.write().x = val;
                            }
                        }
                    }
                }

                // Y position
                div {
                    label { class: "block text-xs text-gray-600 dark:text-gray-400 mb-1", "Y Position" }
                    input {
                        r#type: "number",
                        class: "w-full px-2 py-1 text-sm border rounded",
                        value: "{crop_region().y}",
                        min: 0,
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<u32>() {
                                crop_region.write().y = val;
                            }
                        }
                    }
                }

                // Width
                div {
                    label { class: "block text-xs text-gray-600 dark:text-gray-400 mb-1", "Width" }
                    input {
                        r#type: "number",
                        class: "w-full px-2 py-1 text-sm border rounded",
                        value: "{crop_region().width}",
                        min: 1,
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<u32>() {
                                crop_region.write().width = val;
                            }
                        }
                    }
                }

                // Height
                div {
                    label { class: "block text-xs text-gray-600 dark:text-gray-400 mb-1", "Height" }
                    input {
                        r#type: "number",
                        class: "w-full px-2 py-1 text-sm border rounded",
                        value: "{crop_region().height}",
                        min: 1,
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<u32>() {
                                crop_region.write().height = val;
                            }
                        }
                    }
                }
            }

            Button {
                onclick: handle_apply,
                disabled: is_processing,
                class: "w-full",
                if is_processing { "Applying..." } else { "Apply Crop" }
            }
        }
    }
}
