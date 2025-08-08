use dioxus::prelude::*;

use crate::containers::CategoryFormContainer;

#[component]
pub fn CategoriesAddScreen() -> Element {
    rsx! {
        CategoryFormContainer {}
    }
}
