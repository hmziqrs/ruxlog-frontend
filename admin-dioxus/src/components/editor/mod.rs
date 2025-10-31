//! Rich text editor component for Dioxus.
//!
//! This module provides a full-featured WYSIWYG editor with an AST-first architecture.

pub mod ast;
pub mod commands;
pub mod renderer;
pub mod sanitizer;
pub mod toolbar;

pub use ast::{Block, BlockAlign, BlockKind, Doc, EmbedProvider, Inline, Link, MarkSet, TextSize};
pub use commands::{Command, CommandError, MarkType, Position, Selection};
pub use renderer::render_doc;
pub use sanitizer::sanitize_html;

use dioxus::prelude::*;
use toolbar::Toolbar;

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
    // Editor state
    let mut doc = use_signal(|| {
        if let Some(json) = &props.initial_value {
            serde_json::from_str::<Doc>(json).unwrap_or_default()
        } else {
            Doc::default()
        }
    });

    let mut selection = use_signal(|| Selection::collapsed(Position::start()));
    let mut is_focused = use_signal(|| false);

    // Execute a command
    let mut execute_command = move |cmd: Box<dyn Command>| {
        let mut current_doc = doc.read().clone();
        let current_selection = selection.read().clone();

        if let Ok(new_selection) = cmd.execute(&mut current_doc, &current_selection) {
            doc.set(current_doc.clone());
            selection.set(new_selection);

            // Notify parent of change
            if let Ok(json) = serde_json::to_string(&current_doc) {
                props.on_change.call(json);
            }
        }
    };

    // Render HTML from AST
    let html_content = use_memo(move || render_doc(&doc.read()));

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

            // Editor content area
            div {
                class: "editor-content p-4 min-h-[300px] focus:outline-none text-gray-900 dark:text-gray-100",
                contenteditable: if props.readonly { "false" } else { "true" },
                tabindex: "0",

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

                    // Parse HTML back to AST
                    // For now, we'll use a simplified approach
                    // In production, you'd implement a proper HTML->AST parser
                    let value = evt.value();
                    // Sanitize the HTML
                    let clean = sanitize_html(&value);

                    // Notify parent (simplified - in production parse to AST first)
                    props.on_change.call(clean);
                },

                // Render the document
                dangerous_inner_html: "{html_content}",

                // Show placeholder when empty
                if doc.read().blocks.is_empty() ||
                   (doc.read().blocks.len() == 1 && doc.read().blocks[0].children.is_empty()) {
                    div {
                        class: "text-gray-400 dark:text-gray-500 pointer-events-none absolute",
                        "{props.placeholder}"
                    }
                }
            }

            // Character count (optional)
            if *is_focused.read() {
                div {
                    class: "px-4 py-2 text-xs text-gray-500 dark:text-gray-400 border-t border-gray-200 dark:border-gray-700",
                    {
                        let char_count = count_characters(&doc.read());
                        format!("{} characters", char_count)
                    }
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

/// Counts total characters in the document.
fn count_characters(doc: &Doc) -> usize {
    let mut count = 0;
    for block in &doc.blocks {
        for inline in &block.children {
            if let Inline::Text { text, .. } = inline {
                count += text.chars().count();
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_characters() {
        let mut doc = Doc::default();
        doc.blocks[0] = Block::new_paragraph().with_text("Hello world");
        assert_eq!(count_characters(&doc), 11);
    }

    #[test]
    fn it_parses_initial_value() {
        let json = r#"{"blocks":[{"id":"test","kind":{"type":"paragraph"},"align":"left","attrs":{},"children":[{"type":"text","text":"Test","marks":{}}]}]}"#;
        let doc: Doc = serde_json::from_str(json).unwrap();
        assert_eq!(doc.blocks.len(), 1);
    }
}
