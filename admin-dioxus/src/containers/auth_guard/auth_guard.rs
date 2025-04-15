use dioxus::{logger::tracing, prelude::*};

use crate::{
    router::{Route, OPEN_ROUTES},
    store::use_auth,
};

#[component]
pub fn AuthGuard() -> Element {
    let render_blocked = use_signal(|| true);

    let auth_store = use_auth();
    let nav = use_navigator();
    let route: Route = use_route();

    let init_status = auth_store.init_status.read();
    let init_status_hook = init_status.clone();

    use_effect(move || {
        tracing::info!("AuthGuard: use_hook init_status = {:?}", init_status_hook);
        if init_status_hook.is_init() {
            spawn(async move {
                auth_store.init().await;
            });
        } else if init_status_hook.is_success() {
            //
        }
    });

    // For the second effect, we need to re-read init_status and user inside the closure
    let nav_for_logic = nav.clone();
    let mut render_blocked_for_logic = render_blocked.clone();
    let route_for_logic = route.clone();

    use_effect(use_reactive!(|(route_for_logic)| {
        tracing::info!("AuthGuard: use_effect ");
        let init_status = auth_store.init_status.read();
        if init_status.is_success() {
            let user = auth_store.user.read().clone();
            let is_open_route = OPEN_ROUTES.iter().any(|r| r == &route_for_logic);
            let is_logged_in = user.is_some();
            let is_admin = user
                .as_ref()
                .map(|u| u.role.as_str() == "admin")
                .unwrap_or(false);
            let nav = nav_for_logic.clone();
            let route = route_for_logic.clone();

            spawn(async move {
                if is_logged_in && !is_admin {
                    auth_store.logout().await;
                    if !matches!(route, Route::LoginScreen { .. }) {
                        nav.replace(Route::LoginScreen {});
                    }
                    return;
                }

                if is_logged_in && is_open_route {
                    render_blocked_for_logic.set(false);
                    nav.replace(Route::HomeScreen {});
                    return;
                }
                if !is_logged_in && !is_open_route {
                    nav.replace(Route::LoginScreen {});
                    return;
                }
                render_blocked_for_logic.set(false);
            });
        }
    }));

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
