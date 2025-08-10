use dioxus::prelude::*;
use std::sync::LazyLock;

use crate::containers::AuthGuardContainer;
use crate::containers::NavBarContainer;
use crate::screens::AnalyticsScreen;
use crate::screens::CategoriesAddScreen;
use crate::screens::CategoriesListScreen;
use crate::screens::HomeScreen;
use crate::screens::LoginScreen;
use crate::screens::PostsAddScreen;
use crate::screens::PostsListScreen;
use crate::screens::TagsAddScreen;
use crate::screens::TagsListScreen;
use crate::screens::TagsEditScreen;
use crate::screens::UsersAddScreen;
use crate::screens::UsersListScreen;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AuthGuardContainer)]
    #[layout(NavBarContainer)]
    #[route("/")]
    HomeScreen {},

    #[route("/login")]
    LoginScreen {},

    #[route("/posts/add")]
    PostsAddScreen {},
    #[route("/posts/list")]
    PostsListScreen {},

    #[route("/categories/add")]
    CategoriesAddScreen {},
    #[route("/category/list")]
    CategoriesListScreen {},

    #[route("/tags/new")]
    TagsAddScreen {},
    #[route("/tags/:id/edit")]
    TagsEditScreen { id: i32 },
    #[route("/tags/list")]
    TagsListScreen {},

    #[route("/users/new")]
    UsersAddScreen {},
    #[route("/users/list")]
    UsersListScreen {},

    #[route("/analytics")]
    AnalyticsScreen {},
}
pub static OPEN_ROUTES: LazyLock<Vec<Route>> = LazyLock::new(|| vec![Route::LoginScreen {}]);
