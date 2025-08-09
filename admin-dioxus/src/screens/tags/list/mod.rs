use dioxus::prelude::*;

use crate::router::Route;
use crate::store::{use_tag, Tag, TagsListQuery};
use crate::ui::shadcn::{
    Badge, BadgeVariant, Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator, Button, ButtonVariant, Card, DropdownMenu, DropdownMenuContent,
    DropdownMenuItem, DropdownMenuTrigger,
};
use chrono::NaiveDateTime;
use hmziq_dioxus_free_icons::{
    icons::ld_icons::{LdEllipsis, LdSearch, LdTag},
    Icon,
};

#[component]
pub fn TagsListScreen() -> Element {
    let nav = use_navigator();
    let tags_state = use_tag();
    let mut search_query = use_signal(|| String::new());
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
    if list.is_loading() {
        return rsx! {
            div { class: "flex items-center justify-center h-full py-16",
                p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Loading tags..." }
            }
        };
    }
    if list.is_failed() {
        return rsx! {
            div { class: "flex items-center justify-center h-full py-16",
                h1 { class: "text-sm text-red-600 dark:text-red-400", "Failed to load tags. " }
                if let Some(message) = &list.message {
                    p { class: "text-sm text-red-600 dark:text-red-400", "{message}" }
                }
            }
        };
    }

    // Snapshot data for rendering
    let (tags, total_items, current_page, _per_page) = if let Some(p) = list.data.clone() {
        (p.data.clone(), p.total, p.page, p.per_page)
    } else {
        (Vec::<Tag>::new(), 0, 1, 10)
    };

    let total = tags.len();
    let active = tags.iter().filter(|t| t.is_active).count();
    let inactive = total.saturating_sub(active);

    rsx! {
        // Page wrapper
        div { class: "min-h-screen bg-zinc-50 dark:bg-zinc-950 text-zinc-900 dark:text-zinc-50",
            // Top region with breadcrumb and header
            div { class: "border-b border-zinc-200 dark:border-zinc-800 bg-gradient-to-b from-zinc-50/60 to-transparent dark:from-zinc-950/40",
                div { class: "container mx-auto px-4 py-8 md:py-12",
                    // Breadcrumb
                    Breadcrumb {
                        BreadcrumbList {
                            BreadcrumbItem { BreadcrumbLink { href: "/dashboard".to_string(), "Dashboard" } }
                            BreadcrumbSeparator {}
                            BreadcrumbItem { BreadcrumbPage { "Tags" } }
                        }
                    }

                    // Header row
                    div { class: "mt-6 flex flex-col items-start justify-between gap-6 md:flex-row md:items-center",
                        div { class: "space-y-2",
                            h1 { class: "text-3xl md:text-4xl font-bold tracking-tight", "Tags" }
                            p { class: "text-sm md:text-base text-zinc-600 dark:text-zinc-400",
                                "Organize your content with tags. Create, edit, and manage tags."
                            }
                        }
                        div { class: "flex items-center gap-2",
                            Button { onclick: move |_| { nav.push(Route::TagsAddScreen {}); }, "New Tag" }
                        }
                    }

                    // Stats
                    div { class: "mt-6 grid grid-cols-1 gap-2 sm:grid-cols-3",
                        Card { class: "border-muted",
                            div { class: "flex items-center justify-between p-4",
                                div { class: "space-y-1",
                                    p { class: "text-xs font-medium uppercase tracking-wide text-zinc-500 dark:text-zinc-400", "Total" }
                                    p { class: "text-2xl font-semibold tabular-nums", "{total}" }
                                }
                                div { class: "w-5 h-5 text-zinc-500 dark:text-zinc-400", Icon { icon: LdTag {} } }
                            }
                        }
                        Card { class: "border-muted",
                            div { class: "flex items-center justify-between p-4",
                                div { class: "space-y-1",
                                    p { class: "text-xs font-medium uppercase tracking-wide text-zinc-500 dark:text-zinc-400", "Active" }
                                    p { class: "text-2xl font-semibold tabular-nums", "{active}" }
                                }
                                div { class: "h-5 w-5 rounded-full bg-green-500/15 ring-4 ring-green-500/10" }
                            }
                        }
                        Card { class: "border-muted",
                            div { class: "flex items-center justify-between p-4",
                                div { class: "space-y-1",
                                    p { class: "text-xs font-medium uppercase tracking-wide text-zinc-500 dark:text-zinc-400", "Inactive" }
                                    p { class: "text-2xl font-semibold tabular-nums", "{inactive}" }
                                }
                                div { class: "h-5 w-5 rounded-full bg-zinc-500/15 ring-4 ring-zinc-500/10" }
                            }
                        }
                    }
                }
            }

            // Content
            div { class: "container mx-auto px-4 py-8",
                // Toolbar
                Card { class: "border-muted",
                    div { class: "flex flex-col gap-3 p-4 md:flex-row md:items-center md:justify-between",
                        // Search
                        div { class: "w-full md:w-96",
                            label { class: "sr-only", r#for: "search", "Search tags" }
                            div { class: "relative",
                                div { class: "pointer-events-none absolute left-2.5 top-2.5 h-4 w-4 text-zinc-500 dark:text-zinc-400", Icon { icon: LdSearch {} } }
                                input {
                                    id: "search",
                                    r#type: "search",
                                    class: "pl-8 w-full h-9 rounded-md border border-input bg-transparent px-3 text-sm shadow-sm",
                                    placeholder: "Search tags by name, description, or slug",
                                    value: search_query.read().clone(),
                                    oninput: move |e| {
                                        let val = e.value();
                                        search_query.set(val.clone());
                                        page.set(1);
                                        let q = TagsListQuery { page: Some(1), search: if val.is_empty() { None } else { Some(val) }, sort_order: Some(sort_order.read().clone()) };
                                        spawn({
                                            let tags_state = use_tag();
                                            async move { tags_state.list_with_query(q).await; }
                                        });
                                    },
                                }
                            }
                        }

                        // Sort + Active Filters
                        div { class: "flex w-full items-center gap-2 md:w-auto",
                            div { class: "w-full md:w-48",
                                label { class: "sr-only", r#for: "sort", "Sort order" }
                                select {
                                    id: "sort",
                                    class: "w-full h-9 rounded-md border border-input bg-transparent px-3 text-sm shadow-sm",
                                    value: sort_order.read().clone(),
                                    oninput: move |e| {
                                        let s = e.value();
                                        sort_order.set(s.clone());
                                        page.set(1);
                                        let q = TagsListQuery { page: Some(1), search: if search_query.read().is_empty() { None } else { Some(search_query.read().clone()) }, sort_order: Some(s) };
                                        spawn({
                                            let tags_state = use_tag();
                                            async move { tags_state.list_with_query(q).await; }
                                        });
                                    },
                                    option { value: "desc", "Name: Z → A" }
                                    option { value: "asc", "Name: A → Z" }
                                }
                            }

                            div { class: "flex-1" }

                            div { class: "flex items-center gap-2",
                                if !search_query.read().is_empty() {
                                    Badge { variant: BadgeVariant::Secondary, class: "max-w-[14rem] cursor-pointer truncate",
                                        onclick: move |_| {
                                            search_query.set(String::new());
                                            page.set(1);
                                            let q = TagsListQuery { page: Some(1), search: None, sort_order: Some(sort_order.read().clone()) };
                                            spawn({
                                                let tags_state = use_tag();
                                                async move { tags_state.list_with_query(q).await; }
                                            });
                                        },
                                        "Search: \"{search_query.read().clone()}\""
                                    }
                                }
                                Badge { variant: BadgeVariant::Secondary, class: "cursor-default", "Sort: {sort_order.read().clone()}" }
                            }
                        }
                    }
                }

                // Table
                Card { class: "border-muted overflow-hidden mt-4",
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
                                    if tags.is_empty() {
                                        tr { class: "border-b border-border/60",
                                            td { colspan: "6", class: "py-16 px-4",
                                                div { class: "flex flex-col items-center justify-center gap-3 text-center",
                                                    div { class: "flex h-12 w-12 items-center justify-center rounded-full bg-muted",
                                                        div { class: "h-6 w-6 text-muted-foreground", Icon { icon: LdTag {} } }
                                                    }
                                                    div { class: "space-y-1",
                                                        h3 { class: "text-lg font-medium", "No tags found" }
                                                        p { class: "text-sm text-muted-foreground", "Try adjusting your search or create a new tag to get started." }
                                                    }
                                                    div { class: "flex flex-col items-center gap-2 sm:flex-row",
                                                        Button { variant: ButtonVariant::Outline, onclick: move |_| {search_query.set(String::new());}, "Clear search" }
                                                        Button { onclick: move |_| {nav.push(Route::TagsAddScreen {});}, "Create your first tag" }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {tags.iter().map(|tag| rsx! {
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
                                                td { class: "hidden py-3 px-4 text-muted-foreground md:table-cell", "{format_short_date(&tag.created_at)}" }
                                                td { class: "py-3 px-4",
                                                    if tag.is_active {
                                                        Badge { "Active" }
                                                    } else {
                                                        Badge { variant: BadgeVariant::Secondary, "Inactive" }
                                                    }
                                                }
                                                td { class: "py-3 px-4",
                                                    div { class: "flex items-center justify-end gap-1.5",
                                                        DropdownMenu {
                                                            DropdownMenuTrigger {
                                                                Button { variant: ButtonVariant::Ghost, class: "h-8 w-8", div { class: "w-4 h-4", Icon { icon: LdEllipsis {} } } }
                                                            }
                                                            DropdownMenuContent { class: "w-44 border-border bg-popover",
                                                                DropdownMenuItem { "Edit" }
                                                                DropdownMenuItem { "View Posts" }
                                                                DropdownMenuItem { class: "text-red-600 dark:text-red-400", "Delete" }
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
                            div { class: "flex items-center justify-between border-t border-border/60 p-3 text-sm text-muted-foreground",
                                div { class: "hidden md:block", "Page {current_page} • {total_items} items" }
                                div { class: "flex items-center gap-2 ml-auto",
                                    Button { variant: ButtonVariant::Outline, disabled: !list.data.clone().map(|p| p.has_previous_page()).unwrap_or(false), onclick: move |_| {
                                            let new_page = current_page.saturating_sub(1).max(1);
                                            page.set(new_page);
                                            let q = TagsListQuery { page: Some(new_page), search: if search_query.read().is_empty() { None } else { Some(search_query.read().clone()) }, sort_order: Some(sort_order.read().clone()) };
                                            spawn({ let tags_state = use_tag(); async move { tags_state.list_with_query(q).await; } });
                                        }, "Previous" }
                                    Button { disabled: !list.data.clone().map(|p| p.has_next_page()).unwrap_or(false), onclick: move |_| {
                                            let new_page = current_page + 1;
                                            page.set(new_page);
                                            let q = TagsListQuery { page: Some(new_page), search: if search_query.read().is_empty() { None } else { Some(search_query.read().clone()) }, sort_order: Some(sort_order.read().clone()) };
                                            spawn({ let tags_state = use_tag(); async move { tags_state.list_with_query(q).await; } });
                                        }, "Next" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn format_short_date(date_str: &str) -> String {
    if let Ok(date) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S.%f") {
        date.format("%b %-d, %Y").to_string()
    } else {
        date_str.to_string()
    }
}
