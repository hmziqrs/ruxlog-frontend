use dioxus::prelude::*;

use crate::{
    components::{ErrorDetails, ErrorDetailsVariant, LoadingOverlay},
    router::{Route, OPEN_ROUTES},
    store::{use_auth, AppError},
    ui::shadcn::{Button, ButtonVariant},
};

#[component]
pub fn AuthGuardContainer() -> Element {
    let render_blocked = use_signal(|| true);

    let auth_store = use_auth();
    let nav = use_navigator();
    let route: Route = use_route();

    let init_status = auth_store.init_status.read();
    let init_status_hook = init_status.clone();

    use_effect(move || {
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
        let init_status = auth_store.init_status.read();
        if init_status.is_success() {
            let user = auth_store.user.read().clone();
            let is_open_route = OPEN_ROUTES.iter().any(|r| r == &route_for_logic);
            let is_logged_in = user.is_some();
            let is_admin = user.as_ref().map(|u| u.is_admin()).unwrap_or(false);
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
        let error_msg = init_status
            .error_message()
            .unwrap_or_else(|| "Failed to initialize user".to_string());
        let transport_kind = init_status.transport_error_kind();
        let alert_title = match transport_kind {
            Some(crate::store::TransportErrorKind::Network)
            | Some(crate::store::TransportErrorKind::Timeout) => "Connection Error",
            _ => "Authentication Error",
        }
        .to_string();
        let error = init_status.error.clone().or_else(|| {
            Some(AppError::Other {
                message: error_msg.clone(),
            })
        });

        return rsx! {
            div { class: "min-h-screen flex items-center justify-center bg-background p-4",
                div { class: "max-w-md w-full",
                    div { class: "rounded-xl border border-border/60 bg-background p-8 shadow-lg space-y-6",
                        // Logo
                        div { class: "flex justify-center mb-2",
                            img {
                                class: "h-24 w-24",
                                src: asset!("/assets/logo.png"),
                                alt: "Logo",
                            }
                        }
                        ErrorDetails {
                            error,
                            variant: ErrorDetailsVariant::Minimum,
                            title: Some(alert_title),
                            class: Some("w-full".to_string()),
                        }
                        div { class: "flex justify-center pt-2",
                            Button {
                                variant: ButtonVariant::Outline,
                                onclick: move |_| {
                                    spawn(async move {
                                        auth_store.init().await;
                                    });
                                },
                                "Try Again"
                            }
                        }
                    }
                }
            }
        };
    }

    if *render_blocked.read() {
        return rsx! {
            div { class: "min-h-screen bg-background",
                LoadingOverlay {
                    visible: render_blocked
                }
            }
        };
    }

    rsx! {
        Outlet::<Route> {}
    }
}
