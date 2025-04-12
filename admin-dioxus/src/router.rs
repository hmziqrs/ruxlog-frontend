use dioxus::prelude::*;

use crate::screens::HomeScreen;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // #[layout(Navbar)]
    #[route("/")]
    HomeScreen {},
}
