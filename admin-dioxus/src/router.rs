use dioxus::prelude::*;

use crate::screens::HomeScreen;
use crate::screens::LoginScreen;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // #[layout(Navbar)]
    #[route("/")]
    HomeScreen {},

    #[route("/login")]
    LoginScreen {},
}
