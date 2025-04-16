use dioxus::prelude::*;

use crate::store::use_post;

#[component]
pub fn BlogListScreen() -> Element {
    let post_state = use_post();
    let list_signal = post_state.list.read();

    // Fetch posts on mount
    use_effect(move || {
        spawn(async move {
            post_state.list().await;
        });
    });

    let frame = &*list_signal;

    rsx! {
        div { class: "container mx-auto p-6",
            h1 { class: "text-2xl font-bold mb-4", "Posts" }
            match &frame.data {
                Some(posts) if !posts.is_empty() => rsx! {
                    ul { class: "space-y-4",
                        {posts.iter().map(|post| rsx! {
                            li {
                                key: "{post.id}",
                                class: "border rounded p-4 bg-white dark:bg-zinc-900 shadow",
                                h2 { class: "text-lg font-semibold", "{post.title}" }
                                p { class: "text-sm text-zinc-500 mb-2", "{post.excerpt.clone().unwrap_or_default()}" }
                                p { class: "text-xs text-zinc-400", "By {post.author.name} | {post.created_at}" }
                            }
                        })}
                    }
                },
                Some(_) => rsx! {
                    p { "No posts found." }
                },
                None if frame.is_loading() => rsx! {
                    p { "Loading..." }
                },
                None if frame.is_failed() => rsx! {
                    p { "Failed to load posts." }
                },
                _ => rsx! {
                    p { "No data." }
                },
            }
        }
    }
}
