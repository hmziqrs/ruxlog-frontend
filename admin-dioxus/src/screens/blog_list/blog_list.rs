use chrono::NaiveDateTime;
use crate::router::Route;
use crate::store::{use_post, Post};
use crate::ui::shadcn::{Avatar, AvatarFallback, AvatarImage, Badge, BadgeVariant, Button, ButtonVariant, Card, CardContent, CardFooter, CardHeader, DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger};
use dioxus::prelude::*;
use hmziq_dioxus_free_icons::icons::ld_icons::{
    LdCalendar, LdEllipsis, LdEye, LdGrid3x3, LdHeart, LdLayoutList, LdMessageSquare, LdSearch, LdTag
};
use hmziq_dioxus_free_icons::Icon;

// Helper function for avatar fallback
fn generate_avatar_fallback(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect::<String>()
}

// Helper function for date formatting
fn format_short_date(date_str: &str) -> String {
    if let Ok(date) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S.%f") {
        date.format("%y-%b-%d").to_string()
    } else {
        date_str.to_string()
    }
}

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
    let nav = use_navigator();

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
                            Button {
                                onclick: move |_| {
                                    nav.push(Route::AddBlogScreen {});
                                },
                                "Create post"
                            }
                        }
                    }
                    // Search and view mode
                    div { class: "flex flex-col gap-4 md:flex-row md:items-center md:justify-between",
                        div { class: "relative w-full md:w-96",
                            div { class: "absolute left-2.5 top-2.5 size-4 text-zinc-500 dark:text-zinc-400",
                                div { class: "w-4 h-4",
                                    Icon { icon: LdSearch {} }
                                }
                            }
                            input {
                                class: "w-full pl-8 bg-white dark:bg-zinc-900 rounded border px-3 h-10 text-sm",
                                r#type: "search",
                                placeholder: "Search posts...",
                                value: "{search_query}",
                                oninput: move |e| search_query.set(e.value()),
                            }
                        }
                        div { class: "flex items-center gap-2",
                            button {
                                class: if *layout_type.read() == LayoutType::Grid { "size-10 bg-zinc-900 dark:bg-zinc-700 text-white rounded flex items-center justify-center" } else { "size-10 border rounded flex items-center justify-center" },
                                onclick: move |_| layout_type.set(LayoutType::Grid),
                                div { class: "size-4",
                                    Icon { icon: LdGrid3x3 {} }
                                }
                                span { class: "sr-only", "Grid view" }
                            }
                            button {
                                class: if *layout_type.read() == LayoutType::List { "size-10 bg-zinc-900 dark:bg-zinc-700 text-white rounded flex items-center justify-center" } else { "size-10 border rounded flex items-center justify-center" },
                                onclick: move |_| layout_type.set(LayoutType::List),
                                div { class: "size-4",
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
                            div { class: "flex flex-col gap-6",
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
        Card { class: "pt-0 pb-0 overflow-hidden transition-all hover:shadow-md dark:bg-zinc-900",
            // Featured image
            if let Some(featured_image) = &post.featured_image_url {
                div { class: " aspect-video w-full overflow-hidden",
                    img {
                        src: "{featured_image}",
                        alt: "{post.title}",
                        class: "h-full w-full object-cover transition-transform hover:scale-105",
                    }
                }
            } else {
                div { class: "aspect-video w-full bg-zinc-200 dark:bg-zinc-800 flex items-center justify-center",
                    div { class: "w-10 h-10 text-zinc-400 dark:text-zinc-600",
                        Icon { icon: LdMessageSquare {} }
                    }
                }
            }

            CardHeader { class: "",
                div { class: "flex items-start justify-between",
                    div { class: "space-y-1.5",
                        div { class: "flex items-center gap-2",
                            // Category badge
                            if let Some(category) = &post.category {
                                Badge {
                                    variant: BadgeVariant::Outline,
                                    class: "bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 hover:bg-zinc-100 dark:hover:bg-zinc-800",
                                    "{category.name}"
                                }
                            }
                            // Published/Draft status
                            span { class: if post.is_published { "px-2 py-0.5 text-xs rounded-full bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400" } else { "px-2 py-0.5 text-xs rounded-full bg-amber-100 text-amber-800 dark:bg-amber-900/30 dark:text-amber-400" },
                                if post.is_published {
                                    "Published"
                                } else {
                                    "Draft"
                                }
                            }
                        }
                        h3 { class: "font-semibold text-lg line-clamp-2", "{post.title}" }
                    }
                    DropdownMenu {
                        DropdownMenuTrigger { class: "h-8 w-8",
                            Button {
                                class: "h-8 w-8",
                                variant: ButtonVariant::Ghost,
                                div { class: "w-4 h-4",
                                    Icon { icon: LdEllipsis {} }
                                }
                            }
                        }
                        DropdownMenuContent {
                            DropdownMenuItem { "Edit" }
                            DropdownMenuItem { "Duplicate" }
                            DropdownMenuItem {
                                variant: String::from("destructive"),
                                class: "text-red-500 dark:text-red-400",
                                "Delete"
                            }
                        }
                    }
                }
            }
            CardContent { class: "p-4 pt-0 pb-0",
                if let Some(excerpt) = &post.excerpt {
                    p { class: "text-zinc-500 dark:text-zinc-400 text-sm line-clamp-2 mt-1",
                        "{excerpt}"
                    }
                }
                // Tags
                if !post.tags.is_empty() {
                    div { class: "flex flex-wrap gap-1.5 mt-3",
                        {post.tags.iter().take(3).map(|tag| rsx! {
                            Badge { variant: BadgeVariant::Secondary, "{tag.name}" }
                        })}
                        if post.tags.len() > 3 {
                            Badge { variant: BadgeVariant::Secondary, "+{post.tags.len() - 3}" }
                        }
                    }
                }
            }
            CardFooter { class: "p-4 pt-0 flex items-center justify-between",
                div { class: "flex items-center gap-2",
                    Avatar { class: "w-8 h-8",
                        AvatarImage {
                            src: post.author.avatar.clone().unwrap_or_default(),
                            alt: post.author.name.clone(),
                        }
                        AvatarFallback {
                            span { class: "text-xs font-medium",
                                {generate_avatar_fallback(&post.author.name)}
                            }
                        }
                    }
                    span { class: "text-xs text-zinc-500 dark:text-zinc-400", "{post.author.name}" }
                }
                // Stats with date between views and likes
                div { class: "flex items-center gap-3 text-zinc-500 dark:text-zinc-400",
                    div { class: "flex items-center gap-1 text-xs",
                        div { class: "w-3.5 h-3.5",
                            Icon { icon: LdCalendar {} }
                        }
                        span {
                            if post.is_published && post.published_at.is_some() {
                                {format_short_date(&post.published_at.clone().unwrap_or_default())}
                            } else {
                                {format_short_date(&post.created_at)}
                            }
                        }
                    }
                    div { class: "flex items-center gap-1 text-xs",
                        div { class: "w-3.5 h-3.5",
                            Icon { icon: LdEye {} }
                        }
                        span { "{post.view_count}" }
                    }
                    div { class: "flex items-center gap-1 text-xs",
                        div { class: "w-3.5 h-3.5",
                            Icon { icon: LdHeart {} }
                        }
                        span { "{post.likes_count}" }
                    }
                }
            }
        }
    }
}

#[component]
fn PostListItem(post: Post) -> Element {
    rsx! {
        Card { class: "pt-0 pb-0 overflow-hidden transition-all hover:shadow-md dark:bg-zinc-900",
            div { class: "flex flex-col md:flex-row",
                // Featured image
                if let Some(featured_image) = &post.featured_image_url {
                    div { class: "md:w-48 lg:w-60 aspect-video md:aspect-square overflow-hidden",
                        img {
                            src: "{featured_image}",
                            alt: "{post.title}",
                            class: "h-full w-full object-cover",
                        }
                    }
                } else {
                    div { class: "md:w-48 lg:w-60 aspect-video md:aspect-square bg-zinc-200 dark:bg-zinc-800 flex items-center justify-center",
                        div { class: "w-10 h-10 text-zinc-400 dark:text-zinc-600",
                            Icon { icon: LdMessageSquare {} }
                        }
                    }
                }

                div { class: "flex-1 p-4",
                    div { class: "flex items-start justify-between",
                        div { class: "space-y-1",
                            div { class: "flex items-center gap-2",
                                // Category badge
                                if let Some(category) = &post.category {
                                    Badge { variant: BadgeVariant::Secondary, "{category.name}" }
                                }
                                // Published status
                                span { class: if post.is_published { "px-2 py-0.5 text-xs rounded-full bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400" } else { "px-2 py-0.5 text-xs rounded-full bg-amber-100 text-amber-800 dark:bg-amber-900/30 dark:text-amber-400" },
                                    if post.is_published {
                                        "Published"
                                    } else {
                                        "Draft"
                                    }
                                }
                            }
                            h3 { class: "font-semibold text-lg", "{post.title}" }
                        }
                        DropdownMenu {
                            DropdownMenuTrigger { class: "h-8 w-8",
                                button { class: "h-8 w-8 flex items-center justify-center",
                                    div { class: "w-4 h-4",
                                        Icon { icon: LdEllipsis {} }
                                    }
                                }
                            }
                            DropdownMenuContent {
                                DropdownMenuItem { "Edit" }
                                DropdownMenuItem { "Duplicate" }
                                DropdownMenuItem { class: "text-red-500 dark:text-red-400", "Delete" }
                            }
                        }
                    }

                    if let Some(excerpt) = &post.excerpt {
                        p { class: "text-zinc-500 dark:text-zinc-400 text-sm mt-2 line-clamp-2",
                            "{excerpt}"
                        }
                    }
                    // Tags
                    if !post.tags.is_empty() {
                        div { class: "flex flex-wrap gap-1.5 mt-3",
                            {post.tags.iter().take(5).map(|tag| rsx! {
                                Badge { variant: BadgeVariant::Secondary, "{tag.name}" }
                            })}
                            if post.tags.len() > 5 {
                                Badge { variant: BadgeVariant::Secondary, "+{post.tags.len() - 5}" }
                            }
                        }
                    }
                    // Author and stats
                    div { class: "flex flex-col sm:flex-row sm:items-center justify-between mt-4 pt-4 border-t",
                        div { class: "flex items-center gap-2",
                            Avatar { class: "w-8 h-8",
                                AvatarImage {
                                    src: post.author.avatar.clone().unwrap_or_default(),
                                    alt: post.author.name.clone(),
                                }
                                AvatarFallback {
                                    span { class: "text-xs font-medium",
                                        {generate_avatar_fallback(&post.author.name)}
                                    }
                                }
                            }
                            span { class: "text-xs font-medium text-zinc-500 dark:text-zinc-400",
                                "{post.author.name}"
                            }
                        }
                        // Stats
                        div { class: "flex items-center gap-4 mt-2 sm:mt-0 text-zinc-500 dark:text-zinc-400",
                            div { class: "flex items-center gap-1 text-xs",
                                div { class: "w-3.5 h-3.5",
                                    Icon { icon: LdEye {} }
                                }
                                span { "{post.view_count}" }
                            }
                            div { class: "flex items-center gap-1 text-xs",
                                div { class: "w-3.5 h-3.5",
                                    Icon { icon: LdCalendar {} }
                                }
                                span {
                                    if post.is_published && post.published_at.is_some() {
                                        {format_short_date(&post.published_at.clone().unwrap_or_default())}
                                    } else {
                                        {format_short_date(&post.created_at)}
                                    }
                                }
                            }
                            div { class: "flex items-center gap-1 text-xs",
                                div { class: "w-3.5 h-3.5",
                                    Icon { icon: LdHeart {} }
                                }
                                span { "{post.likes_count}" }
                            }
                            div { class: "flex items-center gap-1 text-xs",
                                div { class: "w-3.5 h-3.5",
                                    Icon { icon: LdTag {} }
                                }
                                span { "{post.tags.len()}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
