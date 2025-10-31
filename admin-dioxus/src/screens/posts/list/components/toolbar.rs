use dioxus::prelude::*;

use crate::store::PostStatus;
use crate::ui::shadcn::{Button, ButtonSize, ButtonVariant};

use hmziq_dioxus_free_icons::{
    icons::ld_icons::{LdGrid3x3, LdLayoutList, LdSearch},
    Icon,
};

use super::super::context::{use_post_list_context, ViewMode};
use super::filter_popover::FilterPopover;

#[component]
pub fn Toolbar(
    list_loading: bool,
    search_input: String,
    on_search: EventHandler<String>,
) -> Element {
    let ctx = use_post_list_context();
    let active_filter_count = ctx.active_filter_count();

    rsx! {
        div { class: "flex flex-col gap-4 md:flex-row md:items-center md:justify-between",
            // Search input
            div { class: "relative w-full md:w-96",
                div { class: "absolute left-2.5 top-2.5 h-4 w-4 text-zinc-500 dark:text-zinc-400 pointer-events-none",
                    Icon { icon: LdSearch {}, class: "w-4 h-4" }
                }
                input {
                    r#type: "search",
                    placeholder: "Search posts by title, content, or author",
                    value: search_input,
                    class: "w-full pl-8 h-9 rounded-md border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 px-3 text-sm focus:outline-none focus:ring-1 focus:ring-zinc-400 dark:focus:ring-zinc-600",
                    disabled: list_loading,
                    oninput: move |evt| {
                        on_search.call(evt.value());
                    },
                }
            }

            // Filters and view controls
            div { class: "flex items-center gap-2",
                // Status filter
                {
                    let current_status = ctx.filters.read().status.as_ref().map(|s| match s {
                        PostStatus::Published => "Published",
                        PostStatus::Draft => "Draft",
                        PostStatus::Archived => "Archived",
                    }).unwrap_or("All Status");
                    let mut ctx_clone = ctx.clone();
                    rsx! {
                        select {
                            class: "h-9 rounded-md border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 px-3 text-sm focus:outline-none focus:ring-1 focus:ring-zinc-400 dark:focus:ring-zinc-600",
                            value: current_status,
                            onchange: move |evt| {
                                let status = match evt.value().as_str() {
                                    "Published" => Some(PostStatus::Published),
                                    "Draft" => Some(PostStatus::Draft),
                                    "Archived" => Some(PostStatus::Archived),
                                    _ => None,
                                };
                                ctx_clone.set_status_filter(status);
                            },
                            option { value: "All Status", "All Status" }
                            option { value: "Published", "Published" }
                            option { value: "Draft", "Draft" }
                            option { value: "Archived", "Archived" }
                        }
                    }
                }

                // Advanced filters popover
                FilterPopover { active_filter_count }

                // View mode switcher
                {
                    let current_mode = *ctx.view_mode.read();
                    let mut ctx_clone1 = ctx.clone();
                    let mut ctx_clone2 = ctx.clone();
                    rsx! {
                        div { class: "flex items-center gap-1 border border-zinc-200 dark:border-zinc-800 rounded-md p-1",
                            Button {
                                variant: if current_mode == ViewMode::Grid {
                                    ButtonVariant::Default
                                } else {
                                    ButtonVariant::Ghost
                                },
                                size: ButtonSize::Sm,
                                class: "h-8 w-8 p-0",
                                onclick: move |_| { ctx_clone1.view_mode.set(ViewMode::Grid); },
                                Icon { icon: LdGrid3x3 {}, class: "w-4 h-4" }
                            }
                            Button {
                                variant: if current_mode == ViewMode::Table {
                                    ButtonVariant::Default
                                } else {
                                    ButtonVariant::Ghost
                                },
                                size: ButtonSize::Sm,
                                class: "h-8 w-8 p-0",
                                onclick: move |_| { ctx_clone2.view_mode.set(ViewMode::Table); },
                                Icon { icon: LdLayoutList {}, class: "w-4 h-4" }
                            }
                        }
                    }
                }
            }
        }
    }
}
