use dioxus::prelude::*;

use crate::router::Route;
use crate::store::{use_tag, Tag, TagsListQuery, SortParam};
use crate::components::{DataTableScreen, ListEmptyState, ListToolbarProps, PageHeaderProps, ListErrorBannerProps, SkeletonTableRows, SkeletonCellConfig};
use crate::ui::shadcn::{
    Badge, BadgeVariant, Button, ButtonVariant, DropdownMenu, DropdownMenuContent,
    DropdownMenuItem, DropdownMenuTrigger,
};
use crate::utils::dates::format_short_date_dt;
use hmziq_dioxus_free_icons::{
    icons::ld_icons::{LdEllipsis, LdArrowUpDown},
    Icon,
};

use std::time::Duration;
use gloo_timers::future::sleep;

#[component]
pub fn TagsListScreen() -> Element {
    let nav = use_navigator();
    let tags_state = use_tag();
    
    let mut filters = use_signal(|| TagsListQuery::new());
    let mut search_input = use_signal(|| String::new());
    let mut reload_tick = use_signal(|| 0u32);
    let mut sort_field = use_signal(|| "name".to_string());
    let mut sort_order = use_signal(|| "asc".to_string());


    use_effect(move || {
        let q = filters();
        let _tick = reload_tick();
        let tags_state = tags_state;
        spawn(async move {
            tags_state.list_with_query(q).await;
        });
    });

    let list = tags_state.list.read();
    let list_loading = list.is_loading();
    let list_failed = list.is_failed();

    let (tags, current_page) = if let Some(p) = &list.data {
        (p.data.clone(), p.page)
    } else {
        (Vec::<Tag>::new(), 1)
    };

    let has_data = !tags.is_empty();

    // Define header columns
    let headers = vec![
        ("Name", true, "py-3 px-4 text-left font-medium text-sm", Some("name")),
        ("Description", false, "hidden py-3 px-4 text-left font-medium text-sm md:table-cell", None),
        ("Posts", false, "hidden py-3 px-4 text-left font-medium text-sm md:table-cell", None),
        ("Created", true, "hidden py-3 px-4 text-left font-medium text-sm md:table-cell", Some("created_at")),
        ("Status", false, "py-3 px-4 text-left font-medium text-sm", None),
        ("", false, "w-12 py-3 px-4", None),
    ];

    let mut handle_sort = move |field: String| {
        let current_field = sort_field();
        let current_order = sort_order();
        
        if current_field == field {
            // Toggle order for same field
            let new_order = if current_order == "asc" { "desc" } else { "asc" };
            sort_order.set(new_order.to_string());
        } else {
            // New field, default to asc
            sort_field.set(field.clone());
            sort_order.set("asc".to_string());
        }
        
        // Update filters with new sort
        let mut q = filters.peek().clone();
        q.page = 1; // Reset to first page when sorting
        q.sorts = Some(vec![SortParam {
            field: sort_field(),
            order: sort_order(),
        }]);
        filters.set(q);
    };

    rsx! {
        DataTableScreen::<Tag> {
            frame: (tags_state.list)(),
            header: Some(PageHeaderProps {
                title: "Tags".to_string(),
                description: "Organize your content with tags".to_string(),
                actions: Some(rsx!{ 
                    Button { 
                        onclick: move |_| { nav.push(Route::TagsAddScreen {}); }, 
                        "New Tag" 
                    } 
                }),
                class: None,
                embedded: false,
            }),
            error_banner: Some(ListErrorBannerProps {
                message: "Failed to load tags. Please try again.".to_string(),
                retry_label: Some("Retry".to_string()),
                on_retry: Some(EventHandler::new(move |_| {
                    let next = *reload_tick.peek() + 1u32;
                    reload_tick.set(next);
                })),
            }),
            toolbar: Some(ListToolbarProps {
                search_value: search_input(),
                search_placeholder: "Search tags by name, description, or slug".to_string(),
                disabled: list_loading,
                on_search_input: EventHandler::new(move |val: String| {
                    search_input.set(val.clone());
                    spawn(async move {
                        sleep(Duration::from_millis(500)).await;
                        if search_input.peek().as_str() == val.as_str() {
                            let mut q = filters.peek().clone();
                            q.page = 1;
                            q.search = if val.is_empty() { None } else { Some(val) };
                            filters.set(q);
                        }
                    });
                }),
                status_selected: match filters.read().is_active {
                    Some(true) => "Active".to_string(),
                    Some(false) => "Inactive".to_string(),
                    None => "All".to_string(),
                },
                on_status_select: EventHandler::new(move |value: String| {
                    let mut q = filters.peek().clone();
                    q.page = 1;
                    q.is_active = match value.as_str() {
                        "Active" | "active" => Some(true),
                        "Inactive" | "inactive" => Some(false),
                        _ => None,
                    };
                    filters.set(q);
                }),
            }),
            on_prev: move |_| {
                let new_page = current_page.saturating_sub(1).max(1);
                let mut q = filters.peek().clone();
                q.page = new_page;
                filters.set(q);
            },
            on_next: move |_| {
                let new_page = current_page + 1;
                let mut q = filters.peek().clone();
                q.page = new_page;
                filters.set(q);
            },
            div { class: "bg-transparent border-zinc-200 dark:border-zinc-800",
                table { class: "w-full",
                    thead { class: "bg-transparent",
                        tr { class: "border-b border-zinc-200 dark:border-zinc-800 hover:bg-transparent",
                            {headers.iter().map(|(label, sortable, class, field)| {
                                let current_sort_field = sort_field();
                                let is_current_sort = field.as_ref().map_or(false, |f| f == &current_sort_field);
                                
                                rsx! {
                                    th { class: "{class}",
                                        if *sortable {
                                            Button {
                                                variant: ButtonVariant::Ghost,
                                                class: "h-8 bg-transparent hover:bg-muted/50 -ml-3 text-left justify-start font-medium p-2",
                                                onclick: {
                                                    let field = field.clone().unwrap_or_default().to_string();
                                                    move |_| handle_sort(field.clone())
                                                },
                                                "{label}"
                                                if is_current_sort {
                                                    div { class: "ml-2 h-4 w-4", Icon { icon: LdArrowUpDown {} } }
                                                }
                                            }
                                        } else {
                                            "{label}"
                                        }
                                    }
                                }
                            })}
                        }
                    }
                    tbody {
                        if tags.is_empty() {
                            if list_loading && !has_data {
                                SkeletonTableRows {
                                    row_count: 6,
                                    cells: vec![
                                        SkeletonCellConfig::avatar(),
                                        SkeletonCellConfig::custom(crate::components::UICellType::Default, "hidden py-3 px-4 md:table-cell"),
                                        SkeletonCellConfig::default(true),
                                        SkeletonCellConfig::default(true),
                                        SkeletonCellConfig::badge(),
                                        SkeletonCellConfig::action(),
                                    ],
                                }
                            } else {
                                tr { class: "border-b border-zinc-200 dark:border-zinc-800",
                                    td { colspan: "6", class: "py-12 px-4 text-center",
                                        ListEmptyState {
                                            title: "No tags found".to_string(),
                                            description: "Try adjusting your search or create a new tag to get started.".to_string(),
                                            clear_label: "Clear search".to_string(),
                                            create_label: "Create your first tag".to_string(),
                                            on_clear: move |_| {
                                                // Reset UI and filters
                                                search_input.set(String::new());
                                                filters.set(TagsListQuery::new());
                                            },
                                            on_create: move |_| { nav.push(Route::TagsAddScreen {}); },
                                        }
                                    }
                                }
                            }
                        } else {
                            {tags.iter().cloned().map(|tag| {
                                let tag_id = tag.id;
                                rsx! {
                                tr { class: "border-b border-zinc-200 dark:border-zinc-800 hover:bg-muted/30 transition-colors",
                                    td { class: "py-3 px-4",
                                        div { class: "flex items-center gap-3",
                                            div { class: "h-4 w-4 text-muted-foreground", "#" }
                                            div { class: "min-w-0",
                                                div { class: "font-medium leading-none", "{tag.name}" }
                                                div { class: "mt-1 text-xs text-muted-foreground", {tag.description.clone().unwrap_or_default()} }
                                            }
                                        }
                                    }
                                    td { class: "hidden max-w-xs py-3 px-4 text-muted-foreground md:table-cell", span { class: "truncate", {tag.description.clone().unwrap_or("—".to_string())} } }
                                    td { class: "hidden py-3 px-4 text-muted-foreground md:table-cell", "0" }
                                    td { class: "hidden py-3 px-4 text-muted-foreground md:table-cell", "{format_short_date_dt(&tag.created_at)}" }
                                    td { class: "py-3 px-4",
                                        if tag.is_active {
                                            Badge { class: "bg-green-100 text-green-800 border-green-200 dark:bg-green-900/20 dark:text-green-400", "Active" }
                                        } else {
                                            Badge { variant: BadgeVariant::Secondary, class: "bg-gray-100 text-gray-800 border-gray-200 dark:bg-gray-900/20 dark:text-gray-400", "Inactive" }
                                        }
                                    }
                                    td { class: "py-3 px-4",
                                        DropdownMenu {
                                            DropdownMenuTrigger {
                                                Button { variant: ButtonVariant::Ghost, class: "h-8 w-8 p-0 bg-transparent hover:bg-muted/50", div { class: "w-4 h-4", Icon { icon: LdEllipsis {} } } }
                                            }
                                            DropdownMenuContent { class: "bg-background border-zinc-200 dark:border-zinc-800",
                                                DropdownMenuItem { onclick: move |_| { nav.push(Route::TagsEditScreen { id: tag_id }); }, "Edit" }
                                                DropdownMenuItem { onclick: move |_| { nav.push(Route::PostsListScreen {}); }, "View Posts" }
                                                DropdownMenuItem { class: "text-red-600", onclick: move |_| {
                                                        let id = tag_id;
                                                        spawn({  async move {
                                                            tags_state.remove(id).await;
                                                        }});
                                                    }, "Delete" }
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
        }
    }
}
