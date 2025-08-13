use dioxus::prelude::*;

use crate::hooks::use_previous;
use crate::router::Route;
use crate::store::{use_tag, Tag, TagsListQuery};
use crate::components::{ListEmptyState, ListErrorBanner, ListToolbar, LoadingOverlay, PageHeader, Pagination, ToastManager, ToastType};
use crate::ui::shadcn::{
    Badge, BadgeVariant, Button, ButtonVariant, Card, DropdownMenu, DropdownMenuContent,
    DropdownMenuItem, DropdownMenuTrigger,
};
use crate::utils::dates::format_short_date_dt;
use hmziq_dioxus_free_icons::{
    icons::ld_icons::{LdEllipsis, LdTag},
    Icon,
};

use std::time::Duration;
use gloo_timers::future::sleep;

#[component]
pub fn TagsListScreen() -> Element {
    let nav = use_navigator();
    let tags_state = use_tag();
    let mut toast: Signal<ToastManager> = use_context();
    let mut search_query = use_signal(|| String::new());
    let mut status_filter = use_signal(|| "all".to_string()); // all | active | inactive
    let mut page = use_signal(|| 1u64);
    let mut sort_order = use_signal(|| "desc".to_string()); // asc | desc


    // Fetch tags on mount
    use_effect(move || {
        spawn(async move {
            let q = TagsListQuery {
                page: Some(page.read().clone()),
                search: if search_query.read().is_empty() { None } else { Some(search_query.read().clone()) },
                sort_order: Some(sort_order.read().clone()),
            };
            tags_state.list_with_query(q).await;
        });
    });

    let list = tags_state.list.read();
    let list_loading = list.is_loading();
    let list_failed = list.is_failed();

    let prev_loading = use_previous(list_loading);
    
    use_effect(use_reactive!(|(list_loading,)| {
        if prev_loading != Some(list_loading) {
            toast
                .write()
                .add_toast("Hello".to_string(), "".to_string(), ToastType::Info, None);
        }
    }));

    // Snapshot data for rendering
    let (tags, total_items, current_page, _per_page) = if let Some(p) = list.data.clone() {
        (p.data.clone(), p.total, p.page, p.per_page)
    } else {
        (Vec::<Tag>::new(), 0, 1, 10)
    };
    let has_data = !tags.is_empty();

    let total = tags.len();
    let active = tags.iter().filter(|t| t.is_active).count();
    let inactive = total.saturating_sub(active);

    // Client-side filter by status to match the reference behavior
    let status_val = status_filter.read().clone();
    let filtered_tags: Vec<Tag> = tags
        .iter()
        .cloned()
        .filter(|t| match status_val.as_str() {
            "active" => t.is_active,
            "inactive" => !t.is_active,
            _ => true,
        })
        .collect();

    rsx! {
        // Page wrapper
        div { class: "min-h-screen bg-zinc-50 dark:bg-zinc-950 text-zinc-900 dark:text-zinc-50",
            // Unified autonomous header
            PageHeader {
                title: "Tags".to_string(),
                description: "Organize your content with tags. Create, edit, and manage tags.".to_string(),
                actions: Some(rsx!{ Button { onclick: move |_| { nav.push(Route::TagsAddScreen {}); }, "New Tag" } })
            }

            // Optional error banner
            if list_failed {
                div { class: "container mx-auto px-4 pt-4",
                    ListErrorBanner {
                        message: "Failed to load tags. Please try again.".to_string(),
                        retry_label: Some("Retry".to_string()),
                        on_retry: Some(EventHandler::new(move |_| {
                            let q = TagsListQuery {
                                page: Some(page.read().clone()),
                                search: if search_query.read().is_empty() { None } else { Some(search_query.read().clone()) },
                                sort_order: Some(sort_order.read().clone()),
                            };
                            spawn(async move { tags_state.list_with_query(q).await; });
                        })),
                    }
                }
            }

            // Stats
            div { class: "container mx-auto px-4 py-6 md:py-8",
                div { class: "grid grid-cols-1 gap-2 sm:grid-cols-3",
                    Card { class: "border-muted shadow-none",
                        div { class: "flex items-center justify-between p-4",
                            div { class: "space-y-1",
                                p { class: "text-xs font-medium uppercase tracking-wide text-zinc-500 dark:text-zinc-400", "Total" }
                                if list_loading && !has_data { 
                                    div { class: "h-6 w-16 rounded bg-muted animate-pulse" }
                                } else { 
                                    p { class: "text-2xl font-semibold tabular-nums", "{total}" }
                                }
                            }
                            div { class: "w-5 h-5 text-zinc-500 dark:text-zinc-400", Icon { icon: LdTag {} } }
                        }
                    }
                    Card { class: "border-muted shadow-none",
                        div { class: "flex items-center justify-between p-4",
                            div { class: "space-y-1",
                                p { class: "text-xs font-medium uppercase tracking-wide text-zinc-500 dark:text-zinc-400", "Active" }
                                if list_loading && !has_data { 
                                    div { class: "h-6 w-16 rounded bg-muted animate-pulse" }
                                } else { 
                                    p { class: "text-2xl font-semibold tabular-nums", "{active}" }
                                }
                            }
                            div { class: "h-5 w-5 rounded-full bg-green-500/15 ring-4 ring-green-500/10" }
                        }
                    }
                    Card { class: "border-muted shadow-none",
                        div { class: "flex items-center justify-between p-4",
                            div { class: "space-y-1",
                                p { class: "text-xs font-medium uppercase tracking-wide text-zinc-500 dark:text-zinc-400", "Inactive" }
                                if list_loading && !has_data { 
                                    div { class: "h-6 w-16 rounded bg-muted animate-pulse" }
                                } else { 
                                    p { class: "text-2xl font-semibold tabular-nums", "{inactive}" }
                                }
                            }
                            div { class: "h-5 w-5 rounded-full bg-zinc-500/15 ring-4 ring-zinc-500/10" }
                        }
                    }
                }
            }

            // Content
            div { class: "container mx-auto px-4 py-8 md:py-12",
                // Toolbar
                ListToolbar {
                    search_value: search_query.read().clone(),
                    search_placeholder: "Search tags by name, description, or slug".to_string(),
                    disabled: list_loading,
                    on_search_input: move |val: String| {
                        // Update UI state immediately, but debounce the fetch by 500ms.
                        search_query.set(val.clone());
                        page.set(1);
                        let search_query = search_query.clone();
                        let sort_order = sort_order.clone();
                        let tags_state = tags_state;
                        spawn(async move {
                            sleep(Duration::from_millis(500)).await;
                            // Only fetch if the input hasn't changed during the debounce window
                            if search_query.read().as_str() == val.as_str() {
                                let q = TagsListQuery {
                                    page: Some(1),
                                    search: if val.is_empty() { None } else { Some(val) },
                                    sort_order: Some(sort_order.read().clone()),
                                };
                                tags_state.list_with_query(q).await;
                            }
                        });
                    },
                    status_selected: status_filter.read().clone(),
                    on_status_select: move |value| { status_filter.set(value); },
                }

                // Table
                Card { class: "border-muted shadow-none overflow-hidden mt-4",
                    div { class: "relative",
                        div { class: "overflow-x-auto",
                            table { class: "w-full border-collapse",
                                thead { class: "sticky top-0 z-[1] bg-muted/60 backdrop-blur supports-[backdrop-filter]:bg-muted/40",
                                    tr { class: "border-b border-border/60",
                                        th { class: "py-3.5 px-4 text-left text-xs font-medium uppercase tracking-wider text-muted-foreground", "Name" }
                                        th { class: "hidden py-3.5 px-4 text-left text-xs font-medium uppercase tracking-wider text-muted-foreground md:table-cell", "Description" }
                                        th { class: "hidden py-3.5 px-4 text-left text-xs font-medium uppercase tracking-wider text-muted-foreground md:table-cell", "Slug" }
                                        th { class: "hidden py-3.5 px-4 text-left text-xs font-medium uppercase tracking-wider text-muted-foreground md:table-cell", "Created" }
                                        th { class: "py-3.5 px-4 text-left text-xs font-medium uppercase tracking-wider text-muted-foreground", "Status" }
                                        th { class: "py-3.5 px-4 text-right text-xs font-medium uppercase tracking-wider text-muted-foreground", "Actions" }
                                    }
                                }
                                tbody {
                                    if filtered_tags.is_empty() {
                                        if list_loading && !has_data {
                                            { (0..6).map(|_| rsx!{
                                                tr { class: "border-b border-border/60",
                                                    td { colspan: "6", class: "py-3 px-4",
                                                        div { class: "flex items-center gap-3",
                                                            div { class: "h-3.5 w-3.5 rounded-full bg-muted animate-pulse" }
                                                            div { class: "flex-1 space-y-2",
                                                                div { class: "h-4 w-1/3 rounded bg-muted animate-pulse" }
                                                                div { class: "h-3 w-2/3 rounded bg-muted animate-pulse" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }) }
                                        } else {
                                            tr { class: "border-b border-border/60",
                                                td { colspan: "6", class: "py-16 px-4",
                                                    ListEmptyState {
                                                        title: "No tags found".to_string(),
                                                        description: "Try adjusting your search or create a new tag to get started.".to_string(),
                                                        clear_label: "Clear search".to_string(),
                                                        create_label: "Create your first tag".to_string(),
                                                        on_clear: move |_| {
                                                            search_query.set(String::new());
                                                            page.set(1);
                                                            let q = TagsListQuery { page: Some(1), search: None, sort_order: Some(sort_order.read().clone()) };
                                                            spawn(async move { tags_state.list_with_query(q).await; });
                                                        },
                                                        on_create: move |_| { nav.push(Route::TagsAddScreen {}); },
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {filtered_tags.iter().map(|tag| {
                                            let tag_id = tag.id;
                                            rsx! {
                                            tr { class: "border-b border-border/60 hover:bg-muted/40 transition-colors",
                                                td { class: "py-3 px-4",
                                                    div { class: "flex items-center gap-3",
                                                        div { class: "h-3.5 w-3.5 shrink-0 rounded-full ring-2 ring-black/5 dark:ring-white/10", style: format!("background-color: {}", if tag.color.is_empty() { "#94a3b8" } else { &tag.color }) }
                                                        div { class: "min-w-0",
                                                            div { class: "font-medium leading-none", "{tag.name}" }
                                                            div { class: "mt-1 text-xs text-muted-foreground md:hidden", span { class: "truncate", "{tag.slug}" } }
                                                        }
                                                    }
                                                }
                                                td { class: "hidden max-w-[28rem] py-3 px-4 text-muted-foreground md:table-cell", span { class: "line-clamp-1", {tag.description.clone().unwrap_or("—".to_string())} } }
                                                td { class: "hidden py-3 px-4 text-muted-foreground md:table-cell", "{tag.slug}" }
                                                td { class: "hidden py-3 px-4 text-muted-foreground md:table-cell", "{format_short_date_dt(&tag.created_at)}" }
                                                td { class: "py-3 px-4",
                                                    if tag.is_active {
                                                        Badge { class: "bg-green-100 text-green-800 hover:bg-green-100 dark:bg-green-900/30 dark:text-green-300 dark:hover:bg-green-900/30", "Active" }
                                                    } else {
                                                        Badge { variant: BadgeVariant::Secondary, class: "bg-muted text-foreground/70 hover:bg-muted", "Inactive" }
                                                    }
                                                }
                                                td { class: "py-3 px-4",
                                                    div { class: "flex items-center justify-end gap-1.5",
                                                        DropdownMenu {
                                                            DropdownMenuTrigger {
                                                                Button { variant: ButtonVariant::Ghost, class: "h-8 w-8", div { class: "w-4 h-4", Icon { icon: LdEllipsis {} } } }
                                                            }
                                                            DropdownMenuContent { class: "w-44 border-border bg-popover",
                                                                DropdownMenuItem { onclick: move |_| { nav.push(Route::TagsEditScreen { id: tag_id }); }, "Edit" }
                                                                DropdownMenuItem { onclick: move |_| { nav.push(Route::PostsListScreen {}); }, "View Posts" }
                                                                DropdownMenuItem { class: "text-red-600 dark:text-red-400", onclick: move |_| {
                                                                        let id = tag_id;
                                                                        spawn({  async move {
                                                                            tags_state.remove(id).await;
                                                                            tags_state.list().await;
                                                                        }});
                                                                    }, "Delete" }
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
                            // Pagination
                            Pagination::<Tag> {
                                page: list.data.clone(),
                                disabled: list_loading,
                                on_prev: move |_| {
                                    let new_page = current_page.saturating_sub(1).max(1);
                                    page.set(new_page);
                                    let q = TagsListQuery { page: Some(new_page), search: if search_query.read().is_empty() { None } else { Some(search_query.read().clone()) }, sort_order: Some(sort_order.read().clone()) };
                                    spawn(async move { tags_state.list_with_query(q).await; });
                                },
                                on_next: move |_| {
                                    let new_page = current_page + 1;
                                    page.set(new_page);
                                    let q = TagsListQuery { page: Some(new_page), search: if search_query.read().is_empty() { None } else { Some(search_query.read().clone()) }, sort_order: Some(sort_order.read().clone()) };
                                    spawn(async move { tags_state.list_with_query(q).await; });
                                },
                            }
                        }
                        // Loading overlay when we have data
                        if list_loading && has_data { LoadingOverlay { visible: true } }
                    }
                }
            }
        }
    }
}
