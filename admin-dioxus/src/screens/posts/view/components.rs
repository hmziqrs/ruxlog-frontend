use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::PageHeader;
use crate::router::Route;
use crate::store::use_post;
use crate::ui::shadcn::{Button, ButtonVariant};

// ============================================================================
// EditorJS Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EditorJsData {
    time: u64,
    blocks: Vec<EditorJsBlock>,
    version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum EditorJsBlock {
    #[serde(rename = "header")]
    Header {
        id: String,
        data: HeaderBlock,
    },
    #[serde(rename = "paragraph")]
    Paragraph {
        id: String,
        data: ParagraphBlock,
    },
    #[serde(rename = "code")]
    Code {
        id: String,
        data: CodeBlock,
    },
    #[serde(rename = "quote")]
    Quote {
        id: String,
        data: QuoteBlock,
    },
    #[serde(rename = "alert")]
    Alert {
        id: String,
        data: AlertBlock,
    },
    #[serde(rename = "checklist")]
    Checklist {
        id: String,
        data: ChecklistBlock,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HeaderBlock {
    text: String,
    level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ParagraphBlock {
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CodeBlock {
    code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuoteBlock {
    text: String,
    caption: Option<String>,
    alignment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AlertBlock {
    #[serde(rename = "type")]
    alert_type: String,
    align: String,
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChecklistItem {
    text: String,
    checked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChecklistBlock {
    items: Vec<ChecklistItem>,
}

// ============================================================================
// EditorJS Block Renderers
// ============================================================================

fn render_header_block(block: &EditorJsBlock) -> Element {
    if let EditorJsBlock::Header { data, .. } = block {
        let level = data.level;
        let text = data.text.clone();

        match level {
            1 => rsx! { h1 { class: "text-4xl font-bold mb-6", "{text}" } },
            2 => rsx! { h2 { class: "text-3xl font-bold mb-5", "{text}" } },
            3 => rsx! { h3 { class: "text-2xl font-bold mb-4", "{text}" } },
            4 => rsx! { h4 { class: "text-xl font-bold mb-3", "{text}" } },
            5 => rsx! { h5 { class: "text-lg font-bold mb-2", "{text}" } },
            6 => rsx! { h6 { class: "text-base font-bold mb-2", "{text}" } },
            _ => rsx! { h1 { class: "text-4xl font-bold mb-6", "{text}" } },
        }
    } else {
        rsx! {}
    }
}

fn render_paragraph_block(block: &EditorJsBlock) -> Element {
    if let EditorJsBlock::Paragraph { data, .. } = block {
        // Simple HTML entity decode for &nbsp; etc.
        let text = data.text
            .replace("&nbsp;", " ")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&");

        rsx! {
            p { class: "mb-4 leading-7", dangerous_inner_html: "{text}" }
        }
    } else {
        rsx! {}
    }
}

fn render_code_block(block: &EditorJsBlock) -> Element {
    if let EditorJsBlock::Code { data, .. } = block {
        let code = data.code.clone();
        rsx! {
            div { class: "my-6 rounded-lg overflow-hidden border bg-muted/50",
                pre { class: "p-4 overflow-x-auto text-sm",
                    code { class: "font-mono", "{code}" }
                }
            }
        }
    } else {
        rsx! {}
    }
}

fn render_quote_block(block: &EditorJsBlock) -> Element {
    if let EditorJsBlock::Quote { data, .. } = &block {
        let alignment = match data.alignment.as_str() {
            "center" => "text-center",
            "right" => "text-right",
            _ => "text-left",
        };
        let text = data.text.clone();
        let caption = data.caption.clone();

        rsx! {
            blockquote { class: "my-6 pl-4 border-l-4 border-muted italic",
                p { class: "text-lg mb-2 {alignment}", "{text}" }
                if let Some(caption) = caption {
                    footer { class: "text-sm text-muted-foreground {alignment}", "{caption}" }
                }
            }
        }
    } else {
        rsx! {}
    }
}

fn render_alert_block(block: &EditorJsBlock) -> Element {
    if let EditorJsBlock::Alert { data, .. } = &block {
        let alert_style = match data.alert_type.as_str() {
            "success" => "bg-green-50 border-green-200 text-green-800 dark:bg-green-950/20 dark:border-green-800 dark:text-green-300",
            "warning" => "bg-yellow-50 border-yellow-200 text-yellow-800 dark:bg-yellow-950/20 dark:border-yellow-800 dark:text-yellow-300",
            "error" => "bg-red-50 border-red-200 text-red-800 dark:bg-red-950/20 dark:border-red-800 dark:text-red-300",
            _ => "bg-blue-50 border-blue-200 text-blue-800 dark:bg-blue-950/20 dark:border-blue-800 dark:text-blue-300", // info
        };

        let alignment = match data.align.as_str() {
            "center" => "text-center",
            "right" => "text-right",
            _ => "text-left",
        };
        let message = data.message.clone();

        rsx! {
            div { class: "my-6 p-4 rounded-lg border {alert_style} {alignment}",
                p { "{message}" }
            }
        }
    } else {
        rsx! {}
    }
}

fn render_checklist_block(block: &EditorJsBlock) -> Element {
    if let EditorJsBlock::Checklist { data, .. } = &block {
        rsx! {
            ul { class: "my-6 space-y-2",
                for item in &data.items {
                    li { class: "flex items-start gap-2",
                        input {
                            type: "checkbox",
                            checked: item.checked,
                            class: "mt-1",
                            readonly: true
                        }
                        span {
                            class: if item.checked { "flex-1 line-through text-muted-foreground" } else { "flex-1" },
                            "{item.text}"
                        }
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}

fn render_unknown_block(block: &EditorJsBlock) -> Element {
    if let EditorJsBlock::Unknown { .. } = block {
        rsx! {
            div { class: "my-4 p-3 rounded bg-muted text-sm",
                "Unsupported block type"
            }
        }
    } else {
        rsx! {}
    }
}

fn render_editorjs_content(content: &str) -> Element {
    match serde_json::from_str::<EditorJsData>(content) {
        Ok(data) => {
            rsx! {
                div { class: "prose prose-neutral dark:prose-invert max-w-none",
                    for block in &data.blocks {
                        match block {
                            EditorJsBlock::Header { .. } => render_header_block(block),
                            EditorJsBlock::Paragraph { .. } => render_paragraph_block(block),
                            EditorJsBlock::Code { .. } => render_code_block(block),
                            EditorJsBlock::Quote { .. } => render_quote_block(block),
                            EditorJsBlock::Alert { .. } => render_alert_block(block),
                            EditorJsBlock::Checklist { .. } => render_checklist_block(block),
                            EditorJsBlock::Unknown { .. } => render_unknown_block(block),
                        }
                    }
                }
            }
        }
        Err(_) => {
            // Fallback to raw content
            rsx! {
                div { class: "prose prose-neutral dark:prose-invert max-w-none",
                    p { class: "text-muted-foreground", "Failed to parse content" }
                }
            }
        }
    }
}

// ============================================================================
// Post View Screen
// ============================================================================

#[component]
pub fn PostsViewScreen(id: i32) -> Element {
    let posts = use_post();
    let nav = use_navigator();

    // Get post by id
    let post = use_memo(move || {
        let posts_read = posts.list.read();
        if let Some(list) = &posts_read.data {
            list.data.iter().find(|p| p.id == id).cloned()
        } else {
            None
        }
    });

    use_effect(move || {
        if post().is_none() {
            let posts_state = posts;
            spawn(async move {
                posts_state.list().await;
            });
        }
    });

    if let Some(post) = post() {
        let published_date = post.published_at
            .map(|dt| dt.format("%B %d, %Y").to_string())
            .unwrap_or_else(|| post.created_at.format("%B %d, %Y").to_string());

        rsx! {
            div { class: "min-h-screen bg-transparent text-foreground",
                PageHeader {
                    title: post.title.clone(),
                    description: post.excerpt.clone().unwrap_or_default(),
                    actions: Some(rsx! {
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| { nav.push(Route::PostsListScreen {}); },
                            "Back to Posts"
                        }
                    }),
                }

                div { class: "container mx-auto px-4 py-10 md:py-12",
                    // Featured image
                    if let Some(image) = &post.featured_image {
                        div { class: "mb-8 overflow-hidden rounded-lg",
                            img {
                                src: "{image.file_url}",
                                alt: "{post.title}",
                                class: "w-full h-auto object-cover"
                            }
                        }
                    }

                    // Post meta
                    div { class: "mb-8 pb-4 border-b",
                        div { class: "flex flex-wrap items-center gap-4 text-sm text-muted-foreground",
                            span { "By {post.author.name}" }
                            span { "•" }
                            span { "Published on {published_date}" }
                            if !post.tags.is_empty() {
                                span { "•" }
                                div { class: "flex flex-wrap gap-2",
                                    for tag in &post.tags {
                                        span { class: "px-2 py-1 rounded-full text-xs bg-muted", "{tag.name}" }
                                    }
                                }
                            }
                        }
                    }

                    // Post content
                    div { class: "prose prose-neutral dark:prose-invert max-w-none",
                        {render_editorjs_content(&post.content)}
                    }

                    // Post stats
                    div { class: "mt-10 pt-6 border-t text-sm text-muted-foreground",
                        div { class: "flex flex-wrap gap-6",
                            span { "{post.view_count} views" }
                            span { "{post.likes_count} likes" }
                            span { "{post.comment_count} comments" }
                            span { "Status: {post.status}" }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div { class: "min-h-screen bg-transparent text-foreground",
                PageHeader {
                    title: "Loading...".to_string(),
                    description: "Fetching post content.".to_string(),
                    actions: Some(rsx! {
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| { nav.push(Route::PostsListScreen {}); },
                            "Back to Posts"
                        }
                    }),
                }
            }
        }
    }
}
