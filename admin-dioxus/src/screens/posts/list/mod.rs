use crate::router::Route;
use crate::store::{use_post, Post};
use crate::ui::shadcn::{
    Avatar, AvatarFallback, AvatarImage, Badge, BadgeVariant, Button, ButtonSize, ButtonVariant,
    Card, CardContent, CardFooter, CardHeader, Checkbox, DropdownMenu, DropdownMenuContent,
    DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger, Popover, PopoverContent,
    PopoverTrigger,
};

use dioxus::logger::tracing;
use dioxus::prelude::*;
use hmziq_dioxus_free_icons::icons::ld_icons::{
    LdCalendar, LdEllipsis, LdEye, LdGrid3x3, LdHeart, LdLayoutList, LdMessageSquare, LdSearch,
    LdTag,
};
use hmziq_dioxus_free_icons::Icon;

// Helper function for avatar fallback
fn generate_avatar_fallback(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect::<String>()
}

// Helper function for date formatting
fn format_short_date(date: &chrono::DateTime<chrono::Utc>) -> String {
    date.format("%y-%b-%d").to_string()
}

// Layout type enum removed as we render Grid-only (non-interactive) like the JS Body example

#[component]
pub fn PostsListScreen() -> Element {
    let posts_state = use_post();

    // Fetch posts on mount
    use_effect(move || {
        tracing::info!("PostsListScreen: Fetching posts on mount");
        spawn(async move {
            tracing::info!("PostsListScreen: Starting async fetch");
            posts_state.list().await;
            tracing::info!("PostsListScreen: Fetch completed");
        });
    });

    rsx! {
        div { class: "min-h-screen bg-zinc-50 dark:bg-zinc-950 text-zinc-900 dark:text-zinc-50",
            div { class: "container mx-auto py-8 px-4",
                div { class: "flex flex-col gap-6",
                    Header {}
                    Filters {}
                    ActiveFilters {}
                    Body {}
                }
            }
        }
    }
}

// Header Component
#[component]
fn Header() -> Element {
    let nav = use_navigator();
    rsx! {
        header { class: "flex flex-col gap-4 md:flex-row md:items-center md:justify-between",
            div {
                h1 { class: "text-3xl font-bold tracking-tight", "Posts" }
                p { class: "text-zinc-500 dark:text-zinc-400 mt-1", "Manage and view your blog posts" }
            }
            div { class: "flex items-center gap-2",
                Button {
                    onclick: move |_| {nav.push(Route::PostsAddScreen {});},
                    "Create Post"
                }
            }
        }
    }
}

// Filters Component (static UI, no interactivity)
#[component]
fn Filters() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 md:flex-row md:items-center md:justify-between",
            // Search
            div { class: "relative w-full md:w-96",
                div { class: "absolute left-2.5 top-2.5 h-4 w-4 text-zinc-500 dark:text-zinc-400",
                    div { class: "w-4 h-4", Icon { icon: LdSearch {} } }
                }
                input {
                    class: "w-full pl-8 bg-white dark:bg-zinc-900 rounded border border-zinc-200 dark:border-zinc-800 px-3 h-10 text-sm",
                    r#type: "search",
                    placeholder: "Search posts...",
                    value: "",
                    readonly: true,
                }
            }

            // Sort, Filter, View buttons
            div { class: "flex items-center gap-2",
                // Sort Dropdown
                DropdownMenu {
                    DropdownMenuTrigger {
                        Button { variant: ButtonVariant::Outline, class: "h-9 gap-1",
                            // No sort icon available; using text only
                            "Sort"
                        }
                    }
                    DropdownMenuContent { class: "w-48",
                        DropdownMenuItem { "Title" }
                        DropdownMenuItem { "Publish Date" }
                        DropdownMenuItem { "Views" }
                        DropdownMenuItem { "Likes" }
                        DropdownMenuSeparator {}
                        DropdownMenuItem { "Descending" }
                    }
                }

                // Filter Popover (exclude Authors section as requested)
                Popover {
                    PopoverTrigger {
                        Button { variant: ButtonVariant::Outline, class: "h-9 gap-1",
                            // No filter icon available; using text only
                            "Filter"
                        }
                    }
                    PopoverContent { class: "w-72 p-4",
                        div { class: "space-y-4",
                            div { class: "flex items-center justify-between",
                                h4 { class: "font-medium", "Filters" }
                                Button { variant: ButtonVariant::Ghost, class: "h-auto p-0 text-xs", "Clear all" }
                            }

                            // Status Filter
                            div { class: "space-y-2",
                                h5 { class: "text-sm font-medium", "Status" }
                                div { class: "space-y-1",
                                    div { class: "flex items-center gap-2",
                                        Checkbox {}
                                        span { class: "text-sm", "All" }
                                    }
                                    div { class: "flex items-center gap-2",
                                        Checkbox {}
                                        span { class: "text-sm", "Published" }
                                    }
                                    div { class: "flex items-center gap-2",
                                        Checkbox {}
                                        span { class: "text-sm", "Draft" }
                                    }
                                }
                            }

                            div { class: "-mx-4 my-2 h-px bg-border" }

                            // Category Filter
                            div { class: "space-y-2",
                                h5 { class: "text-sm font-medium", "Categories" }
                                div { class: "max-h-32 overflow-y-auto space-y-1 pr-2",
                                    div { class: "flex items-center gap-2",
                                        Checkbox {}
                                        span { class: "text-sm", "Technology" }
                                    }
                                    div { class: "flex items-center gap-2",
                                        Checkbox {}
                                        span { class: "text-sm", "Design" }
                                    }
                                    div { class: "flex items-center gap-2",
                                        Checkbox {}
                                        span { class: "text-sm", "Business" }
                                    }
                                }
                            }

                            div { class: "-mx-4 my-2 h-px bg-border" }

                            // Tags Filter
                            div { class: "space-y-2",
                                h5 { class: "text-sm font-medium", "Tags" }
                                div { class: "max-h-32 overflow-y-auto space-y-1 pr-2",
                                    div { class: "flex items-center gap-2",
                                        Checkbox {}
                                        span { class: "text-sm", "React" }
                                    }
                                    div { class: "flex items-center gap-2",
                                        Checkbox {}
                                        span { class: "text-sm", "JavaScript" }
                                    }
                                    div { class: "flex items-center gap-2",
                                        Checkbox {}
                                        span { class: "text-sm", "Tailwind" }
                                    }
                                }
                            }
                        }
                    }
                }

                // View Mode Buttons (static)
                Button { class: "h-9 w-9", variant: ButtonVariant::Default,
                    div { class: "h-4 w-4", Icon { icon: LdGrid3x3 {} } }
                    span { class: "sr-only", "Grid view" }
                }
                Button { class: "h-9 w-9", variant: ButtonVariant::Outline,
                    div { class: "h-4 w-4", Icon { icon: LdLayoutList {} } }
                    span { class: "sr-only", "List view" }
                }
            }
        }
    }
}

// ActiveFilters Component (static badges)
#[component]
fn ActiveFilters() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2 text-sm text-zinc-500 dark:text-zinc-400",
            span { "12 posts" }
            Badge { variant: BadgeVariant::Outline, "Published" }
            Badge { variant: BadgeVariant::Outline, class: "gap-1",
                // No user icon available; showing just text
                "John Doe"
            }
            Badge { variant: BadgeVariant::Outline, class: "gap-1",
                div { class: "h-3.5 w-3.5", Icon { icon: LdTag {} } }
                "React"
            }
            Button { variant: ButtonVariant::Ghost, class: "h-7 px-2 text-xs", size: ButtonSize::Sm,
                "Clear all"
            }
        }
    }
}

// Body Component (renders grid of posts; static layout)
#[component]
fn Body() -> Element {
    let posts_state = use_post();

    rsx! {
        {
            let posts_list = posts_state.list.read();
            tracing::info!("PostsListScreen Body: status={:?}, has_data={}", posts_list.status, posts_list.data.is_some());

            if posts_list.is_init() || posts_list.is_loading() {
                rsx! {
                    div { class: "flex items-center justify-center py-12",
                        div { class: "text-center",
                            div { class: "loading loading-spinner loading-lg" }
                            p { class: "mt-4", "Loading posts..." }
                        }
                    }
                }
            } else if posts_list.is_failed() {
                rsx! {
                    div { class: "flex items-center justify-center py-12",
                        div { class: "alert alert-error max-w-md",
                            span { {posts_list.message.clone().unwrap_or_else(|| "Failed to load posts".to_string())} }
                        }
                    }
                }
            } else {
                match &posts_list.data {
                    Some(posts) => {
                        if posts.is_empty() {
                            rsx! {
                                div { class: "flex flex-col items-center justify-center py-12 text-center",
                                    div { class: "h-12 w-12 text-zinc-300 dark:text-zinc-700 mb-4",
                                        Icon { icon: LdMessageSquare {} }
                                    }
                                    h3 { class: "text-lg font-medium", "No posts found" }
                                    p { class: "text-zinc-500 dark:text-zinc-400 mt-1 max-w-md",
                                        "No posts match your current filters. Try adjusting your search or filter criteria."
                                    }
                                    Button { variant: ButtonVariant::Outline, class: "mt-4", "Clear all filters" }
                                }
                            }
                        } else {
                            rsx! {
                                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                                    {posts.iter().map(|post| rsx! { PostGridCard { post: post.clone() } })}
                                }
                            }
                        }
                    }
                    None => rsx! {
                        // Fallback empty state when no data
                        div { class: "flex flex-col items-center justify-center py-12 text-center",
                            div { class: "h-12 w-12 text-zinc-300 dark:text-zinc-700 mb-4",
                                Icon { icon: LdMessageSquare {} }
                            }
                            h3 { class: "text-lg font-medium", "No posts found" }
                            p { class: "text-zinc-500 dark:text-zinc-400 mt-1 max-w-md",
                                "No posts match your current filters. Try adjusting your search or filter criteria."
                            }
                            Button { variant: ButtonVariant::Outline, class: "mt-4", "Clear all filters" }
                        }
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
            if let Some(featured_image) = &post.featured_image {
                div { class: " aspect-video w-full overflow-hidden",
                    img {
                        src: "{featured_image.file_url}",
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
                                Badge {
                                    variant: BadgeVariant::Outline,
                                    class: "bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300 hover:bg-zinc-100 dark:hover:bg-zinc-800",
                                    "{post.category.name}"
                                }
                            span { class: if post.is_published() { "px-2 py-0.5 text-xs rounded-full bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400" } else { "px-2 py-0.5 text-xs rounded-full bg-amber-100 text-amber-800 dark:bg-amber-900/30 dark:text-amber-400" },
                                "POST_STATUS"
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
                            DropdownMenuItem {
                                onclick: move |_| {
                                    let nav = navigator();
                                    nav.push(Route::PostsEditScreen { id: post.id });
                                },
                                "Edit"
                            }
                            DropdownMenuItem { "Duplicate" }
                            DropdownMenuItem {
                                variant: String::from("destructive"),
                                class: "text-red-500 dark:text-red-400",
                                onclick: move |_| {
                                    let posts = use_post();
                                    spawn(async move {
                                        posts.remove(post.id).await;
                                        posts.list().await;
                                    });
                                },
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
                            src: post.author.avatar.as_ref().map(|a| a.file_url.clone()).unwrap_or_default(),
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
                            if post.is_published() && post.published_at.is_some() {
                                {format_short_date(post.published_at.as_ref().unwrap())}
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
                if let Some(featured_image) = &post.featured_image {
                    div { class: "md:w-48 lg:w-60 aspect-video md:aspect-square overflow-hidden",
                        img {
                            src: "{featured_image.file_url}",
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
                                    Badge { variant: BadgeVariant::Secondary, "{post.category.name}" }
                                span { class: if post.is_published() { "px-2 py-0.5 text-xs rounded-full bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400" } else { "px-2 py-0.5 text-xs rounded-full bg-amber-100 text-amber-800 dark:bg-amber-900/30 dark:text-amber-400" },
                                    "POST_STATIUS"
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
                                DropdownMenuItem {
                                    onclick: move |_| {
                                        let nav = navigator();
                                        nav.push(Route::PostsEditScreen { id: post.id });
                                    },
                                    "Edit"
                                }
                                DropdownMenuItem { "Duplicate" }
                                DropdownMenuItem {
                                    class: "text-red-500 dark:text-red-400",
                                    onclick: move |_| {
                                        let posts = use_post();
                                        spawn(async move {
                                            posts.remove(post.id).await;
                                            posts.list().await;
                                        });
                                    },
                                    "Delete"
                                }
                            }
                        }
                    }

                    if let Some(excerpt) = &post.excerpt {
                        p { class: "text-zinc-500 dark:text-zinc-400 text-sm mt-2 line-clamp-2",
                            "{excerpt}"
                        }
                    }
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
                    div { class: "flex flex-col sm:flex-row sm:items-center justify-between mt-4 pt-4 border-t",
                        div { class: "flex items-center gap-2",
                            Avatar { class: "w-8 h-8",
                                AvatarImage {
                                    src: post.author.avatar.as_ref().map(|a| a.file_url.clone()).unwrap_or_default(),
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
                                    if post.is_published() && post.published_at.is_some() {
                                        {format_short_date(post.published_at.as_ref().unwrap())}
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
