use dioxus::logger::tracing;
use dioxus::prelude::*;
use hmziq_dioxus_free_icons::Icon;
// Import the specific lucide icons we need
use hmziq_dioxus_free_icons::icons::ld_icons::{
    LdBell, LdChevronLeft, LdChevronRight, LdMenu, LdMoon, LdSearch, LdSettings, LdSun,
};

use crate::config::DarkMode;
use crate::{components::Sidebar, router::Route, store::use_auth};

#[component]
pub fn NavBarContainer() -> Element {
    let auth_store = use_auth();
    let auth_user = auth_store.user.read();
    let mut sidebar_open = use_signal(|| false);
    let mut dark_theme = use_context_provider(|| Signal::new(DarkMode(true)));

    use_effect(move || {
        spawn(async move {
            let is_dark =
                document::eval("return document.documentElement.classList.contains('dark');")
                    .await
                    .unwrap()
                    .to_string();
            tracing::info!("Dark mode is: {}", is_dark);
            dark_theme.set(DarkMode(is_dark.parse::<bool>().unwrap_or(false)));
        });
    });

    // Toggle dark mode
    let toggle_dark_mode = move |_: MouseEvent| {
        dark_theme.write().toggle();
        spawn(async move {
            _ = document::eval("document.documentElement.classList.toggle('dark');").await;
            _ = document::eval("localStorage.setItem('theme', document.documentElement.classList.contains('dark') ? 'dark' : 'light');").await;
        });
    };

    // Toggle sidebar handler
    let toggle_sidebar = move |_: MouseEvent| {
        sidebar_open.toggle();
    };

    if auth_user.is_none() {
        return rsx! {
            Outlet::<Route> {}
        };
    }

    rsx! {
        // Sidebar component (fixed position)
        Sidebar { expanded: sidebar_open, toggle: move |_| sidebar_open.toggle() }

        // Main content area (with margin for sidebar on desktop)
        header { class: "bg-zinc-200 dark:bg-zinc-900/60 backdrop-blur-sm shadow transition-colors duration-300 sticky top-0 z-10",
            div { class: "flex h-16 items-center justify-between px-4",
                div { class: "flex items-center",
                    // Mobile menu button
                    button {
                        class: "rounded-md p-2 text-zinc-600 dark:text-zinc-400 hover:bg-zinc-300 hover:text-zinc-800 dark:hover:bg-zinc-700 dark:hover:text-white transition-colors duration-200 sm:hidden",
                        onclick: move |_| sidebar_open.set(true),
                        div { class: "w-4 h-4",
                            Icon { icon: LdMenu }
                        }
                    }
                    // Desktop sidebar toggle button
                    button {
                        class: "hidden rounded-md p-2 text-zinc-600 dark:text-zinc-400 hover:bg-zinc-300 hover:text-zinc-800 dark:hover:bg-zinc-700 dark:hover:text-white transition-colors duration-200 sm:flex",
                        onclick: toggle_sidebar,
                        div { class: "w-4 h-4",
                            Icon { icon: LdMenu }
                        }
                    }
                }

                // Search bar (hidden on small screens)
                div { class: "hidden md:block flex-1 px-4 lg:px-6",
                    div { class: "relative max-w-md",
                        div { class: "pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3",
                            div { class: "w-4 h-4",
                                Icon {
                                    icon: LdSearch,
                                    class: "text-zinc-500 dark:text-zinc-400",
                                }
                            }
                        }
                        input {
                            class: "block w-full rounded-lg border  bg-zinc-50 dark:bg-zinc-700 py-2 pl-10 pr-3 text-sm text-zinc-700 dark:text-zinc-200 placeholder:text-zinc-500 dark:placeholder:text-zinc-400 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 transition-colors duration-200",
                            placeholder: "Search...",
                            r#type: "text",
                        }
                    }
                }

                // Right side nav items
                div { class: "flex items-center space-x-4",
                    button { class: "rounded-full p-1 text-zinc-600 dark:text-zinc-400 hover:bg-zinc-300 hover:text-zinc-800 dark:hover:bg-zinc-700 dark:hover:text-white transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-200 dark:focus:ring-offset-zinc-800",
                        div { class: "w-4 h-4",
                            Icon { icon: LdBell }
                        }
                    }
                    button {
                        onclick: toggle_dark_mode,
                        class: "rounded-full p-1 text-zinc-600 dark:text-zinc-400 hover:bg-zinc-300 hover:text-zinc-800 dark:hover:bg-zinc-700 dark:hover:text-white transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-zinc-200 dark:focus:ring-offset-zinc-800",
                        div { class: "w-4 h-4",
                            if (*dark_theme.read()).0 {
                                Icon { icon: LdSun }
                            } else {
                                Icon { icon: LdMoon }
                            }
                        }
                    }
                    // User profile
                    div { class: "relative ml-3",
                        div { class: "flex rounded-full bg-zinc-200 dark:bg-zinc-700 text-sm focus:outline-none transition-colors duration-200",
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
        Outlet::<Route> {}
    }
}
