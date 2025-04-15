use dioxus::{logger::tracing, prelude::*};

use crate::{router::{Route, OPEN_ROUTES}, store::use_auth};

#[component]
pub fn AuthGuard() -> Element {
    let render_blocked = use_signal(|| true);

    let auth_store = use_auth();
    let nav = use_navigator();
    let route: Route = use_route();

    let init_status = auth_store.init_status.read();
    let init_status_hook  = init_status.clone();
    tracing::info!("AuthGuard: init_status = {:?}", init_status);

    // Clone for closure use
    // let route_for_effect = route.clone();
    // let nav_for_effect = nav.clone();
    // let auth_store_for_effect = auth_store;
    // let render_blocked_for_effect = render_blocked.clone();

    use_effect(move || {
        if init_status_hook.is_init() {
            spawn(async move {
                auth_store.init().await;
            });
        }
    });

    // For the second effect, we need to re-read init_status and user inside the closure
    let nav_for_logic = nav.clone();
    let mut render_blocked_for_logic = render_blocked.clone();
    let route_for_logic = route.clone();

    use_effect(move || {
        let init_status = auth_store.init_status.read();
        if init_status.is_success() {
            let user = auth_store.user.read().clone();
            let is_open_route = OPEN_ROUTES.iter().any(|r| r == &route_for_logic);
            let is_logged_in = user.is_some();
            let is_admin = user.as_ref().map(|u| u.role.as_str() == "admin").unwrap_or(false);
            let nav = nav_for_logic.clone();
            // let render_blocked = render_blocked_for_logic.clone();
            let route = route_for_logic.clone();
            spawn(async move {
                if is_logged_in && !is_admin {
                    auth_store.logout().await;
                    if !matches!(route, Route::LoginScreen { .. }) {
                        nav.replace(Route::LoginScreen {});
                    }
                    render_blocked_for_logic.set(true);
                    return;
                }
                if is_logged_in && is_open_route {
                    nav.replace(Route::HomeScreen {});
                    render_blocked_for_logic.set(true);
                    return;
                }
                if !is_logged_in && !is_open_route {
                    nav.replace(Route::LoginScreen {});
                    render_blocked_for_logic.set(true);
                    return;
                }
                render_blocked_for_logic.set(false);
            });
        }
    });

    let init_status = auth_store.init_status.read();
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
