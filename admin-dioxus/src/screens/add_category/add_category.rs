use dioxus::prelude::*;

use crate::containers::CategoryFormContainer;

#[component]
pub fn AddCategoryScreen() -> Element {
    rsx! {
        CategoryFormContainer {}
    }
}