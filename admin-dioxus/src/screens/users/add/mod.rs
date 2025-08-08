use dioxus::prelude::*;

use crate::containers::UserFormContainer;

#[component]
pub fn UsersAddScreen() -> Element {
    rsx! {
        UserFormContainer {}
    }
}
