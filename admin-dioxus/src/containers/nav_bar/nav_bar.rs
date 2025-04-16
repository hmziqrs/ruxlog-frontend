use dioxus::prelude::*;
use hmziq_dioxus_free_icons::{Icon, IconShape};
// Import the specific lucide icons we need
use hmziq_dioxus_free_icons::icons::ld_icons::{
    LdHome, LdFileText, LdFolder, LdTag, LdUser, LdLogOut,
    LdMenu, LdBell, LdSearch, LdSettings
};

use crate::{router::Route, store::use_auth};

#[component]
pub fn NavBarContainer() -> Element {
    let auth_store = use_auth();
    let auth_user = auth_store.user.read();
    let current_route = use_route::<Route>();
    let mut sidebar_open = use_signal(|| false);

    // Helper function to determine if a route is active
    let is_active = |route: Route| -> bool {
        // Simply compare the route variants, ignoring any internal data
        std::mem::discriminant(&current_route) == std::mem::discriminant(&route)
    };

    rsx! {
        if auth_user.is_some() {
            // Main wrapper (sidebar + content)
            div { class: "flex min-h-screen bg-zinc-900",
                // Sidebar (mobile version will be hidden by default)
                div {
                    class: format_args!(
                        "fixed inset-y-0 left-0 z-50 w-64 transform bg-zinc-800 shadow-lg transition-transform duration-300 ease-in-out sm:relative sm:translate-x-0 {}",
                        if *sidebar_open.read() { "translate-x-0" } else { "-translate-x-full" },
                    ),
                    // Sidebar header
                    div { class: "flex h-16 items-center justify-between border-b border-zinc-700 px-4",
                        div { class: "flex items-center space-x-2",
                            img {
                                class: "h-8 w-8",
                                src: asset!("/assets/logo.png"),
                                alt: "Logo",
                            }
                            h1 { class: "text-lg font-bold text-white", "Ruxlog Admin" }
                        }
                        // Close sidebar button (mobile only)
                        button {
                            class: "rounded-md p-2 text-gray-400 hover:bg-zinc-700 hover:text-white sm:hidden",
                            onclick: move |_| sidebar_open.set(false),
                            Icon { icon: LdMenu, width: 20, height: 20 }
                        }
                    }

                    // Sidebar navigation
                    nav { class: "mt-4 px-2",
                        // Navigation items
                        div { class: "space-y-1",
                            Link {
                                class: format_args!(
                                    "flex items-center rounded-lg px-3 py-2 text-sm font-medium {} transition-colors",
                                    if is_active(Route::HomeScreen {}) {
                                        "bg-zinc-700 text-white"
                                    } else {
                                        "text-zinc-300 hover:bg-zinc-700 hover:text-white"
                                    },
                                ),
                                to: Route::HomeScreen {},
                                Icon { icon: LdHome, width: 18, height: 18 }
                                span { class: "ml-3", "Dashboard" }
                            }

                            Link {
                                class: format_args!(
                                    "flex items-center rounded-lg px-3 py-2 text-sm font-medium {} transition-colors",
                                    if is_active(Route::AddBlogScreen {}) {
                                        "bg-zinc-700 text-white"
                                    } else {
                                        "text-zinc-300 hover:bg-zinc-700 hover:text-white"
                                    },
                                ),
                                to: Route::AddBlogScreen {},
                                Icon { icon: LdFileText, width: 18, height: 18 }
                                span { class: "ml-3", "Posts" }
                            }

                            Link {
                                class: format_args!(
                                    "flex items-center rounded-lg px-3 py-2 text-sm font-medium {} transition-colors",
                                    if is_active(Route::AddCategoryScreen {}) {
                                        "bg-zinc-700 text-white"
                                    } else {
                                        "text-zinc-300 hover:bg-zinc-700 hover:text-white"
                                    },
                                ),
                                to: Route::AddCategoryScreen {},
                                Icon { icon: LdFolder, width: 18, height: 18 }
                                span { class: "ml-3", "Categories" }
                            }

                            Link {
                                class: format_args!(
                                    "flex items-center rounded-lg px-3 py-2 text-sm font-medium {} transition-colors",
                                    if is_active(Route::AddTagScreen {}) {
                                        "bg-zinc-700 text-white"
                                    } else {
                                        "text-zinc-300 hover:bg-zinc-700 hover:text-white"
                                    },
                                ),
                                to: Route::AddTagScreen {},
                                Icon { icon: LdTag, width: 18, height: 18 }
                                span { class: "ml-3", "Tags" }
                            }

                            Link {
                                class: format_args!(
                                    "flex items-center rounded-lg px-3 py-2 text-sm font-medium {} transition-colors",
                                    if is_active(Route::AddUserScreen {}) {
                                        "bg-zinc-700 text-white"
                                    } else {
                                        "text-zinc-300 hover:bg-zinc-700 hover:text-white"
                                    },
                                ),
                                to: Route::AddUserScreen {},
                                Icon { icon: LdUser, width: 18, height: 18 }
                                span { class: "ml-3", "Users" }
                            }
                        }
                    }
                    // Sidebar footer with logout button
                    div { class: "mt-auto border-t border-zinc-700 p-4",
                        button {
                            class: "flex w-full items-center rounded-lg px-3 py-2 text-sm font-medium text-zinc-300 hover:bg-zinc-700 hover:text-white",
                            onclick: move |_| {
                                auth_store.logout();
                            },
                            Icon { icon: LdLogOut, width: 18, height: 18 }
                            span { class: "ml-3", "Logout" }
                        }
                    }
                }

                // Main content area
                div { class: "flex flex-1 flex-col",
                    // Top navigation bar
                    header { class: "bg-zinc-800 shadow",
                        div { class: "flex h-16 items-center justify-between px-4",
                            // Mobile menu button
                            button {
                                class: "rounded-md p-2 text-gray-400 hover:bg-zinc-700 hover:text-white sm:hidden",
                                onclick: move |_| sidebar_open.set(true),
                                Icon { icon: LdMenu, width: 20, height: 20 }
                            }

                            // Search bar (hidden on small screens)
                            div { class: "hidden md:block flex-1 px-4 lg:px-6",
                                div { class: "relative max-w-md",
                                    div { class: "pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3",
                                        Icon {
                                            icon: LdSearch,
                                            width: 16,
                                            height: 16,
                                            class: "text-zinc-400",
                                        }
                                    }
                                    input {
                                        class: "block w-full rounded-lg border border-zinc-600 bg-zinc-700 py-2 pl-10 pr-3 text-sm placeholder:text-zinc-400 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500",
                                        placeholder: "Search...",
                                        r#type: "text",
                                    }
                                }
                            }

                            // Right side nav items
                            div { class: "flex items-center space-x-4",
                                button { class: "rounded-full p-1 text-zinc-400 hover:bg-zinc-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-800",
                                    Icon { icon: LdBell, width: 20, height: 20 }
                                }
                                button { class: "rounded-full p-1 text-zinc-400 hover:bg-zinc-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-800",
                                    Icon {
                                        icon: LdSettings,
                                        width: 20,
                                        height: 20,
                                    }
                                }
                                // User profile
                                div { class: "relative ml-3",
                                    div { class: "flex rounded-full bg-zinc-700 text-sm focus:outline-none",
                                        img {
                                            class: "h-8 w-8 rounded-full",
                                            src: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80",
                                            alt: "User",
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Page content
                    main { class: "flex-1 overflow-y-auto p-6 bg-zinc-900", Outlet::<Route> {} }
                }
            }
        } else {
            Outlet::<Route> {}
        }
    }
}
