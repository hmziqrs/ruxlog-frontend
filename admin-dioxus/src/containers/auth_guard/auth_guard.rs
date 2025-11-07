use dioxus::prelude::*;

use crate::{
    components::LoadingOverlay,
    router::{Route, OPEN_ROUTES},
    store::use_auth,
    ui::shadcn::{Alert, AlertDescription, AlertTitle, AlertVariant, Button, ButtonVariant},
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
            .unwrap_or("Failed to initialize user");
        let error_type = init_status.error_type().map(|c| c.to_string());
        let error_status = init_status.error_status();
        let error_details = init_status.error_details().map(|d| d.to_string());
        let transport_kind = init_status.transport_error_kind();

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
                        Alert {
                            variant: AlertVariant::Destructive,
                            class: "border-red-200 dark:border-red-900/40 bg-transparent [&>svg]:text-current",
                            svg {
                                class: "h-5 w-5",
                                fill: "none",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                view_box: "0 0 24 24",
                                path { d: "M12 9v4m0 4h.01M21 12c0 4.97-4.03 9-9 9s-9-4.03-9-9 4.03-9 9-9 9 4.03 9 9z" }
                            }
                        AlertTitle {
                            if let Some(kind) = transport_kind {
                                match kind {
                                    crate::store::TransportErrorKind::Network | crate::store::TransportErrorKind::Timeout => {
                                        rsx!("Connection Error")
                                    }
                                    _ => rsx!("Authentication Error"),
                                }
                            } else {
                                rsx!("Authentication Error")
                            }
                        }
                            AlertDescription {
                                class: "mt-2 space-y-1",
                                p { class: "text-sm leading-6", {error_msg} }
                                if let Some(kind) = transport_kind {
                                    match kind {
                                        crate::store::TransportErrorKind::Network => {
                                            p { class: "text-xs text-muted-foreground", "The API server is unreachable. Check the backend is running and CORS/proxy settings." }
                                        }
                                        crate::store::TransportErrorKind::Timeout => {
                                            p { class: "text-xs text-muted-foreground", "The request timed out. Please try again." }
                                        }
                                        _ => {}
                                    }
                                }
                                if let Some(t) = error_type {
                                    p { class: "text-xs text-muted-foreground", "Type: ", {t} }
                                }
                                if let Some(status) = error_status {
                                    p { class: "text-xs text-muted-foreground", "Status: ", {status.to_string()} }
                                }
                                if let Some(details) = error_details {
                                    p { class: "text-xs text-muted-foreground break-words", {details} }
                                }
                            }
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
