use dioxus::prelude::*;

use crate::containers::TagFormContainer;

#[component]
pub fn TagsAddScreen() -> Element {
    rsx! {
        TagFormContainer {}
    }
}
