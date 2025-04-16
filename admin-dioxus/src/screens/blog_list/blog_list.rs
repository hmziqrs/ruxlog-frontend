use crate::components::shadcn_ui::*;
use crate::store::{use_post, Post};
use dioxus::html::div;
use dioxus::prelude::*;
use hmziq_dioxus_free_icons::icons::ld_icons::{
    LdCalendar, LdEye, LdLayoutGrid, LdHeart, LdLayoutList, LdMessageSquare, 
    LdEllipsis, LdSearch, LdTag
};
use hmziq_dioxus_free_icons::Icon;

#[derive(Clone, Copy, PartialEq)]
pub enum LayoutType {
    Grid,
    List,
}

#[component]
pub fn BlogListScreen() -> Element {
    let post_state = use_post();
    let list_signal = post_state.list.read();
    let mut layout_type = use_signal(|| LayoutType::Grid);
    let mut search_query = use_signal(|| String::new());

    // Fetch posts on mount
    use_effect(move || {
        spawn(async move {
            post_state.list().await;
        });
    });

    let frame = &*list_signal;

    // Filter posts by search query
    let filtered_posts = match &frame.data {
        Some(posts) => posts
            .iter()
            .filter(|post| {
                let q = search_query.read().to_lowercase();
                post.title.to_lowercase().contains(&q)
                    || post
                        .excerpt
                        .as_ref()
                        .map(|e| e.to_lowercase().contains(&q))
                        .unwrap_or(false)
                    || post.author.name.to_lowercase().contains(&q)
            })
            .cloned()
            .collect::<Vec<_>>(),
        _ => vec![],
    };

    rsx! {
        div { class: "min-h-screen bg-zinc-50 dark:bg-zinc-950 text-zinc-900 dark:text-zinc-50",
            div { class: "container mx-auto py-8 px-4",
                div { class: "flex flex-col gap-6",
                    // Header
                    div { class: "flex flex-col gap-4 md:flex-row md:items-center md:justify-between",
                        div {
                            h1 { class: "text-3xl font-bold tracking-tight", "Posts" }
                            p { class: "text-zinc-500 dark:text-zinc-400 mt-1",
                                "Manage and view your blog posts"
                            }
                        }
                        div { class: "flex items-center gap-2",
                            // TODO: ThemeToggle
                            // ThemeToggle {}
                            Card { "Create Post" }
                        }
                    }
                    // Search and view mode
                    div { class: "flex flex-col gap-4 md:flex-row md:items-center md:justify-between",
                        div { class: "relative w-full md:w-96",
                            div { class: "absolute left-2.5 top-2.5 h-4 w-4 text-zinc-500 dark:text-zinc-400",
                                div { class: "w-4 h-4",
                                    Icon { icon: LdSearch {} }
                                }
                            }
                            input {
                                class: "w-full pl-8 bg-white dark:bg-zinc-900 border-zinc-200 dark:border-zinc-800 rounded border px-3 py-2 text-sm",
                                r#type: "search",
                                placeholder: "Search posts...",
                                value: "{search_query}",
                                oninput: move |e| search_query.set(e.value()),
                            }
                        }
                        div { class: "flex items-center gap-2",
                            button {
                                class: if *layout_type.read() == LayoutType::Grid { "h-9 w-9 bg-zinc-900 dark:bg-zinc-700 text-white rounded flex items-center justify-center" } else { "h-9 w-9 border border-zinc-200 dark:border-zinc-800 rounded flex items-center justify-center" },
                                onclick: move |_| layout_type.set(LayoutType::Grid),
                                div { class: "w-4 h-4",
                                    Icon { icon: LdLayoutGrid {} }
                                }
                                span { class: "sr-only", "Grid view" }
                            }
                            button {
                                class: if *layout_type.read() == LayoutType::List { "h-9 w-9 bg-zinc-900 dark:bg-zinc-700 text-white rounded flex items-center justify-center" } else { "h-9 w-9 border border-zinc-200 dark:border-zinc-800 rounded flex items-center justify-center" },
                                onclick: move |_| layout_type.set(LayoutType::List),
                                div { class: "w-4 h-4",
                                    Icon { icon: LdLayoutList {} }
                                }
                                span { class: "sr-only", "List view" }
                            }
                        }
                    }
                    // Posts
                    match *layout_type.read() {
                        LayoutType::Grid => rsx! {
                            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                                {filtered_posts.iter().map(|post| rsx! {
                                    PostGridCard { post: post.clone() }
                                })}
                            }
                        },
                        LayoutType::List => rsx! {
                            div { class: "flex flex-col gap-4",
                                {filtered_posts.iter().map(|post| rsx! {
                                    PostListItem { post: post.clone() }
                                })}
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn PostGridCard(post: Post) -> Element {
    rsx! {
        Card {
            // TODO: Featured image
            CardHeader {
                div { class: "flex items-start justify-between",
                    div { class: "space-y-1.5",
                        // TODO: Category badge
                        h3 { class: "font-semibold text-lg line-clamp-2", "{post.title}" }
                    }
                    DropdownMenu {
                        DropdownMenuTrigger { "⋮" }
                        DropdownMenuContent {
                            DropdownMenuItem { "Edit" }
                            DropdownMenuItem { "Duplicate" }
                            DropdownMenuItem { "Delete" }
                        }
                    }
                }
            }
            CardContent {
                if let Some(excerpt) = &post.excerpt {
                    p { class: "text-zinc-500 dark:text-zinc-400 text-sm line-clamp-2 mt-1",
                        "{excerpt}"
                    }
                }
                        // TODO: Tags
            }
            CardFooter {
                div { class: "flex items-center gap-2",
                    Avatar {
                        AvatarImage {
                            src: post.author.avatar.clone().unwrap_or_default(),
                            alt: post.author.name.clone(),
                        }
                        AvatarFallback { {"{post.author.name.chars().next().unwrap_or('U')}"} }
                    }
                    span { class: "text-xs text-zinc-500 dark:text-zinc-400", "{post.author.name}" }
                }
                        // TODO: Views, Likes
            }
        }
    }
}

#[component]
fn PostListItem(post: Post) -> Element {
    rsx! {
        Card {
            div { class: "flex flex-col md:flex-row",
                // TODO: Featured image
                div { class: "flex-1 p-4",
                    div { class: "flex items-start justify-between",
                        div { class: "space-y-1",
                            // TODO: Category badge, status
                            h3 { class: "font-semibold text-lg", "{post.title}" }
                        }
                        DropdownMenu {
                            DropdownMenuTrigger { "⋮" }
                            DropdownMenuContent {
                                DropdownMenuItem { "Edit" }
                                DropdownMenuItem { "Duplicate" }
                                DropdownMenuItem { "Delete" }
                            }
                        }
                    }
                    if let Some(excerpt) = &post.excerpt {
                        p { class: "text-zinc-500 dark:text-zinc-400 text-sm mt-2 line-clamp-2",
                            "{excerpt}"
                        }
                    }
                    // TODO: Tags
                    div { class: "flex flex-col sm:flex-row sm:items-center justify-between mt-4 pt-4 border-t border-zinc-200 dark:border-zinc-800",
                        div { class: "flex items-center gap-2",
                            Avatar {
                                AvatarImage {
                                    src: post.author.avatar.clone().unwrap_or_default(),
                                    alt: post.author.name.clone(),
                                }
                                AvatarFallback { {"{post.author.name.chars().next().unwrap_or('U')}"} }
                            }
                            span { class: "text-xs text-zinc-500 dark:text-zinc-400",
                                "{post.author.name}"
                            }
                        }
                                        // TODO: Date, Views, Likes, Tags count
                    }
                }
            }
        }
    }
}
