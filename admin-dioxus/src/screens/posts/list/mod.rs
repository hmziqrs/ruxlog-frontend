mod components;
mod context;
mod utils;

use components::{ActiveFilters, BulkActionsBar, GridView, TableView, Toolbar};
use context::{PostListContext, ViewMode};

use dioxus::prelude::*;

use crate::hooks::{use_list_screen_with_handlers, ListScreenConfig};
use crate::router::Route;
use crate::store::{use_categories, use_post, use_tag, use_user, ListStore};
use crate::types::Order;
use crate::ui::shadcn::Button;

#[component]
pub fn PostsListScreen() -> Element {
    let nav = use_navigator();
    let posts_state = use_post();
    let categories_state = use_categories();
    let tags_state = use_tag();
    let users_state = use_user();

    // Initialize context
    let ctx = PostListContext::new();
    use_context_provider(|| ctx.clone());

    let (list_state, handlers) = use_list_screen_with_handlers(
        Some(ListScreenConfig {
            default_sort_field: "created_at".to_string(),
            default_sort_order: Order::Desc,
        }),
        ctx.filters,
    );

    // Load filter data on mount
    use_effect(move || {
        spawn(async move {
            categories_state.list().await;
            tags_state.list().await;
            users_state.list().await;
        });
    });

    // Effect to load posts when filters change
    use_effect({
        let list_state = list_state;
        let selected_ids = ctx.selected_ids;
        move || {
            let q = ctx.filters.read().clone();
            let _tick = list_state.reload_tick();
            let posts_state = posts_state;
            selected_ids.write().clear();
            spawn(async move {
                posts_state.fetch_list_with_query(q).await;
            });
        }
    });

    let list = posts_state.list.read();
    let list_loading = list.is_loading();

    let (posts, current_page) = if let Some(p) = &list.data {
        (p.data.clone(), p.page)
    } else {
        (Vec::new(), 1)
    };

    let has_data = !posts.is_empty();
    let active_filter_count = ctx.active_filter_count();

    rsx! {
        div { class: "flex flex-col gap-6 w-full",
            // Header
            header { class: "flex flex-col gap-4 md:flex-row md:items-center md:justify-between",
                div {
                    h1 { class: "text-3xl font-bold tracking-tight", "Posts" }
                    p { class: "text-zinc-500 dark:text-zinc-400 mt-1",
                        "Manage and view your blog posts. Create, edit, and organize content."
                    }
                }
                Button {
                    onclick: move |_| { nav.push(Route::PostsAddScreen {}); },
                    "Create Post"
                }
            }

            // Toolbar with search, filters, and view switcher
            Toolbar {
                list_loading,
                search_input: list_state.search_input(),
                on_search: handlers.handle_search
            }

            // Active filter badges
            ActiveFilters { active_filter_count }

            // Bulk actions bar
            BulkActionsBar {}

            // Content - switch between table and grid views
            if *ctx.view_mode.read() == ViewMode::Table {
                TableView {
                    posts: posts.clone(),
                    list_loading,
                    has_data,
                    current_sort_field: list_state.sort_field(),
                    on_sort: handlers.handle_sort,
                    on_clear: handlers.handle_clear
                }
            } else {
                GridView {
                    posts: posts.clone(),
                    list_loading,
                    has_data
                }
            }

            // Pagination
            if let Some(paginated) = &list.data {
                div { class: "flex items-center justify-between border-t border-zinc-200 dark:border-zinc-800 pt-4",
                    div { class: "text-sm text-zinc-500 dark:text-zinc-400",
                        "Page {paginated.page} of {paginated.total / paginated.per_page + 1} ({paginated.total} total)"
                    }
                    div { class: "flex items-center gap-2",
                        Button {
                            variant: crate::ui::shadcn::ButtonVariant::Outline,
                            size: crate::ui::shadcn::ButtonSize::Sm,
                            disabled: paginated.page <= 1,
                            onclick: move |_| { handlers.handle_prev.call(current_page); },
                            "Previous"
                        }
                        Button {
                            variant: crate::ui::shadcn::ButtonVariant::Outline,
                            size: crate::ui::shadcn::ButtonSize::Sm,
                            disabled: paginated.page >= (paginated.total / paginated.per_page + 1),
                            onclick: move |_| { handlers.handle_next.call(current_page); },
                            "Next"
                        }
                    }
                }
            }
        }
    }
}
