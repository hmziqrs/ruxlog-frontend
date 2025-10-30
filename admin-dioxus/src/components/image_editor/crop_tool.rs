use crate::store::use_image_editor;
use crate::ui::shadcn::Button;
use dioxus::prelude::*;

/// Crop tool controls
#[component]
pub fn CropTool() -> Element {
    let editor = use_image_editor();
    let is_processing = *editor.is_processing.read();

    let handle_apply = move |_| {
        spawn(async move {
            let _ = editor.apply_crop().await;
        });
    };

    rsx! {
        div {
            class: "space-y-4 p-4 rounded-lg border border-zinc-200 dark:border-zinc-800",

            h3 { class: "text-sm font-semibold mb-2", "Crop Settings" }

            div { class: "grid grid-cols-2 gap-3",
                // X position
                div {
                    label { class: "block text-xs text-muted-foreground mb-1", "X Position" }
                    input {
                        r#type: "number",
                        class: "w-full px-2 py-1 text-sm border border-zinc-200 dark:border-zinc-800 rounded",
                        value: "{editor.crop_region.read().x}",
                        min: 0,
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<u32>() {
                                editor.crop_region.write().x = val;
                            }
                        }
                    }
                }

                // Y position
                div {
                    label { class: "block text-xs text-muted-foreground mb-1", "Y Position" }
                    input {
                        r#type: "number",
                        class: "w-full px-2 py-1 text-sm border border-zinc-200 dark:border-zinc-800 rounded",
                        value: "{editor.crop_region.read().y}",
                        min: 0,
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<u32>() {
                                editor.crop_region.write().y = val;
                            }
                        }
                    }
                }

                // Width
                div {
                    label { class: "block text-xs text-muted-foreground mb-1", "Width" }
                    input {
                        r#type: "number",
                        class: "w-full px-2 py-1 text-sm border border-zinc-200 dark:border-zinc-800 rounded",
                        value: "{editor.crop_region.read().width}",
                        min: 1,
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<u32>() {
                                editor.crop_region.write().width = val;
                            }
                        }
                    }
                }

                // Height
                div {
                    label { class: "block text-xs text-muted-foreground mb-1", "Height" }
                    input {
                        r#type: "number",
                        class: "w-full px-2 py-1 text-sm border border-zinc-200 dark:border-zinc-800 rounded",
                        value: "{editor.crop_region.read().height}",
                        min: 1,
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<u32>() {
                                editor.crop_region.write().height = val;
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
