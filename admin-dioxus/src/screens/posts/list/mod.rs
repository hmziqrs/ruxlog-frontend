use dioxus::prelude::*;

use crate::components::{
    DataTableScreen, HeaderColumn, ListEmptyState, ListErrorBannerProps, ListToolbarProps,
    PageHeaderProps, SkeletonCellConfig, SkeletonTableRows, UICellType,
};
use crate::hooks::{use_list_screen_with_handlers, ListScreenConfig};
use crate::router::Route;
use crate::store::{use_post, ListQuery, ListStore, Post, PostListQuery, PostStatus};
use crate::types::Order;
use crate::ui::shadcn::{
    Avatar, AvatarFallback, AvatarImage, Badge, BadgeVariant, Button, ButtonVariant, Checkbox,
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::utils::dates::format_short_date_dt;

use hmziq_dioxus_free_icons::{
    icons::ld_icons::{LdEllipsis, LdEye, LdHeart, LdMessageSquare, LdTag},
    Icon,
};

// Helper function for avatar fallback
fn generate_avatar_fallback(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect::<String>()
}

#[component]
pub fn PostsListScreen() -> Element {
    let nav = use_navigator();
    let posts_state = use_post();

    let filters = use_signal(|| PostListQuery::new());
    // Local selection state for the current page
    let selected_ids = use_signal(|| Vec::<i32>::new());

    // Use the enhanced hook that creates handlers for us
    let (list_state, handlers) = use_list_screen_with_handlers(
        Some(ListScreenConfig {
            default_sort_field: "created_at".to_string(),
            default_sort_order: Order::Desc,
        }),
        filters,
    );

    // Effect to load data when filters change - using the trait method
    use_effect({
        let list_state = list_state;
        let mut selected_ids = selected_ids;
        move || {
            let q = filters();
            let _tick = list_state.reload_tick();
            let posts_state = posts_state;
            // Clear any selection on query changes (page, search, filters, sorts)
            selected_ids.set(Vec::new());
            spawn(async move {
                posts_state.fetch_list_with_query(q).await;
            });
        }
    });

    let list = posts_state.list.read();
    let list_loading = list.is_loading();
    let _list_failed = list.is_failed();

    let (posts, current_page) = if let Some(p) = &list.data {
        (p.data.clone(), p.page)
    } else {
        (Vec::<Post>::new(), 1)
    };

    let has_data = !posts.is_empty();

    // Define header columns (prepend a blank cell for the selection checkbox column)
    let headers = vec![
        HeaderColumn::new("", false, "w-12 py-2 px-3", None),
        HeaderColumn::new(
            "Title",
            true,
            "py-2 px-3 text-left font-medium text-xs md:text-sm whitespace-nowrap",
            Some("title"),
        ),
        HeaderColumn::new(
            "Author",
            true,
            "py-2 px-3 text-left font-medium text-xs md:text-sm whitespace-nowrap",
            Some("author_id"),
        ),
        HeaderColumn::new(
            "Category",
            true,
            "py-2 px-3 text-left font-medium text-xs md:text-sm whitespace-nowrap",
            Some("category_id"),
        ),
        HeaderColumn::new(
            "Status",
            false,
            "py-2 px-3 text-left font-medium text-xs md:text-sm whitespace-nowrap",
            None,
        ),
        HeaderColumn::new(
            "Stats",
            false,
            "py-2 px-3 text-left font-medium text-xs md:text-sm whitespace-nowrap",
            None,
        ),
        HeaderColumn::new(
            "Published",
            true,
            "py-2 px-3 text-left font-medium text-xs md:text-sm whitespace-nowrap",
            Some("published_at"),
        ),
        HeaderColumn::new(
            "Created",
            true,
            "py-2 px-3 text-left font-medium text-xs md:text-sm whitespace-nowrap",
            Some("created_at"),
        ),
        HeaderColumn::new(
            "Updated",
            true,
            "py-2 px-3 text-left font-medium text-xs md:text-sm whitespace-nowrap",
            Some("updated_at"),
        ),
        HeaderColumn::new("", false, "w-12 py-2 px-3", None),
    ];

    // Custom status filter handler for posts
    let handle_status_select = {
        let mut filters = filters;
        move |value: String| {
            let mut q = filters.peek().clone();
            q.set_page(1);
            q.status = match value.as_str() {
                "Published" | "published" => Some(PostStatus::Published),
                "Draft" | "draft" => Some(PostStatus::Draft),
                "Archived" | "archived" => Some(PostStatus::Archived),
                _ => None,
            };
            filters.set(q);
        }
    };

    rsx! {
        DataTableScreen::<Post> {
            frame: (posts_state.list)(),
            header: Some(PageHeaderProps {
                title: "Posts".to_string(),
                description: "Manage and view your blog posts. Create, edit, and organize content.".to_string(),
                actions: Some(rsx!{
                    Button {
                        onclick: move |_| { nav.push(Route::PostsAddScreen {}); },
                        "Create Post"
                    }
                }),
                class: None,
                embedded: false,
            }),
            headers: Some(headers),
            current_sort_field: Some(list_state.sort_field()),
            on_sort: Some(handlers.handle_sort.clone()),
            error_banner: Some(ListErrorBannerProps {
                message: "Failed to load posts. Please try again.".to_string(),
                retry_label: Some("Retry".to_string()),
                on_retry: Some(EventHandler::new(move |_| handlers.handle_retry.call(()))),
            }),
            toolbar: Some(ListToolbarProps {
                search_value: list_state.search_input(),
                search_placeholder: "Search posts by title, content, or author".to_string(),
                disabled: list_loading,
                on_search_input: handlers.handle_search.clone(),
                status_selected: match filters.read().status {
                    Some(PostStatus::Published) => "Published".to_string(),
                    Some(PostStatus::Draft) => "Draft".to_string(),
                    Some(PostStatus::Archived) => "Archived".to_string(),
                    None => "All".to_string(),
                },
                on_status_select: EventHandler::new(handle_status_select),
            }),
            on_prev: move |_| { handlers.handle_prev.call(current_page); },
            on_next: move |_| { handlers.handle_next.call(current_page); },
            // Render selection actions between toolbar and table (below_toolbar slot)
            below_toolbar: if !selected_ids.read().is_empty() {
                Some(rsx! {
                    div { class: "w-full flex items-center justify-between bg-transparent border border-zinc-200 dark:border-zinc-800 rounded-md px-4 py-3 shadow-sm",
                        span { class: "text-sm text-muted-foreground", "{selected_ids.read().len()} selected" }
                        div { class: "flex items-center gap-2",
                            Button { variant: ButtonVariant::Outline, class: "h-8",
                                onclick: {
                                    let mut selected_ids = selected_ids;
                                    move |_| {
                                        // TODO: Implement bulk publish action
                                        selected_ids.set(Vec::new());
                                    }
                                },
                                "Publish"
                            }
                            Button { variant: ButtonVariant::Outline, class: "h-8",
                                onclick: {
                                    let mut selected_ids = selected_ids;
                                    move |_| {
                                        // TODO: Implement bulk draft action
                                        selected_ids.set(Vec::new());
                                    }
                                },
                                "Set as Draft"
                            }
                            Button { variant: ButtonVariant::Outline, class: "h-8 text-red-600 border-red-200 dark:border-red-800 dark:hover:bg-red-950/20 hover:bg-red-50",
                                onclick: {
                                    let mut selected_ids = selected_ids;
                                    move |_| {
                                        // TODO: Implement bulk delete
                                        selected_ids.set(Vec::new());
                                    }
                                },
                                "Delete"
                            }
                        }
                    }
                })
            } else { None },
            // Table body content only - headers are now handled by DataTableScreen
            if posts.is_empty() {
                if list_loading && !has_data {
                    SkeletonTableRows {
                        row_count: 6,
                        cells: vec![
                            SkeletonCellConfig::custom(UICellType::Default, "w-12 py-2 px-3"),
                            SkeletonCellConfig::custom(UICellType::Default, "py-2 px-3"),
                            SkeletonCellConfig::custom(UICellType::Default, "py-2 px-3"),
                            SkeletonCellConfig::custom(UICellType::Default, "py-2 px-3"),
                            SkeletonCellConfig::custom(UICellType::Badge, "py-2 px-3"),
                            SkeletonCellConfig::custom(UICellType::Default, "py-2 px-3"),
                            SkeletonCellConfig::custom(UICellType::Default, "py-2 px-3"),
                            SkeletonCellConfig::custom(UICellType::Default, "py-2 px-3"),
                            SkeletonCellConfig::custom(UICellType::Default, "py-2 px-3"),
                            SkeletonCellConfig::custom(UICellType::Action, "w-12 py-2 px-3"),
                        ],
                    }
                } else {
                    td { colspan: "10", class: "py-12 px-4 text-center",
                        ListEmptyState {
                            title: "No posts found".to_string(),
                            description: "Try adjusting your search or create a new post to get started.".to_string(),
                            clear_label: "Clear search".to_string(),
                            create_label: "Create your first post".to_string(),
                            on_clear: move |_| { handlers.handle_clear.call(()); },
                            on_create: move |_| { nav.push(Route::PostsAddScreen {}); },
                        }
                    }
                }
            } else {
                {posts.iter().map(|post| {
                    let post_id = post.id;
                    let is_selected = selected_ids.read().contains(&post_id);

                    rsx! {
                        tr {
                            key: "{post_id}",
                            class: "border-b border-zinc-200 dark:border-zinc-800 hover:bg-zinc-50 dark:hover:bg-zinc-900/50 transition-colors",
                            // Selection checkbox
                            td { class: "w-12 py-2 px-3",
                                Checkbox {
                                    checked: is_selected,
                                    onchange: {
                                        let mut selected_ids = selected_ids;
                                        move |_| {
                                            let mut ids = selected_ids.peek().clone();
                                            if ids.contains(&post_id) {
                                                ids.retain(|id| *id != post_id);
                                            } else {
                                                ids.push(post_id);
                                            }
                                            selected_ids.set(ids);
                                        }
                                    },
                                }
                            }
                            // Title with featured image
                            td { class: "py-2 px-3",
                                div { class: "flex items-center gap-3",
                                    if let Some(featured_image) = &post.featured_image {
                                        div { class: "w-12 h-12 rounded overflow-hidden flex-shrink-0",
                                            img {
                                                src: "{featured_image.file_url}",
                                                alt: "{post.title}",
                                                class: "w-full h-full object-cover",
                                            }
                                        }
                                    } else {
                                        div { class: "w-12 h-12 rounded bg-zinc-100 dark:bg-zinc-800 flex items-center justify-center flex-shrink-0",
                                            div { class: "w-5 h-5 text-zinc-400 dark:text-zinc-600",
                                                Icon { icon: LdMessageSquare {} }
                                            }
                                        }
                                    }
                                    div { class: "min-w-0",
                                        div { class: "font-medium text-sm truncate max-w-xs", "{post.title}" }
                                        if let Some(excerpt) = &post.excerpt {
                                            p { class: "text-xs text-zinc-500 dark:text-zinc-400 truncate max-w-xs mt-0.5",
                                                "{excerpt}"
                                            }
                                        }
                                    }
                                }
                            }
                            // Author
                            td { class: "py-2 px-3",
                                div { class: "flex items-center gap-2",
                                    Avatar { class: "w-7 h-7",
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
                                    span { class: "text-xs font-medium text-zinc-700 dark:text-zinc-300 truncate max-w-[120px]",
                                        "{post.author.name}"
                                    }
                                }
                            }
                            // Category
                            td { class: "py-2 px-3",
                                Badge {
                                    variant: BadgeVariant::Outline,
                                    class: "bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-300",
                                    "{post.category.name}"
                                }
                            }
                            // Status
                            td { class: "py-2 px-3",
                                {match post.status {
                                    PostStatus::Published => rsx! {
                                        Badge {
                                            variant: BadgeVariant::Secondary,
                                            class: "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400",
                                            "Published"
                                        }
                                    },
                                    PostStatus::Draft => rsx! {
                                        Badge {
                                            variant: BadgeVariant::Secondary,
                                            class: "bg-amber-100 text-amber-800 dark:bg-amber-900/30 dark:text-amber-400",
                                            "Draft"
                                        }
                                    },
                                    PostStatus::Archived => rsx! {
                                        Badge {
                                            variant: BadgeVariant::Secondary,
                                            class: "bg-zinc-100 text-zinc-800 dark:bg-zinc-800/30 dark:text-zinc-400",
                                            "Archived"
                                        }
                                    },
                                }}
                            }
                            // Stats (views, likes, tags)
                            td { class: "py-2 px-3",
                                div { class: "flex items-center gap-3 text-xs text-zinc-500 dark:text-zinc-400",
                                    div { class: "flex items-center gap-1",
                                        div { class: "w-3.5 h-3.5",
                                            Icon { icon: LdEye {} }
                                        }
                                        span { "{post.view_count}" }
                                    }
                                    div { class: "flex items-center gap-1",
                                        div { class: "w-3.5 h-3.5",
                                            Icon { icon: LdHeart {} }
                                        }
                                        span { "{post.likes_count}" }
                                    }
                                    div { class: "flex items-center gap-1",
                                        div { class: "w-3.5 h-3.5",
                                            Icon { icon: LdTag {} }
                                        }
                                        span { "{post.tags.len()}" }
                                    }
                                }
                            }
                            // Published date
                            td { class: "py-2 px-3 text-xs text-zinc-500 dark:text-zinc-400 whitespace-nowrap",
                                {if let Some(published_at) = &post.published_at {
                                    format_short_date_dt(published_at)
                                } else {
                                    "—".to_string()
                                }}
                            }
                            // Created date
                            td { class: "py-2 px-3 text-xs text-zinc-500 dark:text-zinc-400 whitespace-nowrap",
                                {format_short_date_dt(&post.created_at)}
                            }
                            // Updated date
                            td { class: "py-2 px-3 text-xs text-zinc-500 dark:text-zinc-400 whitespace-nowrap",
                                {format_short_date_dt(&post.updated_at)}
                            }
                            // Actions dropdown
                            td { class: "w-12 py-2 px-3",
                                DropdownMenu {
                                    DropdownMenuTrigger { class: "h-8 w-8",
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            class: "h-8 w-8 p-0",
                                            div { class: "w-4 h-4",
                                                Icon { icon: LdEllipsis {} }
                                            }
                                        }
                                    }
                                    DropdownMenuContent { class: "w-40",
                                        DropdownMenuItem {
                                            onclick: {
                                                let nav = nav.clone();
                                                move |_| {
                                                    nav.push(Route::PostsEditScreen { id: post_id });
                                                }
                                            },
                                            "Edit"
                                        }
                                        DropdownMenuItem {
                                            onclick: move |_| {
                                                // TODO: Implement duplicate
                                            },
                                            "Duplicate"
                                        }
                                        DropdownMenuItem {
                                            class: "text-red-600 dark:text-red-400",
                                            onclick: {
                                                let posts_state = posts_state;
                                                move |_| {
                                                    let posts_state = posts_state;
                                                    spawn(async move {
                                                        posts_state.remove(post_id).await;
                                                        let remove_state = posts_state.remove.read();
                                                        if let Some(state) = remove_state.get(&post_id) {
                                                            if state.is_success() {
                                                                // Reload the list
                                                                posts_state.fetch_list_with_query(filters()).await;
                                                            }
                                                        }
                                                    });
                                                }
                                            },
                                            "Delete"
                                        }
                                    }
                                }
                            }
                        }
                    }
                })}
            }
        }
    }
}
