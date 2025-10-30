use crate::store::{use_image_editor, EditorTool};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct EditorCanvasProps {
    pub blob_url: String,
    pub width: u32,
    pub height: u32,
}

/// Canvas component for displaying the image being edited
#[component]
pub fn EditorCanvas(props: EditorCanvasProps) -> Element {
    let editor = use_image_editor();
    let active_tool = *editor.active_tool.read();
    let crop_region = editor.crop_region.read().clone();

    // Calculate display dimensions (max 800px width while maintaining aspect ratio)
    let max_display_width = 800.0;
    let aspect_ratio = props.width as f64 / props.height as f64;
    let (display_width, display_height) = if props.width as f64 > max_display_width {
        let width = max_display_width;
        let height = width / aspect_ratio;
        (width, height)
    } else {
        (props.width as f64, props.height as f64)
    };

    // Scale factor for crop overlay
    let scale_x = display_width / props.width as f64;
    let scale_y = display_height / props.height as f64;

    rsx! {
        div {
            class: "relative flex items-center justify-center rounded-lg overflow-hidden border border-zinc-200 dark:border-zinc-800",
            style: "min-height: 400px;",

            // Image display
            img {
                src: "{props.blob_url}",
                alt: "Image being edited",
                class: "max-w-full h-auto",
                style: "width: {display_width}px; height: {display_height}px; object-fit: contain;",
            }

            // Crop overlay
            if active_tool == EditorTool::Crop {
                div {
                    class: "absolute border-2 border-blue-500 bg-blue-500/20 cursor-move",
                    style: "left: {crop_region.x as f64 * scale_x}px; top: {crop_region.y as f64 * scale_y}px; width: {crop_region.width as f64 * scale_x}px; height: {crop_region.height as f64 * scale_y}px;",

                    // Resize handles
                    div { class: "absolute w-3 h-3 bg-blue-500 rounded-full", style: "left: -6px; top: -6px; cursor: nw-resize;" }
                    div { class: "absolute w-3 h-3 bg-blue-500 rounded-full", style: "right: -6px; top: -6px; cursor: ne-resize;" }
                    div { class: "absolute w-3 h-3 bg-blue-500 rounded-full", style: "left: -6px; bottom: -6px; cursor: sw-resize;" }
                    div { class: "absolute w-3 h-3 bg-blue-500 rounded-full", style: "right: -6px; bottom: -6px; cursor: se-resize;" }
                }
            }
        }
    }
}
