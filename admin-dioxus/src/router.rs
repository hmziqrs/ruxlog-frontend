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
use crate::screens::AnalyticsScreen;
use crate::screens::BlogListScreen;
use crate::screens::CategoryListScreen;
use crate::screens::TagListScreen;
use crate::screens::UserListScreen;

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
    #[route("/blog/list")]
    BlogListScreen {},

    #[route("/category/new")]
    AddCategoryScreen {},
    #[route("/category/list")]
    CategoryListScreen {},

    #[route("/tag/new")]
    AddTagScreen {},
    #[route("/tag/list")]
    TagListScreen {},

    #[route("/user/new")]
    AddUserScreen {},
    #[route("/user/list")]
    UserListScreen {},

    #[route("/analytics")]
    AnalyticsScreen {},
}
pub static OPEN_ROUTES: LazyLock<Vec<Route>> = LazyLock::new(|| vec![
    Route::LoginScreen {},
]);
