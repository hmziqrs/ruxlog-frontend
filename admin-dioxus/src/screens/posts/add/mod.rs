use dioxus::prelude::*;

use crate::containers::BlogFormContainer;

#[component]
pub fn PostsAddScreen() -> Element {
    rsx! {
        BlogFormContainer {}
    }
}
