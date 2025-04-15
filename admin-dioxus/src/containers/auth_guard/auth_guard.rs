use dioxus::prelude::*;

use crate::{router::Route, store::use_auth};

#[component]
pub fn AuthGuard() -> Element {
    let mut render_blocked = use_signal(|| true);
    let auth_store = use_auth();
    let init_status = auth_store.init_status.read();
    let init_status_hook = init_status.clone();
    let init_status_effect = init_status.clone();
    let user_effect = auth_store.user.read();
    let route: Route = use_route();
    let mut nav = use_navigator();
    

    use_effect(move || {
        if init_status_hook.is_init() {
            spawn(async move {
                auth_store.init().await;
            });
        }
    });

    use_effect(move || {
        if init_status_effect.is_success() {
            if let Some(user) = &*user_effect {
                if user.role != "admin" {
                    render_blocked.set(true);
                    return;
                }
            }
        }
    });



    if init_status.is_failed() {
        return rsx! {
            div { "Error: Failed to initialize user" }
        };
    }

    if *render_blocked.read() {
        return rsx! {
            div { "Loading..." }
        };
    }

    rsx! {

        Outlet::<Route> {}
    }
}
