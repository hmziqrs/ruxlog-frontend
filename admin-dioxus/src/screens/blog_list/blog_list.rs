use dioxus::prelude::*;

use crate::store::{use_post, Post};

#[derive(Clone, Copy, PartialEq)]
pub enum LayoutType {
    Grid,
    List,
}

#[component]
pub fn PostCard(post: Post, layout_type: LayoutType) -> Element {
    let base_class = match layout_type {
        LayoutType::Grid => "group border border-zinc-200 dark:border-zinc-800 rounded-xl bg-zinc-50 dark:bg-zinc-950 p-5 shadow hover:shadow-xl transition-all duration-200 flex flex-col gap-2 cursor-pointer hover:bg-zinc-100 dark:hover:bg-zinc-900",
        LayoutType::List => "group border border-zinc-200 dark:border-zinc-800 rounded-xl bg-zinc-50 dark:bg-zinc-950 p-5 shadow hover:shadow-xl transition-all duration-200 flex flex-row gap-4 items-center cursor-pointer hover:bg-zinc-100 dark:hover:bg-zinc-900",
    };
    rsx! {
        div { key: post.id, class: base_class,
            h2 { class: "text-lg font-bold text-zinc-900 dark:text-zinc-100 group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors duration-150",
                "{post.title}"
            }
            p { class: "text-sm text-zinc-500 dark:text-zinc-400 mb-1 line-clamp-2",
                "{post.excerpt.clone().unwrap_or_default()}"
            }
            div { class: "flex items-center gap-2 text-xs text-zinc-400 dark:text-zinc-500 mt-auto pt-2 border-t border-zinc-100 dark:border-zinc-800",
                span { {"By {post.author.name}"} }
                span { {"·"} }
                span { {"{post.created_at}"} }
            }
        }
    }
}

#[component]
pub fn BlogListScreen() -> Element {
    let post_state = use_post();
    let list_signal = post_state.list.read();
    let  mut layout_type = use_signal(|| LayoutType::Grid);

    // Fetch posts on mount
    use_effect(move || {
        spawn(async move {
            post_state.list().await;
        });
    });

    let frame = &*list_signal;

    rsx! {
        div { class: "container mx-auto p-6 min-h-screen transition-colors duration-300",
            // Header
            div { class: "mb-8 flex flex-col md:flex-row md:items-center md:justify-between gap-2",
                h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100",
                    "Posts"
                }
                div { class: "flex gap-2 items-center",
                    button { class: "btn btn-primary px-5 py-2 rounded-lg bg-zinc-900 text-white dark:bg-zinc-100 dark:text-zinc-900 font-semibold shadow hover:bg-zinc-800 dark:hover:bg-zinc-200 transition-colors duration-150",
                        // TODO: Add navigation to new post
                        "New Post"
                    }
                    button {
                        class: format!(
                            "px-3 py-2 rounded-lg border text-xs font-semibold {}",
                            if *layout_type.read() == LayoutType::Grid {
                                "bg-zinc-900 text-white dark:bg-zinc-100 dark:text-zinc-900"
                            } else {
                                "bg-white dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100 border-zinc-200 dark:border-zinc-800"
                            },
                        ),
                        onclick: move |_| layout_type.set(LayoutType::Grid),
                        "Grid"
                    }
                    button {
                        class: format!(
                            "px-3 py-2 rounded-lg border text-xs font-semibold {}",
                            if *layout_type.read() == LayoutType::List {
                                "bg-zinc-900 text-white dark:bg-zinc-100 dark:text-zinc-900"
                            } else {
                                "bg-white dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100 border-zinc-200 dark:border-zinc-800"
                            },
                        ),
                        onclick: move |_| layout_type.set(LayoutType::List),
                        "List"
                    }
                }
            }
            // Card container for posts
            div { class: "bg-white/80 dark:bg-zinc-900/80 border border-zinc-200 dark:border-zinc-800 rounded-2xl shadow-lg p-6 transition-colors duration-300",
                match &frame.data {
                    Some(posts) if !posts.is_empty() => rsx! {
                        {
                            match *layout_type.read() {
                                LayoutType::Grid => rsx! {
                                    div { class: "grid gap-6 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4",
                                        {posts.iter().map(|post| rsx! {
                                            PostCard { post: post.clone(), layout_type: LayoutType::Grid }
                                        })}
                                    }
                                },
                                LayoutType::List => rsx! {
                                    div { class: "flex flex-col gap-4",
                                        {posts.iter().map(|post| rsx! {
                                            PostCard { post: post.clone(), layout_type: LayoutType::List }
                                        })}
                                    }
                                },
                            }
                        }
                    },
                    Some(_) => rsx! {
                        p { class: "text-center text-zinc-400 py-12", "No posts found." }
                    },
                    None if frame.is_loading() => rsx! {
                        div { class: if *layout_type.read() == LayoutType::Grid { "grid gap-6 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4" } else { "flex flex-col gap-4" },
                            {(0..6).map(|i| rsx! {
                                div {
                                    key: "skeleton-{i}",
                                    class: "animate-pulse border border-zinc-200 dark:border-zinc-800 rounded-xl bg-zinc-100 dark:bg-zinc-900 p-5 shadow flex flex-col gap-2",
                                    div { class: "h-5 w-3/4 bg-zinc-200 dark:bg-zinc-800 rounded mb-2" }
                                    div { class: "h-3 w-full bg-zinc-200 dark:bg-zinc-800 rounded mb-1" }
                                    div { class: "h-3 w-1/2 bg-zinc-200 dark:bg-zinc-800 rounded" }
                                }
                            })}
                        }
                    },
                    None if frame.is_failed() => rsx! {
                        p { class: "text-center text-red-500 py-12", "Failed to load posts." }
                    },
                    _ => rsx! {
                        p { class: "text-center text-zinc-400 py-12", "No data." }
                    },
                }
            }
        }
    }
}
