use dioxus::prelude::*;

use crate::router::Route;
use crate::ui::shadcn::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};

#[derive(Props, PartialEq, Clone)]
pub struct PageHeaderProps {
    pub title: String,
    pub description: String,
    #[props(optional)]
    pub actions: Option<Element>, // right side actions (e.g., Create button)
    #[props(optional)]
    pub class: Option<String>, // optional class to tweak paddings
    #[props(default = false)]
    pub embedded: bool, // when true, render without outer wrapper and container
}

#[component]
pub fn PageHeader(props: PageHeaderProps) -> Element {
    let nav = use_navigator();
    let current_route = use_route::<Route>();

    // Derive module name and suffix to match URL paths
    let (module, add_suffix): (String, Option<String>) = match current_route {
        Route::PostsAddScreen {} => ("posts".to_string(), Some("add".to_string())),
        Route::PostsEditScreen { id } => ("posts".to_string(), Some(format!("{}/edit", id))),
        Route::PostsViewScreen { id } => ("posts".to_string(), Some(id.to_string())),
        Route::PostsListScreen {} => ("posts".to_string(), None),
        Route::CategoriesAddScreen {} => ("categories".to_string(), Some("add".to_string())),
        Route::CategoriesListScreen {} => ("categories".to_string(), None),
        Route::CategoriesEditScreen { id } => {
            ("categories".to_string(), Some(format!("{}/edit", id)))
        }
        Route::TagsAddScreen {} => ("tags".to_string(), Some("add".to_string())),
        Route::TagsEditScreen { id } => ("tags".to_string(), Some(format!("{}/edit", id))),
        Route::TagsListScreen {} => ("tags".to_string(), None),
        Route::MediaUploadScreen {} => ("media".to_string(), Some("upload".to_string())),
        Route::MediaListScreen {} => ("media".to_string(), None),
        Route::UsersAddScreen {} => ("users".to_string(), Some("add".to_string())),
        Route::UsersEditScreen { id } => ("users".to_string(), Some(format!("{}/edit", id))),
        Route::UsersListScreen {} => ("users".to_string(), None),
        Route::AnalyticsScreen {} => ("analytics".to_string(), None),
        Route::SonnerDemoScreen {} => ("demo".to_string(), Some("sonner".to_string())),
        Route::HomeScreen {} | Route::LoginScreen {} => ("".to_string(), None),
    };

    // Resolve the list route for the current module, if applicable
    let list_route_for_module = |m: &str| -> Option<Route> {
        match m {
            // LIST_ROUTES_START (auto-generated)
            "posts" => Some(Route::PostsListScreen {}),
            "categories" => Some(Route::CategoriesListScreen {}),
            "tags" => Some(Route::TagsListScreen {}),
            "media" => Some(Route::MediaListScreen {}),
            "users" => Some(Route::UsersListScreen {}),
            // LIST_ROUTES_END
            _ => None,
        }
    };

    let container_class = props
        .class
        .clone()
        .unwrap_or_else(|| "container mx-auto px-4 py-6 md:py-8".to_string());

    if props.embedded {
        rsx! {
            // Breadcrumb
            Breadcrumb {
                BreadcrumbList {
                    // Dashboard root
                    BreadcrumbItem {
                        BreadcrumbLink {
                            onclick: Some(Callback::new(move |_| { nav.push(Route::HomeScreen {}); })),
                            "Dashboard"
                        }
                    }
                    BreadcrumbSeparator {}

                    // Middle crumbs and page
                    match add_suffix {
                        None => rsx!{ if !module.is_empty() { BreadcrumbItem { BreadcrumbPage { "{module}" } } } },
                        Some(suffix) => {
                            rsx!{
                                BreadcrumbItem {
                                    match list_route_for_module(&module) {
                                        Some(list_route) => rsx!{ BreadcrumbLink { onclick: Some(Callback::new(move |_| { nav.push(list_route.clone()); })), "{module}" } },
                                        None => rsx!{ BreadcrumbPage { "{module}" } }
                                    }
                                }
                                BreadcrumbSeparator {}
                                BreadcrumbItem { BreadcrumbPage { "{suffix}" } }
                            }
                        }
                    }
                }
            }

            // Header row
            div { class: "mt-6 flex flex-col items-start justify-between gap-6 md:flex-row md:items-center",
                div { class: "space-y-2",
                    h1 { class: "text-3xl md:text-4xl font-bold tracking-tight", "{props.title}" }
                    p { class: "text-sm md:text-base text-zinc-600 dark:text-zinc-400", "{props.description}" }
                }
                div { class: "flex items-center gap-2",
                    if let Some(actions) = props.actions.clone() { {actions} }
                }
            }
        }
    } else {
        rsx! {
            // Top region with breadcrumb and header
            div { class: "border-b border-border/60 bg-transparent transition-colors duration-300",
                div { class: container_class,
                    // Breadcrumb
                    Breadcrumb {
                        BreadcrumbList {
                            // Dashboard root
                            BreadcrumbItem {
                                BreadcrumbLink {
                                    // href can be omitted; we handle nav in onclick
                                    onclick: Some(Callback::new(move |_| { nav.push(Route::HomeScreen {}); })),
                                    "Dashboard"
                                }
                            }
                            BreadcrumbSeparator {}

                            // Middle crumbs and page
                            match add_suffix {
                                // List-like screens or single-level pages
                                None => rsx!{ if !module.is_empty() { BreadcrumbItem { BreadcrumbPage { "{module}" } } } },
                                // Add/new screens -> Dashboard / Module / New
                                Some(suffix) => {
                                    rsx!{
                                        // Module link back to list
                                        BreadcrumbItem {
                                            match list_route_for_module(&module) {
                                                Some(list_route) => rsx!{
                                                    BreadcrumbLink { onclick: Some(Callback::new(move |_| { nav.push(list_route.clone()); })), "{module}" }
                                                },
                                                None => rsx!{ BreadcrumbPage { "{module}" } }
                                            }
                                        }
                                        BreadcrumbSeparator {}
                                        BreadcrumbItem { BreadcrumbPage { "{suffix}" } }
                                    }
                                }
                            }
                        }
                    }

                    // Header row
                    div { class: "mt-6 flex flex-col items-start justify-between gap-6 md:flex-row md:items-center",
                        div { class: "space-y-2",
                            h1 { class: "text-3xl md:text-4xl font-bold tracking-tight", "{props.title}" }
                            p { class: "text-sm md:text-base text-zinc-600 dark:text-zinc-400", "{props.description}" }
                        }
                        div { class: "flex items-center gap-2",
                            if let Some(actions) = props.actions.clone() { {actions} }
                        }
                    }
                }
            }
        }
    }
}
