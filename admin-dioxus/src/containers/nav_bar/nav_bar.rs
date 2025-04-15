use dioxus::prelude::*;

use crate::{router::Route, store::use_auth};

#[component]
pub fn NavBarContainer() -> Element {
    let auth_store = use_auth();
    let auth_user = auth_store.user.read();


    rsx! {
        if auth_user.is_some() {
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
                        Link {
                            class: "font-medium",
                            to: Route::AddCategoryScreen {},
                            "New Category"
                        }
                    }
                    li { class: "hover:text-zinc-300 transition-colors duration-200",
                        Link { class: "font-medium", to: Route::AddTagScreen {}, "New Tag" }
                    }
                    li { class: "hover:text-zinc-300 transition-colors duration-200",
                        Link { class: "font-medium", to: Route::AddUserScreen {}, "New User" }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
