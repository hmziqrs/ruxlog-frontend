use dioxus::prelude::*;
use hmziq_dioxus_free_icons::icons::ld_icons::{
    LdAreaChart, LdFileText, LdFolder, LdHome, LdImage, LdLogOut, LdPlus, LdTag, LdUser,
};
use hmziq_dioxus_free_icons::Icon;

use crate::{router::Route, store::use_auth};

#[derive(Props, PartialEq, Clone)]
pub struct SidebarModuleLinkProps {
    pub main_route: Route,
    #[props(optional)]
    pub add_route: Option<Route>,
    pub icon: Element,
    pub label: String,
    pub is_active: bool,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn SidebarModuleLink(props: SidebarModuleLinkProps) -> Element {
    let nav = use_navigator();
    rsx! {
        div { class: "flex flex-row w-full",
            div {
                class: format_args!(
                    "flex items-center flex-1 rounded-md px-3 py-2 text-sm font-medium {} transition-colors duration-200",
                    if props.is_active {
                        "bg-zinc-300 text-zinc-800 dark:bg-zinc-700 dark:text-white"
                    } else {
                        "text-zinc-600 dark:text-zinc-300 hover:bg-zinc-300 hover:text-zinc-800 dark:hover:bg-zinc-900/90 dark:hover:text-white"
                    },
                ),
                onclick: move |_| {
                    nav.push(props.main_route.clone());
                    props.on_close.call(());
                },
                div { class: "h-4 w-4", {props.icon} }
                span { class: "ml-3", "{props.label}" }
                div { class: "flex-1" }
                if let Some(add_route) = props.add_route {
                    div {
                        class: "self-end rounded hover:bg-zinc-300 dark:hover:bg-zinc-800/90 p-1",
                        onclick: move |e| {
                            e.stop_propagation();
                            nav.push(add_route.clone());
                            props.on_close.call(());
                        },
                        aria_label: format!("Add {}", props.label),
                        div { class: "w-5 h-5",
                            Icon { icon: LdPlus }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Sidebar(expanded: Signal<bool>, toggle: EventHandler<()>) -> Element {
    let auth_store = use_auth();
    let current_route = use_route::<Route>();

    let is_active = |route: Route| -> bool {
        std::mem::discriminant(&current_route) == std::mem::discriminant(&route)
    };

    rsx! {
        div {
            class: format!(
                "fixed inset-0 bg-black/30 z-30 transition-opacity duration-300 {} backdrop-blur-xs",
                if expanded() { "opacity-100" } else { "opacity-0 pointer-events-none" },
            ),
            onclick: move |_| toggle.call(()),
        }

        aside {
            class: format!(
                "fixed inset-y-0 left-0 z-40 w-64 bg-zinc-200 dark:bg-zinc-950/95 transition-all duration-300 transform border-r border-zinc-300 dark:border-zinc-800 {}",
                if expanded() { "translate-x-0" } else { "-translate-x-full" },
            ),
            div { class: "flex h-16 items-center justify-between border-b border-zinc-300 dark:border-zinc-800 px-4 transition-colors duration-300",
                div { class: "flex items-center space-x-2",
                    img {
                        class: "h-8 w-8",
                        src: asset!("/assets/logo.png"),
                        alt: "Logo",
                    }
                    h1 { class: "text-lg font-bold text-zinc-800 dark:text-white transition-colors duration-300",
                        "Ruxlog Admin"
                    }
                }
                button {
                    class: "rounded-md p-2 text-zinc-500 dark:text-zinc-400 hover:bg-zinc-300 hover:text-zinc-800 dark:hover:bg-zinc-900/90 dark:hover:text-white transition-colors duration-200 sm:hidden",
                    onclick: move |_| toggle.call(()),
                    "×"
                }
            }

            nav { class: "px-2",
                div { class: "",
                    SidebarModuleLink {
                        main_route: Route::HomeScreen {},
                        icon: rsx! {
                            Icon { icon: LdHome }
                        },
                        label: "Dashboard",
                        is_active: is_active(Route::HomeScreen {}),
                        on_close: move |_| toggle.call(()),
                    }
                    SidebarModuleLink {
                        main_route: Route::PostsListScreen {},
                        add_route: Some(Route::PostsAddScreen {}),
                        icon: rsx! {
                            Icon { icon: LdFileText }
                        },
                        label: "Posts",
                        is_active: is_active(Route::PostsListScreen {}),
                        on_close: move |_| toggle.call(()),
                    }
                    SidebarModuleLink {
                        main_route: Route::CategoriesListScreen {},
                        add_route: Some(Route::CategoriesAddScreen {}),
                        icon: rsx! {
                            Icon { icon: LdFolder }
                        },
                        label: "Categories",
                        is_active: is_active(Route::CategoriesListScreen {}),
                        on_close: move |_| toggle.call(()),
                    }
                    SidebarModuleLink {
                        main_route: Route::TagsListScreen {},
                        add_route: Some(Route::TagsAddScreen {}),
                        icon: rsx! {
                            Icon { icon: LdTag }
                        },
                        label: "Tags",
                        is_active: is_active(Route::TagsListScreen {}),
                        on_close: move |_| toggle.call(()),
                    }
                    SidebarModuleLink {
                        main_route: Route::MediaListScreen {},
                        add_route: Some(Route::MediaUploadScreen {}),
                        icon: rsx! {
                            Icon { icon: LdImage }
                        },
                        label: "Media",
                        is_active: is_active(Route::MediaListScreen {}),
                        on_close: move |_| toggle.call(()),
                    }
                    SidebarModuleLink {
                        main_route: Route::UsersListScreen {},
                        add_route: Some(Route::UsersAddScreen {}),
                        icon: rsx! {
                            Icon { icon: LdUser }
                        },
                        label: "Users",
                        is_active: is_active(Route::UsersListScreen {}),
                        on_close: move |_| toggle.call(()),
                    }
                    SidebarModuleLink {
                        main_route: Route::AnalyticsScreen {},
                        icon: rsx! {
                            Icon { icon: LdAreaChart }
                        },
                        label: "Analytics",
                        is_active: is_active(Route::AnalyticsScreen {}),
                        on_close: move |_| toggle.call(()),
                    }
                }
            }
            div { class: "absolute bottom-0 left-0 right-0 border-t border-zinc-300 dark:border-zinc-800 p-4 transition-colors duration-300",
                button {
                    class: "flex w-full items-center flex-1 rounded-md px-3 py-2 text-sm font-medium text-zinc-600 dark:text-zinc-300 hover:bg-zinc-300 hover:text-zinc-800 dark:hover:bg-zinc-900/90 dark:hover:text-white transition-colors duration-200",
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
