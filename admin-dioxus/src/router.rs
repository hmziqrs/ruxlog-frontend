use dioxus::prelude::*;
use std::sync::LazyLock;

use crate::containers::AuthGuardContainer;
use crate::containers::NavBarContainer;
use crate::screens::HomeScreen;
use crate::screens::LoginScreen;
use crate::screens::AddBlogScreen;
use crate::screens::AddCategoryScreen;
use crate::screens::AddTagScreen;
use crate::screens::AddUserScreen;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AuthGuardContainer)]
    #[layout(NavBarContainer)]
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
