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

use commands::{InsertBlock, InsertLink, SetBlockType};
use dioxus::prelude::*;
use toolbar::Toolbar;
use wasm_bindgen::JsCast;

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

    let selection = use_signal(|| Selection::collapsed(Position::start()));
    let mut is_focused = use_signal(|| false);

    // Execute a command using browser's native execCommand
    let execute_command = move |cmd: Box<dyn Command>| {
        let Some(document) = current_document() else {
            return;
        };

        if let Some(toggle_mark) = cmd.as_any().downcast_ref::<ToggleMark>() {
            if handle_toggle_mark(&document, toggle_mark.mark_type) {
                return;
            }
        }

        if let Some(set_block) = cmd.as_any().downcast_ref::<SetBlockType>() {
            handle_set_block_type(&document, &set_block.block_type);
            return;
        }

        if let Some(insert_block) = cmd.as_any().downcast_ref::<InsertBlock>() {
            handle_insert_block(&document, &insert_block.block_type);
            return;
        }

        if let Some(insert_link) = cmd.as_any().downcast_ref::<InsertLink>() {
            handle_insert_link(&document, insert_link);
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

fn current_document() -> Option<web_sys::Document> {
    web_sys::window().and_then(|window| window.document())
}

fn exec_command(document: &web_sys::Document, command: &str) {
    if let Some(html_document) = document.dyn_ref::<web_sys::HtmlDocument>() {
        let _ = html_document.exec_command(command);
    }
}

fn exec_command_with_value(document: &web_sys::Document, command: &str, value: &str) {
    if let Some(html_document) = document.dyn_ref::<web_sys::HtmlDocument>() {
        let _ = html_document.exec_command_with_show_ui_and_value(command, false, value);
    }
}

fn handle_toggle_mark(document: &web_sys::Document, mark: MarkType) -> bool {
    match mark {
        MarkType::Bold => {
            exec_command(document, "bold");
            true
        }
        MarkType::Italic => {
            exec_command(document, "italic");
            true
        }
        MarkType::Underline => {
            exec_command(document, "underline");
            true
        }
        MarkType::Strike => {
            exec_command(document, "strikeThrough");
            true
        }
        MarkType::Code => {
            toggle_inline_code(document);
            true
        }
    }
}

fn handle_set_block_type(document: &web_sys::Document, block_kind: &BlockKind) {
    match block_kind {
        BlockKind::Paragraph => {
            exec_command_with_value(document, "formatBlock", "<p>");
        }
        BlockKind::Heading { level } => {
            let level = (*level).clamp(1, 6);
            let tag = format!("<h{}>", level);
            exec_command_with_value(document, "formatBlock", &tag);
        }
        BlockKind::Quote => {
            exec_command_with_value(document, "formatBlock", "<blockquote>");
        }
        BlockKind::CodeBlock { .. } => {
            exec_command_with_value(document, "formatBlock", "<pre>");
            ensure_pre_has_code(document);
        }
        BlockKind::BulletList { .. } => {
            exec_command(document, "insertUnorderedList");
        }
        BlockKind::OrderedList { .. } => {
            exec_command(document, "insertOrderedList");
        }
        BlockKind::TaskList { .. } => {
            exec_command(document, "insertUnorderedList");
        }
        BlockKind::ListItem
        | BlockKind::TaskItem { .. }
        | BlockKind::Image { .. }
        | BlockKind::Embed { .. }
        | BlockKind::Rule => {}
    }
}

fn handle_insert_block(document: &web_sys::Document, block_kind: &BlockKind) {
    match block_kind {
        BlockKind::Rule => {
            exec_command(document, "insertHorizontalRule");
        }
        BlockKind::Image {
            src,
            alt,
            title,
            width,
            height,
            caption,
        } => {
            let html = build_image_html(
                src,
                alt.as_deref(),
                title.as_deref(),
                *width,
                *height,
                caption.as_deref(),
            );
            insert_html(document, &html);
        }
        BlockKind::Embed {
            provider,
            url,
            title,
            width,
            height,
        } => {
            let html = build_embed_html(provider, url, title.as_deref(), *width, *height);
            insert_html(document, &html);
        }
        BlockKind::Paragraph
        | BlockKind::Heading { .. }
        | BlockKind::BulletList { .. }
        | BlockKind::OrderedList { .. }
        | BlockKind::TaskList { .. }
        | BlockKind::ListItem
        | BlockKind::TaskItem { .. }
        | BlockKind::Quote
        | BlockKind::CodeBlock { .. } => {}
    }
}

fn handle_insert_link(document: &web_sys::Document, command: &InsertLink) {
    let href = command.href.trim();
    if href.is_empty() {
        return;
    }

    exec_command_with_value(document, "createLink", href);

    if let Ok(Some(selection)) = document.get_selection() {
        if let Some(anchor_node) = selection.anchor_node() {
            if let Some(link_element) = find_ancestor_element(&anchor_node, "a") {
                let _ = link_element.set_attribute("href", href);

                if let Some(title) = command
                    .title
                    .as_ref()
                    .map(|value| value.trim())
                    .filter(|value| !value.is_empty())
                {
                    let _ = link_element.set_attribute("title", title);
                } else {
                    let _ = link_element.remove_attribute("title");
                }

                if command.target_blank {
                    let _ = link_element.set_attribute("target", "_blank");
                    let _ = link_element.set_attribute("rel", "noopener noreferrer");
                } else {
                    let _ = link_element.remove_attribute("target");
                    let _ = link_element.remove_attribute("rel");
                }
            }
        }
    }
}

fn insert_html(document: &web_sys::Document, html: &str) {
    exec_command_with_value(document, "insertHTML", html);
}

fn toggle_inline_code(document: &web_sys::Document) {
    if selection_in_code(document) {
        remove_inline_code(document);
    } else {
        apply_inline_code(document);
    }
}

fn apply_inline_code(document: &web_sys::Document) {
    if let Ok(Some(selection)) = document.get_selection() {
        let selected_text = selection.to_string().as_string().unwrap_or_default();
        let use_placeholder = selected_text.trim().is_empty();
        let content = if use_placeholder {
            "code".to_string()
        } else {
            selected_text
        };
        let html = format!("<code>{}</code>", escape_html(&content));
        exec_command_with_value(document, "insertHTML", &html);
    }
}

fn remove_inline_code(document: &web_sys::Document) {
    if let Ok(Some(selection)) = document.get_selection() {
        if let Some(anchor) = selection.anchor_node() {
            if let Some(code_element) = find_ancestor_element(&anchor, "code") {
                unwrap_element(code_element);
            }
        }
        if let Some(focus) = selection.focus_node() {
            if let Some(code_element) = find_ancestor_element(&focus, "code") {
                unwrap_element(code_element);
            }
        }
    }
}

fn ensure_pre_has_code(document: &web_sys::Document) {
    if let Ok(Some(selection)) = document.get_selection() {
        if let Some(anchor) = selection.anchor_node() {
            if let Some(pre_element) = find_ancestor_element(&anchor, "pre") {
                if pre_element.query_selector("code").ok().flatten().is_none() {
                    if let Ok(code_element) = document.create_element("code") {
                        while let Some(child) = pre_element.first_child() {
                            let _ = code_element.append_child(&child);
                        }
                        let _ = pre_element.append_child(&code_element);
                    }
                }
            }
        }
    }
}

fn find_ancestor_element(node: &web_sys::Node, tag: &str) -> Option<web_sys::Element> {
    let mut current: Option<web_sys::Node> = Some(node.clone());

    while let Some(node) = current {
        if let Some(element) = node.dyn_ref::<web_sys::Element>() {
            if element.tag_name().eq_ignore_ascii_case(tag) {
                return Some(element.clone());
            }
        }
        current = node.parent_node();
    }

    None
}

fn unwrap_element(element: web_sys::Element) {
    if let Some(parent) = element.parent_node() {
        while let Some(child) = element.first_child() {
            let _ = parent.insert_before(&child, Some(&element));
        }
        let _ = parent.remove_child(&element);
    }
}

pub(crate) fn selection_in_code(document: &web_sys::Document) -> bool {
    if let Ok(Some(selection)) = document.get_selection() {
        if let Some(anchor) = selection.anchor_node() {
            if find_ancestor_element(&anchor, "code").is_some() {
                return true;
            }
        }
        if let Some(focus) = selection.focus_node() {
            if find_ancestor_element(&focus, "code").is_some() {
                return true;
            }
        }
    }
    false
}

fn build_image_html(
    src: &str,
    alt: Option<&str>,
    title: Option<&str>,
    width: Option<u32>,
    height: Option<u32>,
    caption: Option<&str>,
) -> String {
    let src_attr = escape_attribute(src);
    let alt_attr = escape_attribute(alt.unwrap_or(""));
    let title_attr = title
        .map(|value| format!(" title=\"{}\"", escape_attribute(value)))
        .unwrap_or_default();
    let width_attr = width
        .map(|value| format!(" width=\"{}\"", value))
        .unwrap_or_default();
    let height_attr = height
        .map(|value| format!(" height=\"{}\"", value))
        .unwrap_or_default();

    let img_tag = format!(
        "<img src=\"{}\" alt=\"{}\"{}{}{} class=\"editor-image max-w-full rounded-md\" loading=\"lazy\" />",
        src_attr, alt_attr, title_attr, width_attr, height_attr
    );

    if let Some(caption_text) = caption.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(escape_html(trimmed))
        }
    }) {
        format!(
            "<figure class=\"editor-figure text-center\">{}<figcaption class=\"mt-2 text-sm text-gray-500 dark:text-gray-400\">{}</figcaption></figure>",
            img_tag, caption_text
        )
    } else {
        img_tag
    }
}

fn build_embed_html(
    provider: &EmbedProvider,
    url: &str,
    title: Option<&str>,
    width: Option<u32>,
    height: Option<u32>,
) -> String {
    let src_attr = escape_attribute(url);
    let title_attr = escape_attribute(title.unwrap_or(match provider {
        EmbedProvider::Youtube => "YouTube embed",
        EmbedProvider::X => "X embed",
        EmbedProvider::Generic => "Embedded content",
    }));
    let width_attr = width
        .map(|value| format!(" width=\"{}\"", value))
        .unwrap_or_default();
    let height_attr = height
        .map(|value| format!(" height=\"{}\"", value))
        .unwrap_or_default();

    let allow_attr = match provider {
        EmbedProvider::Youtube => "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share",
        EmbedProvider::X => "clipboard-write",
        EmbedProvider::Generic => "accelerometer; autoplay; encrypted-media",
    };

    format!(
        "<div class=\"editor-embed aspect-video\"><iframe src=\"{}\" title=\"{}\" frameborder=\"0\" allow=\"{}\" allowfullscreen{}{}></iframe></div>",
        src_attr, title_attr, allow_attr, width_attr, height_attr
    )
}

fn escape_attribute(value: &str) -> String {
    escape_html(value)
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
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
#[allow(dead_code)]
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
