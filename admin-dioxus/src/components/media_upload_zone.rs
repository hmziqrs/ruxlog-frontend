use crate::store::{use_media, MediaReference, MediaUploadPayload};
use crate::ui::shadcn::{Button, ButtonVariant};
use crate::utils::file_helpers::validate_file_type;
use dioxus::prelude::*;
use hmziq_dioxus_free_icons::{icons::ld_icons::LdUpload, Icon};
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

static INPUT_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Props, Clone, PartialEq)]
pub struct MediaUploadZoneProps {
    /// Called when files are selected and upload initiated
    /// Returns Vec of blob URLs
    pub on_upload: EventHandler<Vec<String>>,
    /// Reference type for uploaded media
    #[props(default)]
    pub reference_type: Option<MediaReference>,
    /// Maximum number of files to accept at once (0 = unlimited)
    #[props(default = 0)]
    pub max_files: usize,
    /// Allowed mime type prefixes (e.g., ["image/", "video/"])
    /// Empty vec means all types allowed
    #[props(default)]
    pub allowed_types: Vec<String>,
    /// Title text for the upload zone
    #[props(default = "Upload Files".to_string())]
    pub title: String,
    /// Description text
    #[props(default = "Drag and drop files here, or click to select".to_string())]
    pub description: String,
    /// Whether to allow multiple files
    #[props(default = true)]
    pub multiple: bool,
}

/// Upload zone for media files (drag-and-drop temporarily disabled)
#[component]
pub fn MediaUploadZone(props: MediaUploadZoneProps) -> Element {
    let media_state = use_media();

    let input_id = use_signal(|| {
        format!(
            "media-upload-zone-{}",
            INPUT_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
        )
    });

    // Pre-clone props for event handlers
    let reference_type = props.reference_type.clone();
    let max_files = props.max_files;
    let allowed_types = props.allowed_types.clone();
    let on_upload_handler = props.on_upload.clone();

    // Use a signal to trigger file processing
    let mut trigger_upload = use_signal(|| 0u32);

    // Open file picker and trigger upload processing
    let open_picker = {
        let id = input_id();
        let mut trigger = trigger_upload;
        move |_| {
            if let Some(window) = web_sys::window() {
                if let Some(doc) = window.document() {
                    if let Some(el) = doc.get_element_by_id(&id) {
                        if let Ok(input) = el.dyn_into::<HtmlInputElement>() {
                            let _ = input.click();
                            // Trigger the effect by incrementing
                            trigger.set(trigger() + 1);
                        }
                    }
                }
            }
        }
    };

    // Watch for file selection and process
    use_effect(move || {
        let _tick = trigger_upload();
        if _tick == 0 {
            return; // Skip initial render
        }

        let id = input_id();
        let reference_type_clone = reference_type.clone();
        let allowed_types_clone = allowed_types.clone();
        let max_files_clone = max_files;
        let on_upload_clone = on_upload_handler.clone();

        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Some(el) = doc.get_element_by_id(&id) {
                    if let Ok(input) = el.dyn_into::<HtmlInputElement>() {
                        if let Some(files) = input.files() {
                            if files.length() > 0 {
                                spawn(async move {
                                    let allowed_refs: Vec<&str> = allowed_types_clone.iter().map(|s| s.as_str()).collect();
                                    let blob_urls = process_files_async(
                                        files,
                                        max_files_clone,
                                        &allowed_refs,
                                        reference_type_clone,
                                    ).await;

                                    if !blob_urls.is_empty() {
                                        on_upload_clone.call(blob_urls);
                                    }
                                });

                                // Reset input
                                input.set_value("");
                            }
                        }
                    }
                }
            }
        }
    });

    // TODO: Add drag-and-drop support
    // Drag event handlers are temporarily disabled due to type compatibility issues
    // Will be re-enabled after fixing event handler signatures

    let border_class = "border-zinc-200 dark:border-zinc-800 hover:border-zinc-300 dark:hover:border-zinc-700";

    rsx! {
        div {
            class: "border-2 border-dashed rounded-lg p-8 text-center transition-colors {border_class}",

            // Hidden file input
            input {
                r#type: "file",
                accept: if props.allowed_types.is_empty() { "*".to_string() } else { props.allowed_types.join(",") },
                multiple: props.multiple,
                class: "hidden",
                id: "{input_id()}",
            }

            div { class: "flex flex-col items-center gap-3",
                // Icon
                div { class: "p-4 rounded-full bg-zinc-100 dark:bg-zinc-800",
                    Icon { icon: LdUpload, class: "h-8 w-8 text-zinc-500 dark:text-zinc-400" }
                }

                // Text
                div { class: "space-y-1",
                    p { class: "text-sm font-medium text-foreground",
                        "{props.title}"
                    }
                    p { class: "text-xs text-muted-foreground",
                        "{props.description}"
                    }
                    if props.max_files > 0 {
                        {
                            let file_text = if props.max_files > 1 { "files" } else { "file" };
                            rsx! {
                                p { class: "text-xs text-muted-foreground",
                                    "Maximum {props.max_files} {file_text}"
                                }
                            }
                        }
                    }
                }

                // Button
                Button {
                    r#type: "button".to_string(),
                    variant: ButtonVariant::Outline,
                    onclick: open_picker,
                    class: "border-zinc-200 dark:border-zinc-800".to_string(),
                    "Select Files"
                }
            }
        }
    }
}

/// Process selected files and initiate uploads
/// Returns vector of blob URLs
async fn process_files_async(
    files: web_sys::FileList,
    max_files: usize,
    allowed_types: &[&str],
    reference_type: Option<MediaReference>,
) -> Vec<String> {
    let media_state = use_media();
    let mut blob_urls = Vec::new();
    let limit = if max_files > 0 {
        max_files.min(files.length() as usize)
    } else {
        files.length() as usize
    };

    for i in 0..limit {
        if let Some(file) = files.get(i as u32) {
            // Validate file type
            if !validate_file_type(&file, allowed_types) {
                gloo_console::warn!("File type not allowed:", file.type_());
                continue;
            }

            // Create payload
            let payload = MediaUploadPayload {
                file: file.clone(),
                reference_type: reference_type.clone(),
                width: None,  // Could be calculated if needed
                height: None,
            };

            // Initiate upload (returns blob URL immediately)
            match media_state.upload(payload).await {
                Ok(blob_url) => {
                    blob_urls.push(blob_url);
                }
                Err(e) => {
                    gloo_console::error!("Upload failed:", e);
                }
            }
        }
    }

    blob_urls
}
