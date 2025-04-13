use dioxus::prelude::*;

use crate::containers::TagFormContainer;

#[component]
pub fn AddTagScreen() -> Element {
    rsx! {
        TagFormContainer {}
    }
}