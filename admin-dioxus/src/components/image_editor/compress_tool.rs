use crate::store::use_image_editor;
use crate::ui::shadcn::Button;
use dioxus::prelude::*;

/// Compress tool controls
#[component]
pub fn CompressTool() -> Element {
    let editor = use_image_editor();
    let mut compress_params = editor.compress_params;
    let is_processing = *editor.is_processing.read();

    let handle_apply = move |_| {
        spawn(async move {
            let _ = editor.apply_compress().await;
        });
    };

    rsx! {
        div {
            class: "space-y-4 p-4 bg-white dark:bg-gray-800 rounded-lg border",

            h3 { class: "text-sm font-semibold mb-2", "Compression Settings" }

            div { class: "space-y-3",
                // Quality slider
                div {
                    label { class: "block text-xs text-gray-600 dark:text-gray-400 mb-1",
                        "Quality: {compress_params().quality}%"
                    }
                    input {
                        r#type: "range",
                        class: "w-full",
                        value: "{compress_params().quality}",
                        min: 1,
                        max: 100,
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<u8>() {
                                compress_params.write().quality = val;
                            }
                        }
                    }
                }

                // Quality presets
                div { class: "grid grid-cols-3 gap-2",
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        onclick: move |_| compress_params.write().quality = 60,
                        "Low (60%)"
                    }
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        onclick: move |_| compress_params.write().quality = 85,
                        "Medium (85%)"
                    }
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        onclick: move |_| compress_params.write().quality = 95,
                        "High (95%)"
                    }
                }

                // Helper text
                p { class: "text-xs text-gray-500 dark:text-gray-400",
                    "Lower quality = smaller file size. Higher quality = better image but larger file."
                }
            }

            Button {
                onclick: handle_apply,
                disabled: is_processing,
                class: "w-full",
                if is_processing { "Applying..." } else { "Apply Compression" }
            }
        }
    }
}
