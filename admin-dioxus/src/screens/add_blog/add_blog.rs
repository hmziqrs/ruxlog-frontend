use dioxus::prelude::*;

use crate::containers::BlogFormContainer;

#[component]
pub fn AddBlogScreen() -> Element {
    
    rsx! {
        BlogFormContainer {}
    }
}