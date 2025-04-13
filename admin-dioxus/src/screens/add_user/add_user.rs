use dioxus::prelude::*;

use crate::containers::UserFormContainer;

#[component]
pub fn AddUserScreen() -> Element {
    rsx! {
        UserFormContainer {}
    }
}