use dioxus::prelude::*;

use crate::containers::BlogFormContainer;

#[component]
pub fn PostsEditScreen(id: i32) -> Element {
    rsx! {
        BlogFormContainer { post_id: Some(id) }
    }
}
