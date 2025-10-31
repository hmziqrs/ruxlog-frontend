use dioxus::prelude::*;

use crate::components::{
    DataTableScreen, HeaderColumn, ListEmptyState, ListErrorBannerProps, ListToolbarProps,
    PageHeaderProps, SkeletonCellConfig, SkeletonTableRows,
};
use crate::hooks::{use_list_screen_with_handlers, ListScreenConfig};
use crate::router::Route;
use crate::store::{use_user, ListQuery, ListStore, User, UserRole, UsersListQuery};
use crate::types::Order;
use crate::ui::shadcn::{
    Badge, BadgeVariant, Button, ButtonVariant, Checkbox, DropdownMenu, DropdownMenuContent,
    DropdownMenuItem, DropdownMenuTrigger,
};
use crate::utils::dates::format_short_date_dt;

use hmziq_dioxus_free_icons::{icons::ld_icons::LdEllipsis, Icon};

fn generate_avatar_fallback(name: &str) -> String {
    let initials: String = name
        .split_whitespace()
        .take(2)
        .filter_map(|word| word.chars().next())
        .collect::<String>()
        .to_uppercase();
    initials
}

#[component]
pub fn UsersListScreen() -> Element {
    let nav = use_navigator();
    let users_state = use_user();

    let filters = use_signal(|| UsersListQuery::new());
    // Local selection state for the current page
    let selected_ids = use_signal(|| Vec::<i32>::new());

    // Use the enhanced hook that creates handlers for us
    let (list_state, handlers) = use_list_screen_with_handlers(
        Some(ListScreenConfig {
            default_sort_field: "name".to_string(),
            default_sort_order: Order::Asc,
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
            let users_state = users_state;
            // Clear any selection on query changes (page, search, filters, sorts)
            selected_ids.set(Vec::new());
            spawn(async move {
                users_state.fetch_list_with_query(q).await;
            });
        }
    });

    let list = users_state.list.read();
    let list_loading = list.is_loading();
    let _list_failed = list.is_failed();

    let (users, current_page) = if let Some(p) = &list.data {
        (p.data.clone(), p.page)
    } else {
        (Vec::<User>::new(), 1)
    };

    let has_data = !users.is_empty();

    // Define header columns (prepend a blank cell for the selection checkbox column)
    let headers = vec![
        HeaderColumn::new("", false, "w-12 py-2 px-3", None),
        HeaderColumn::new(
            "User",
            true,
            "py-2 px-3 text-left font-medium text-xs md:text-sm whitespace-nowrap",
            Some("name"),
        ),
        HeaderColumn::new(
            "Email",
            true,
            "py-2 px-3 text-left font-medium text-xs md:text-sm whitespace-nowrap",
            Some("email"),
        ),
        HeaderColumn::new(
            "Role",
            true,
            "py-2 px-3 text-left font-medium text-xs md:text-sm whitespace-nowrap",
            Some("role"),
        ),
        HeaderColumn::new(
            "Verified",
            false,
            "py-2 px-3 text-left font-medium text-xs md:text-sm whitespace-nowrap",
            None,
        ),
        HeaderColumn::new(
            "2FA",
            false,
            "py-2 px-3 text-left font-medium text-xs md:text-sm whitespace-nowrap",
            None,
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

    // Custom verification filter handler
    let handle_verified_select = {
        let mut filters = filters;
        move |value: String| {
            let mut q = filters.peek().clone();
            q.set_page(1);
            q.status = match value.as_str() {
                "verified" | "Verified" => Some(true),
                "unverified" | "Unverified" => Some(false),
                _ => None,
            };
            filters.set(q);
        }
    };

    rsx! {
        DataTableScreen::<User> {
            frame: (users_state.list)(),
            header: Some(PageHeaderProps {
                title: "Users".to_string(),
                description: "Manage user accounts, roles, permissions, and verification status.".to_string(),
                actions: Some(rsx!{
                    Button {
                        onclick: move |_| { nav.push(Route::UsersAddScreen {}); },
                        "New User"
                    }
                }),
                class: None,
                embedded: false,
            }),
            headers: Some(headers),
            current_sort_field: Some(list_state.sort_field()),
            on_sort: Some(handlers.handle_sort.clone()),
            error_banner: Some(ListErrorBannerProps {
                message: "Failed to load users. Please try again.".to_string(),
                retry_label: Some("Retry".to_string()),
                on_retry: Some(EventHandler::new(move |_| handlers.handle_retry.call(()))),
            }),
            toolbar: Some(ListToolbarProps {
                search_value: list_state.search_input(),
                search_placeholder: "Search users by name or email".to_string(),
                disabled: list_loading,
                on_search_input: handlers.handle_search.clone(),
                status_selected: match filters.read().status {
                    Some(true) => "Verified".to_string(),
                    Some(false) => "Unverified".to_string(),
                    None => "All".to_string(),
                },
                on_status_select: EventHandler::new(handle_verified_select),
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
                                    move |_| { selected_ids.set(Vec::new()); }
                                },
                                "Verify"
                            }
                            Button { variant: ButtonVariant::Outline, class: "h-8",
                                onclick: {
                                    let mut selected_ids = selected_ids;
                                    move |_| { selected_ids.set(Vec::new()); }
                                },
                                "Change Role"
                            }
                            Button { variant: ButtonVariant::Outline, class: "h-8 text-red-600 border-red-200 dark:border-red-800 dark:hover:bg-red-950/20 hover:bg-red-50",
                                onclick: {
                                    let mut selected_ids = selected_ids;
                                    move |_| { selected_ids.set(Vec::new()); }
                                },
                                "Delete"
                            }
                        }
                    }
                })
            } else { None },
            // Table body content only - headers are now handled by DataTableScreen
            if users.is_empty() {
                if list_loading && !has_data {
                    SkeletonTableRows {
                        row_count: 6,
                        cells: vec![
                            // Selection checkbox placeholder
                            SkeletonCellConfig::custom(crate::components::UICellType::Default, "w-12 py-2 px-3"),
                            SkeletonCellConfig::custom(crate::components::UICellType::Default, "py-2 px-3"),
                            SkeletonCellConfig::custom(crate::components::UICellType::Default, "py-2 px-3"),
                            SkeletonCellConfig::custom(crate::components::UICellType::Badge, "py-2 px-3"),
                            SkeletonCellConfig::custom(crate::components::UICellType::Badge, "py-2 px-3"),
                            SkeletonCellConfig::custom(crate::components::UICellType::Badge, "py-2 px-3"),
                            SkeletonCellConfig::custom(crate::components::UICellType::Default, "py-2 px-3"),
                            SkeletonCellConfig::custom(crate::components::UICellType::Default, "py-2 px-3"),
                            SkeletonCellConfig::custom(crate::components::UICellType::Action, "py-2 px-3"),
                        ],
                    }
                } else {
                    tr { class: "border-b border-zinc-200 dark:border-zinc-800",
                        td { colspan: "9", class: "py-12 px-4 text-center",
                            ListEmptyState {
                                title: "No users found".to_string(),
                                description: "Try adjusting your search or create a new user to get started.".to_string(),
                                clear_label: "Clear search".to_string(),
                                create_label: "Create your first user".to_string(),
                                on_clear: move |_| { handlers.handle_clear.call(()); },
                                on_create: move |_| { nav.push(Route::UsersAddScreen {}); },
                            }
                        }
                    }
                }
            } else {
                {users.iter().cloned().map(|user| {
                    let user_id = user.id;
                    let initials = generate_avatar_fallback(&user.name);

                    rsx! {
                        tr { class: "border-b border-zinc-200 dark:border-zinc-800 hover:bg-muted/30 transition-colors",
                            // Selection checkbox cell
                            td { class: "py-2 px-3 w-12 text-xs md:text-sm",
                                Checkbox {
                                    checked: selected_ids.read().contains(&user_id),
                                    onchange: Some(EventHandler::new({
                                        let mut selected_ids = selected_ids;
                                        move |checked: bool| {
                                            let mut current = selected_ids.peek().clone();
                                            if checked {
                                                if !current.contains(&user_id) {
                                                    current.push(user_id);
                                                }
                                            } else {
                                                current.retain(|&id| id != user_id);
                                            }
                                            selected_ids.set(current);
                                        }
                                    })),
                                }
                            }
                            // User name and avatar
                            td { class: "py-2 px-3 text-xs md:text-sm whitespace-nowrap",
                                div { class: "flex items-center gap-3",
                                    if let Some(avatar) = &user.avatar {
                                        img {
                                            src: "{avatar.file_url}",
                                            alt: "{user.name}",
                                            class: "h-8 w-8 rounded-full object-cover ring-2 ring-black/5 dark:ring-white/10"
                                        }
                                    } else {
                                        div {
                                            class: "h-8 w-8 rounded-full bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white text-xs font-semibold ring-2 ring-black/5 dark:ring-white/10",
                                            "{initials}"
                                        }
                                    }
                                    span { class: "font-medium leading-none truncate", "{user.name}" }
                                }
                            }
                            // Email
                            td { class: "py-2 px-3 text-xs md:text-sm text-muted-foreground whitespace-nowrap",
                                span { class: "truncate", "{user.email}" }
                            }
                            // Role
                            td { class: "py-2 px-3 text-xs md:text-sm",
                                {match user.role {
                                    UserRole::SuperAdmin => rsx! {
                                        Badge { class: "bg-purple-100 text-purple-800 border-purple-200 dark:bg-purple-900/20 dark:text-purple-400", "Super Admin" }
                                    },
                                    UserRole::Admin => rsx! {
                                        Badge { class: "bg-red-100 text-red-800 border-red-200 dark:bg-red-900/20 dark:text-red-400", "Admin" }
                                    },
                                    UserRole::Moderator => rsx! {
                                        Badge { class: "bg-orange-100 text-orange-800 border-orange-200 dark:bg-orange-900/20 dark:text-orange-400", "Moderator" }
                                    },
                                    UserRole::Author => rsx! {
                                        Badge { class: "bg-blue-100 text-blue-800 border-blue-200 dark:bg-blue-900/20 dark:text-blue-400", "Author" }
                                    },
                                    UserRole::User => rsx! {
                                        Badge { variant: BadgeVariant::Secondary, class: "bg-gray-100 text-gray-800 border-gray-200 dark:bg-gray-900/20 dark:text-gray-400", "User" }
                                    },
                                }}
                            }
                            // Verified status
                            td { class: "py-2 px-3 text-xs md:text-sm",
                                if user.is_verified {
                                    Badge { class: "bg-green-100 text-green-800 border-green-200 dark:bg-green-900/20 dark:text-green-400", "Verified" }
                                } else {
                                    Badge { variant: BadgeVariant::Secondary, class: "bg-yellow-100 text-yellow-800 border-yellow-200 dark:bg-yellow-900/20 dark:text-yellow-400", "Unverified" }
                                }
                            }
                            // 2FA status
                            td { class: "py-2 px-3 text-xs md:text-sm",
                                if user.two_fa_enabled {
                                    Badge { class: "bg-green-100 text-green-800 border-green-200 dark:bg-green-900/20 dark:text-green-400", "Enabled" }
                                } else {
                                    Badge { variant: BadgeVariant::Secondary, class: "bg-gray-100 text-gray-800 border-gray-200 dark:bg-gray-900/20 dark:text-gray-400", "Disabled" }
                                }
                            }
                            // Created at
                            td { class: "py-2 px-3 text-xs md:text-sm text-muted-foreground whitespace-nowrap", "{format_short_date_dt(&user.created_at)}" }
                            // Updated at
                            td { class: "py-2 px-3 text-xs md:text-sm text-muted-foreground whitespace-nowrap", "{format_short_date_dt(&user.updated_at)}" }
                            // Actions dropdown
                            td { class: "py-2 px-3 text-xs md:text-sm",
                                DropdownMenu {
                                    DropdownMenuTrigger {
                                        Button { variant: ButtonVariant::Ghost, class: "h-8 w-8 p-0 bg-transparent hover:bg-muted/50", div { class: "w-4 h-4", Icon { icon: LdEllipsis {} } } }
                                    }
                                    DropdownMenuContent { class: "bg-background border-zinc-200 dark:border-zinc-800",
                                        DropdownMenuItem {
                                            onclick: move |_| {
                                                // TODO: Navigate to user view/edit screen
                                                // nav.push(Route::UsersEditScreen { id: user_id });
                                            },
                                            "Edit"
                                        }
                                        DropdownMenuItem {
                                            onclick: move |_| {
                                                // TODO: View user details
                                            },
                                            "View Details"
                                        }
                                        DropdownMenuItem {
                                            onclick: move |_| {
                                                // TODO: Reset password
                                            },
                                            "Reset Password"
                                        }
                                        if !user.is_verified {
                                            DropdownMenuItem {
                                                onclick: move |_| {
                                                    // TODO: Verify user
                                                },
                                                "Verify User"
                                            }
                                        }
                                        DropdownMenuItem { class: "text-red-600",
                                            onclick: move |_| {
                                                let id = user_id;
                                                spawn(async move {
                                                    users_state.remove(id).await;
                                                });
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
