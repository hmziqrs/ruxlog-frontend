//! Rich text editor component for Dioxus.
//!
//! This module provides a full-featured WYSIWYG editor with an AST-first architecture.

pub mod ast;
pub mod commands;
pub mod renderer;
pub mod sanitizer;
pub mod toolbar;

pub use ast::{Block, BlockAlign, BlockKind, Doc, EmbedProvider, Inline, Link, MarkSet, TextSize};
pub use commands::{Command, CommandError, MarkType, Position, Selection, ToggleMark};
pub use renderer::render_doc;
pub use sanitizer::sanitize_html;

use dioxus::prelude::*;
use toolbar::Toolbar;
use wasm_bindgen::prelude::*;

/// Props for the RichTextEditor component.
#[derive(Props, Clone, PartialEq)]
pub struct RichTextEditorProps {
    /// Initial document content (JSON).
    #[props(default)]
    pub initial_value: Option<String>,

    /// Callback when content changes.
    pub on_change: EventHandler<String>,

    /// Placeholder text when editor is empty.
    #[props(default = "Start typing...".to_string())]
    pub placeholder: String,

    /// Whether the editor is read-only.
    #[props(default = false)]
    pub readonly: bool,

    /// Additional CSS classes.
    #[props(default = String::new())]
    pub class: String,
}

/// Main rich text editor component.
#[component]
pub fn RichTextEditor(props: RichTextEditorProps) -> Element {
    // Initial HTML content from props (computed once)
    let initial_html = {
        if let Some(json) = &props.initial_value {
            if let Ok(doc) = serde_json::from_str::<Doc>(json) {
                render_doc(&doc)
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    };

    let mut selection = use_signal(|| Selection::collapsed(Position::start()));
    let mut is_focused = use_signal(|| false);

    // Execute a command using browser's native execCommand
    let mut execute_command = move |cmd: Box<dyn Command>| {
        // Determine the execCommand based on command type
        let command_name = if let Some(toggle_mark) = cmd.as_any().downcast_ref::<ToggleMark>() {
            match toggle_mark.mark_type {
                MarkType::Bold => Some("bold"),
                MarkType::Italic => Some("italic"),
                MarkType::Underline => Some("underline"),
                MarkType::Strike => Some("strikeThrough"),
                MarkType::Code => {
                    // For code, we'll use a different approach
                    // execCommand doesn't support inline code well
                    None
                }
            }
        } else {
            None
        };

        if let Some(cmd_name) = command_name {
            // Call execCommand via JavaScript
            let _ = js_sys::eval(&format!(
                r#"document.execCommand('{}', false, null)"#,
                cmd_name
            ));
        }
    };

    // Set initial content on mount
    use_effect(move || {
        if !initial_html.is_empty() {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Ok(Some(element)) = document.query_selector(".editor-content") {
                        let _ = element.set_inner_html(&initial_html);
                    }
                }
            }
        }
    });

    rsx! {
        div {
            class: "rich-text-editor border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 shadow-sm {props.class}",

            // Toolbar
            if !props.readonly {
                Toolbar {
                    on_command: move |cmd| execute_command(cmd),
                    selection: selection.read().clone(),
                }
            }

            // Editor content area - simple contenteditable without dangerous_inner_html
            div {
                class: "editor-content min-h-[300px] focus:outline-none",
                contenteditable: if props.readonly { "false" } else { "true" },
                tabindex: "0",
                "data-placeholder": "{props.placeholder}",

                style: "
                    padding: 1rem;
                    white-space: pre-wrap;
                    word-wrap: break-word;
                    line-height: 1.6;
                    color: inherit;
                ",

                onfocus: move |_| {
                    is_focused.set(true);
                },

                onblur: move |_| {
                    is_focused.set(false);
                },

                oninput: move |evt| {
                    if props.readonly {
                        return;
                    }

                    // Get the HTML content from contenteditable
                    let html_value = evt.value();

                    // Sanitize and notify parent
                    let clean = sanitize_html(&html_value);
                    props.on_change.call(clean);
                },
            }

            // Inline styles for placeholder
            style {
                r"
                .editor-content[data-placeholder]:empty:before {{
                    content: attr(data-placeholder);
                    color: #9ca3af;
                    pointer-events: none;
                }}
                .dark .editor-content[data-placeholder]:empty:before {{
                    color: #6b7280;
                }}
                .editor-content:focus {{
                    outline: 2px solid #3b82f6;
                    outline-offset: -2px;
                }}
                .dark .editor-content:focus {{
                    outline-color: #60a5fa;
                }}
                "
            }

            // Character count (optional)
            if *is_focused.read() {
                div {
                    class: "px-4 py-2 text-xs text-gray-500 dark:text-gray-400 border-t border-gray-200 dark:border-gray-700",
                    "Type to edit..."
                }
            }
        }
    }
}

/// Simple editor variant with minimal toolbar.
#[component]
pub fn SimpleEditor(
    #[props(default)] initial_value: Option<String>,
    on_change: EventHandler<String>,
    #[props(default = "Write something...".to_string())] placeholder: String,
) -> Element {
    rsx! {
        RichTextEditor {
            initial_value,
            on_change,
            placeholder,
            class: "simple-editor",
        }
    }
}

/// Read-only viewer for rendered content.
#[component]
pub fn ContentViewer(value: String, #[props(default = String::new())] class: String) -> Element {
    let doc = use_memo(move || serde_json::from_str::<Doc>(&value).unwrap_or_default());

    let html_content = use_memo(move || render_doc(&doc.read()));

    rsx! {
        div {
            class: "content-viewer prose prose-sm dark:prose-invert max-w-none {class}",
            dangerous_inner_html: "{html_content}",
        }
    }
}

/// Strip HTML tags for character counting.
fn strip_html_tags(html: &str) -> String {
    let re = regex::Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(html, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_strips_html_tags() {
        let html = "<p>Hello <strong>world</strong></p>";
        let stripped = strip_html_tags(html);
        assert_eq!(stripped, "Hello world");
    }

    #[test]
    fn it_parses_initial_value() {
        let json = r#"{"blocks":[{"id":"test","kind":{"type":"paragraph"},"align":"left","attrs":{},"children":[{"type":"text","text":"Test","marks":{}}]}]}"#;
        let doc: Doc = serde_json::from_str(json).unwrap();
        assert_eq!(doc.blocks.len(), 1);
    }

    #[test]
    fn it_renders_empty_content() {
        let doc = Doc::default();
        let html = render_doc(&doc);
        // Should have at least the default paragraph
        assert!(!html.is_empty());
    }
}
