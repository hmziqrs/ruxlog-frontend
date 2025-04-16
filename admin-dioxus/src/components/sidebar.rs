use dioxus::prelude::*;
use hmziq_dioxus_free_icons::{Icon, IconShape};
use hmziq_dioxus_free_icons::icons::ld_icons::{
    LdHome, LdFileText, LdFolder, LdTag, LdUser, LdLogOut,
};

use crate::{router::Route, store::use_auth};

#[component]
pub fn Sidebar(expanded: Signal<bool>, toggle: EventHandler<()>) -> Element {
    let auth_store = use_auth();
    let current_route = use_route::<Route>();

    // Helper function to determine if a route is active
    let is_active = |route: Route| -> bool {
        // Simply compare the route variants, ignoring any internal data
        std::mem::discriminant(&current_route) == std::mem::discriminant(&route)
    };

    rsx! {
        // Sidebar overlay (only visible when expanded on mobile)
        div {
            class: format_args!(
                "fixed inset-0 bg-black/50 z-30 transition-opacity duration-300 {} sm:hidden",
                if expanded() { "opacity-100" } else { "opacity-0 pointer-events-none" },
            ),
            onclick: move |_| toggle.call(()),
        }

        // Sidebar container
        aside {
            class: format_args!(
                "fixed inset-y-0 left-0 z-40 w-64 bg-zinc-200 dark:bg-zinc-800 shadow-lg transition-all duration-300 transform {} sm:translate-x-0",
                if expanded() { "translate-x-0" } else { "-translate-x-full" },
            ),
            // Sidebar header
            div { class: "flex h-16 items-center justify-between border-b border-zinc-300 dark:border-zinc-700 px-4 transition-colors duration-300",
                div { class: "flex items-center space-x-2",
                    img {
                        class: "h-8 w-8",
                        src: asset!("/assets/logo.png"),
                        alt: "Logo",
                    }
                    h1 { class: "text-lg font-bold text-zinc-800 dark:text-white transition-colors duration-300", "Ruxlog Admin" }
                }
                // Close sidebar button (mobile only)
                button {
                    class: "rounded-md p-2 text-zinc-500 dark:text-zinc-400 hover:bg-zinc-300 hover:text-zinc-800 dark:hover:bg-zinc-700 dark:hover:text-white transition-colors duration-200 sm:hidden",
                    onclick: move |_| toggle.call(()),
                    "×"
                }
            }

            // Sidebar navigation
            nav { class: "mt-4 px-2",
                // Navigation items
                div { class: "space-y-1",
                    Link {
                        class: format_args!(
                            "flex items-center rounded-lg px-3 py-2 text-sm font-medium {} transition-colors duration-200",
                            if is_active(Route::HomeScreen {}) {
                                "bg-zinc-300 text-zinc-800 dark:bg-zinc-700 dark:text-white"
                            } else {
                                "text-zinc-600 dark:text-zinc-300 hover:bg-zinc-300 hover:text-zinc-800 dark:hover:bg-zinc-700 dark:hover:text-white"
                            },
                        ),
                        to: Route::HomeScreen {},
                        Icon { icon: LdHome, width: 18, height: 18 }
                        span { class: "ml-3", "Dashboard" }
                    }

                    Link {
                        class: format_args!(
                            "flex items-center rounded-lg px-3 py-2 text-sm font-medium {} transition-colors duration-200",
                            if is_active(Route::AddBlogScreen {}) {
                                "bg-zinc-300 text-zinc-800 dark:bg-zinc-700 dark:text-white"
                            } else {
                                "text-zinc-600 dark:text-zinc-300 hover:bg-zinc-300 hover:text-zinc-800 dark:hover:bg-zinc-700 dark:hover:text-white"
                            },
                        ),
                        to: Route::AddBlogScreen {},
                        Icon { icon: LdFileText, width: 18, height: 18 }
                        span { class: "ml-3", "Posts" }
                    }

                    Link {
                        class: format_args!(
                            "flex items-center rounded-lg px-3 py-2 text-sm font-medium {} transition-colors duration-200",
                            if is_active(Route::AddCategoryScreen {}) {
                                "bg-zinc-300 text-zinc-800 dark:bg-zinc-700 dark:text-white"
                            } else {
                                "text-zinc-600 dark:text-zinc-300 hover:bg-zinc-300 hover:text-zinc-800 dark:hover:bg-zinc-700 dark:hover:text-white"
                            },
                        ),
                        to: Route::AddCategoryScreen {},
                        Icon { icon: LdFolder, width: 18, height: 18 }
                        span { class: "ml-3", "Categories" }
                    }

                    Link {
                        class: format_args!(
                            "flex items-center rounded-lg px-3 py-2 text-sm font-medium {} transition-colors duration-200",
                            if is_active(Route::AddTagScreen {}) {
                                "bg-zinc-300 text-zinc-800 dark:bg-zinc-700 dark:text-white"
                            } else {
                                "text-zinc-600 dark:text-zinc-300 hover:bg-zinc-300 hover:text-zinc-800 dark:hover:bg-zinc-700 dark:hover:text-white"
                            },
                        ),
                        to: Route::AddTagScreen {},
                        Icon { icon: LdTag, width: 18, height: 18 }
                        span { class: "ml-3", "Tags" }
                    }

                    Link {
                        class: format_args!(
                            "flex items-center rounded-lg px-3 py-2 text-sm font-medium {} transition-colors duration-200",
                            if is_active(Route::AddUserScreen {}) {
                                "bg-zinc-300 text-zinc-800 dark:bg-zinc-700 dark:text-white"
                            } else {
                                "text-zinc-600 dark:text-zinc-300 hover:bg-zinc-300 hover:text-zinc-800 dark:hover:bg-zinc-700 dark:hover:text-white"
                            },
                        ),
                        to: Route::AddUserScreen {},
                        Icon { icon: LdUser, width: 18, height: 18 }
                        span { class: "ml-3", "Users" }
                    }
                }
            }
            // Sidebar footer with logout button
            div { class: "absolute bottom-0 left-0 right-0 border-t border-zinc-300 dark:border-zinc-700 p-4 transition-colors duration-300",
                button {
                    class: "flex w-full items-center rounded-lg px-3 py-2 text-sm font-medium text-zinc-600 dark:text-zinc-300 hover:bg-zinc-300 hover:text-zinc-800 dark:hover:bg-zinc-700 dark:hover:text-white transition-colors duration-200",
                    onclick: move |_| {
                        spawn(async move {
                            auth_store.logout().await;
                        });
                    },
                    Icon { icon: LdLogOut, width: 18, height: 18 }
                    span { class: "ml-3", "Logout" }
                }
            }
        }
    }
}