use dioxus::prelude::*;

use crate::{router::Route, store::use_auth};

#[component]
pub fn AuthGuard() -> Element {
    let auth_store = use_auth();
    let init_status = auth_store.init_status.read();
    let init_status_hook = init_status.clone();

    use_effect(move || {
        if init_status_hook.is_init() {
            spawn(async move {
                auth_store.init().await;
            });
        }
    });

    if init_status.is_loading() {
        return rsx! {
            div { "Loading..." }
        };
    }

    rsx! {

        Outlet::<Route> {}
    }
}
