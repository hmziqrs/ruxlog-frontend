use dioxus::prelude::*;
use hmziq_dioxus_free_icons::{Icon, IconShape};
// Import the specific lucide icons we need
use hmziq_dioxus_free_icons::icons::ld_icons::{
    LdHome, LdFileText, LdFolder, LdTag, LdUser, LdLogOut
};

use crate::{router::Route, store::use_auth};

#[component]
pub fn NavBarContainer() -> Element {
    let auth_store = use_auth();
    let auth_user = auth_store.user.read();
    let current_route = use_route::<Route>();

    // Helper function to determine if a route is active
    let is_active = |route: Route| -> bool {
        // Simply compare the route variants, ignoring any internal data
        std::mem::discriminant(&current_route) == std::mem::discriminant(&route)
    };

    rsx! {
        if auth_user.is_some() {
            nav { class: "py-3 px-6 shadow-xl sticky",
                // Main navbar container
                div { class: "max-w-7xl mx-auto flex flex-col md:flex-row justify-between items-center",
                    // Logo section
                    div { class: "flex items-center space-x-2 mb-3 md:mb-0",
                        img {
                            class: "h-8 w-8",
                            src: asset!("/assets/logo.png"),
                            alt: "Logo",
                        }
                        h1 { class: "text-xl font-bold tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-zinc-100 to-zinc-400",
                            "Admin Dioxus"
                        }
                    }
                    // Navigation links
                    ul { class: "flex flex-wrap justify-center space-x-1 md:space-x-2",
                        li {
                            Link {
                                class: format_args!(
                                    "px-3 py-2 rounded-lg flex items-center space-x-1 transition-all duration-200 {} hover:bg-zinc-700",
                                    if is_active(Route::HomeScreen {}) {
                                        "bg-zinc-700 text-white"
                                    } else {
                                        "text-zinc-300"
                                    },
                                ),
                                to: Route::HomeScreen {},
                                Icon { icon: LdHome, width: 18, height: 18 }
                                span { class: "ml-1", "Home" }
                            }
                        }
                        li {
                            Link {
                                class: format_args!(
                                    "px-3 py-2 rounded-lg flex items-center space-x-1 transition-all duration-200 {} hover:bg-zinc-700",
                                    if is_active(Route::AddBlogScreen {}) {
                                        "bg-zinc-700 text-white"
                                    } else {
                                        "text-zinc-300"
                                    },
                                ),
                                to: Route::AddBlogScreen {},
                                Icon { icon: LdFileText, width: 18, height: 18 }
                                span { class: "ml-1", "New Post" }
                            }
                        }
                        li {
                            Link {
                                class: format_args!(
                                    "px-3 py-2 rounded-lg flex items-center space-x-1 transition-all duration-200 {} hover:bg-zinc-700",
                                    if is_active(Route::AddCategoryScreen {}) {
                                        "bg-zinc-700 text-white"
                                    } else {
                                        "text-zinc-300"
                                    },
                                ),
                                to: Route::AddCategoryScreen {},
                                Icon { icon: LdFolder, width: 18, height: 18 }
                                span { class: "ml-1", "Category" }
                            }
                        }
                        li {
                            Link {
                                class: format_args!(
                                    "px-3 py-2 rounded-lg flex items-center space-x-1 transition-all duration-200 {} hover:bg-zinc-700",
                                    if is_active(Route::AddTagScreen {}) {
                                        "bg-zinc-700 text-white"
                                    } else {
                                        "text-zinc-300"
                                    },
                                ),
                                to: Route::AddTagScreen {},
                                Icon { icon: LdTag, width: 18, height: 18 }
                                span { class: "ml-1", "Tag" }
                            }
                        }
                        li {
                            Link {
                                class: format_args!(
                                    "px-3 py-2 rounded-lg flex items-center space-x-1 transition-all duration-200 {} hover:bg-zinc-700",
                                    if is_active(Route::AddUserScreen {}) {
                                        "bg-zinc-700 text-white"
                                    } else {
                                        "text-zinc-300"
                                    },
                                ),
                                to: Route::AddUserScreen {},
                                Icon { icon: LdUser, width: 18, height: 18 }
                                span { class: "ml-1", "User" }
                            }
                        }
                        // User profile and logout
                        li {
                            button {
                                class: "px-3 py-2 rounded-lg flex items-center space-x-1 text-zinc-300 hover:bg-zinc-700 transition-all duration-200",
                                onclick: move |_| {
                                    auth_store.logout();
                                },
                                Icon { icon: LdLogOut, width: 18, height: 18 }
                                span { class: "ml-1", "Logout" }
                            }
                        }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
