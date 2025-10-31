use dioxus::prelude::*;
use std::sync::LazyLock;

use crate::containers::AuthGuardContainer;
use crate::containers::NavBarContainer;
use crate::screens::AnalyticsScreen;
use crate::screens::CategoriesAddScreen;
use crate::screens::CategoriesEditScreen;
use crate::screens::CategoriesListScreen;
use crate::screens::HomeScreen;
use crate::screens::LoginScreen;
use crate::screens::MediaListScreen;
use crate::screens::MediaUploadScreen;
use crate::screens::PostsAddScreen;
use crate::screens::PostsListScreen;
use crate::screens::SonnerDemoScreen;
use crate::screens::TagsAddScreen;
use crate::screens::TagsEditScreen;
use crate::screens::TagsListScreen;
use crate::screens::UsersAddScreen;
use crate::screens::UsersEditScreen;
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
    #[route("/categories/:id/edit")]
    CategoriesEditScreen { id: i32 },

    #[route("/tags/new")]
    TagsAddScreen {},
    #[route("/tags/:id/edit")]
    TagsEditScreen { id: i32 },
    #[route("/tags/list")]
    TagsListScreen {},

    #[route("/media/upload")]
    MediaUploadScreen {},
    #[route("/media/list")]
    MediaListScreen {},

    #[route("/users/new")]
    UsersAddScreen {},
    #[route("/users/:id/edit")]
    UsersEditScreen { id: i32 },
    #[route("/users/list")]
    UsersListScreen {},

    #[route("/analytics")]
    AnalyticsScreen {},

    #[route("/demo/sonner")]
    SonnerDemoScreen {},
}
pub static OPEN_ROUTES: LazyLock<Vec<Route>> = LazyLock::new(|| vec![Route::LoginScreen {}]);
