use dioxus::prelude::*;

use crate::store::use_tag;

#[component]
pub fn TagsListScreen() -> Element {
    let tags = use_tag();
    rsx! {
        div { "Tag List (placeholder)" }
    }
}
