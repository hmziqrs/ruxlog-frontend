use dioxus::prelude::*;
use std::sync::LazyLock;

use crate::containers::AuthGuard;
use crate::screens::HomeScreen;
use crate::screens::LoginScreen;
use crate::screens::AddBlogScreen;
use crate::screens::AddCategoryScreen;
use crate::screens::AddTagScreen;
use crate::screens::AddUserScreen;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AuthGuard)]
    #[layout(NavBar)]
    #[route("/")]
    HomeScreen {},

    #[route("/login")]
    LoginScreen {},
    
    #[route("/blog/new")]
    AddBlogScreen {},

    #[route("/category/new")]
    AddCategoryScreen {},

    #[route("/tag/new")]
    AddTagScreen {},

    #[route("/user/new")]
    AddUserScreen {},
}
pub static OPEN_ROUTES: LazyLock<Vec<Route>> = LazyLock::new(|| vec![
    Route::LoginScreen {},
]);

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
                    Link { class: "font-medium", to: Route::AddBlogScreen {}, "New Blog Post" }
                }
                li { class: "hover:text-zinc-300 transition-colors duration-200",
                    Link { class: "font-medium", to: Route::AddCategoryScreen {}, "New Category" }
                }
                li { class: "hover:text-zinc-300 transition-colors duration-200",
                    Link { class: "font-medium", to: Route::AddTagScreen {}, "New Tag" }
                }
                li { class: "hover:text-zinc-300 transition-colors duration-200",
                    Link { class: "font-medium", to: Route::AddUserScreen {}, "New User" }
                }
            }
        }
        Outlet::<Route> {}
    }
}

