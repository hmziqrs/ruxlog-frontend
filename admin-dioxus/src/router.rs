use dioxus::prelude::*;

use crate::screens::HomeScreen;
use crate::screens::LoginScreen;
use crate::screens::BlogScreen;
use crate::screens::CategoryScreen;
use crate::screens::TagScreen;
use crate::screens::UserScreen;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
    #[layout(Footer)]
    #[route("/")]
    HomeScreen {},

    #[route("/login")]
    LoginScreen {},
    
    #[route("/blog/new")]
    BlogScreen {},

    #[route("/category/new")]
    CategoryScreen {},

    #[route("/tag/new")]
    TagScreen {},

    #[route("/user/new")]
    UserScreen {},
}

#[component]
fn NavBar() -> Element {
    rsx! {
        div { class: "bg-zinc-800 text-white py-4 px-6 flex justify-between items-center shadow-lg",
            h1 { class: "text-2xl font-bold tracking-tight", "Admin Dioxus" }
            ul { class: "flex space-x-6",
                li { class: "hover:text-zinc-300 transition-colors duration-200",
                    Link { class: "font-medium", to: Route::HomeScreen {}, "Home" }
                }
                li { class: "hover:text-zinc-300 transition-colors duration-200",
                    Link { class: "font-medium", to: Route::LoginScreen {}, "Login" }
                }
                li { class: "hover:text-zinc-300 transition-colors duration-200",
                    Link { class: "font-medium", to: Route::BlogScreen {}, "New Blog Post" }
                }
                li { class: "hover:text-zinc-300 transition-colors duration-200",
                    Link { class: "font-medium", to: Route::CategoryScreen {}, "New Category" }
                }
                li { class: "hover:text-zinc-300 transition-colors duration-200",
                    Link { class: "font-medium", to: Route::TagScreen {}, "New Tag" }
                }
                li { class: "hover:text-zinc-300 transition-colors duration-200",
                    Link { class: "font-medium", to: Route::UserScreen {}, "New User" }
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        Outlet::<Route> {}
        div { class: "footer", "Copyright © 2025 Admin Dioxus" }
    }
}
