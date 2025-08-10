use dioxus::prelude::*;
use hmziq_dioxus_free_icons::{Icon, icons::ld_icons::{LdPlus, LdX}};
use crate::ui::shadcn::{Button, ButtonSize, ButtonVariant};

#[derive(Props, PartialEq, Clone)]
pub struct ImageUploadProps {
    /// Current image URL (None or empty string means no image)
    #[props(default)]
    pub value: Option<String>,
    /// Called with the new URL when uploaded, or empty string when removed
    pub onchange: EventHandler<String>,
    /// Primary call-to-action label
    pub title: String,
    /// Helper text shown in the empty state
    pub description: String,
    /// Tailwind aspect ratio utility (e.g. "aspect-video")
    #[props(default = "aspect-video".to_string())]
    pub aspect_ratio: String,
}

/// Simple image upload UI that simulates upload with a timeout
#[component]
pub fn ImageUpload(props: ImageUploadProps) -> Element {
    let mut is_uploading = use_signal(|| false);

    let value = props.value.clone().unwrap_or_default();
    let has_image = !value.trim().is_empty();

    // Simulate image upload (instant for now)
    let handle_upload = move |_| {
        if is_uploading() { return; }
        is_uploading.set(true);
        props.onchange.call("/placeholder.svg?height=600&width=1200".to_string());
        is_uploading.set(false);
    };

    // Remove image
    let handle_remove = move |_| {
        props.onchange.call(String::new());
    };

    rsx! {
        div { class: "space-y-2",
            if has_image {
                div { class: "relative overflow-hidden rounded-md border border-zinc-200 dark:border-zinc-800",
                    img { src: value, alt: props.title.clone(), class: format!("w-full h-auto object-cover {}", props.aspect_ratio) }
                    Button {
                        r#type: "button".to_string(),
                        variant: ButtonVariant::Destructive,
                        size: ButtonSize::Icon,
                        onclick: handle_remove,
                        class: "absolute top-2 right-2 h-8 w-8 rounded-full".to_string(),
                        Icon { icon: LdX }
                        span { class: "sr-only", "Remove image" }
                    }
                }
            } else {
                div { class: "border border-dashed rounded-md p-8 text-center border-zinc-200 dark:border-zinc-800",
                    div { class: "flex flex-col items-center gap-2",
                        div { class: "p-3 rounded-full bg-zinc-100 dark:bg-zinc-800",
                            Icon { icon: LdPlus, class: "h-6 w-6 text-zinc-500 dark:text-zinc-400" }
                        }
                        div { class: "text-sm text-zinc-500 dark:text-zinc-400",
                            p { "{props.description}" }
                        }
                        Button {
                            r#type: "button".to_string(),
                            variant: ButtonVariant::Outline,
                            onclick: handle_upload,
                            disabled: is_uploading(),
                            class: "mt-2 border-zinc-200 dark:border-zinc-800".to_string(),
                            if is_uploading() { "Uploading..." } else { "{props.title}" }
                        }
                    }
                }
            }
        }
    }
}
