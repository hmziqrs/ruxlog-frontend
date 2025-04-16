use dioxus::prelude::*;
use hmziq_dioxus_free_icons::{Icon, IconShape};
// Import the specific lucide icons we need
use hmziq_dioxus_free_icons::icons::ld_icons::{
    LdMenu, LdBell, LdSearch, LdSettings, LdChevronLeft, LdChevronRight
};

use crate::{router::Route, store::use_auth, components::Sidebar};

#[component]
pub fn NavBarContainer() -> Element {
    let auth_store = use_auth();
    let auth_user = auth_store.user.read();
    let mut sidebar_open = use_signal(|| false); // Default to open on desktop
    
    // Toggle sidebar handler
    let toggle_sidebar = move |_: MouseEvent| {
        sidebar_open.toggle();
    };

    if auth_user.is_none() {
        return rsx! {
            Outlet::<Route> {}
        }
    }

    rsx! {
        // Layout wrapper
        div { class: "min-h-screen bg-zinc-900",
            // Sidebar component (fixed position)
            Sidebar {
                expanded: sidebar_open,
                toggle: move |_| sidebar_open.toggle(),
            }

            // Main content area (with margin for sidebar on desktop)
            div {
                class: format_args!(
                    "transition-all duration-300 ease-in-out {}",
                    if *sidebar_open.read() { "sm:ml-64" } else { "ml-0" },
                ),
                // Top navigation bar
                header { class: "bg-zinc-800 shadow",
                    div { class: "flex h-16 items-center justify-between px-4",
                        div { class: "flex items-center",
                            // Mobile menu button
                            button {
                                class: "rounded-md p-2 text-gray-400 hover:bg-zinc-700 hover:text-white sm:hidden",
                                onclick: move |_| sidebar_open.set(true),
                                Icon { icon: LdMenu, width: 20, height: 20 }
                            }
                            // Desktop sidebar toggle button
                            button {
                                class: "hidden rounded-md p-2 text-gray-400 hover:bg-zinc-700 hover:text-white sm:flex",
                                onclick: toggle_sidebar,
                                if *sidebar_open.read() {
                                    Icon { icon: LdChevronLeft }
                                } else {
                                    Icon {
                                        icon: LdChevronRight,
                                        width: 20,
                                        height: 20,
                                    }
                                }
                            }
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
                                Icon { icon: LdSettings, width: 20, height: 20 }
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
                main { class: "p-6 bg-zinc-900", Outlet::<Route> {} }
            }
        }
    }
}
