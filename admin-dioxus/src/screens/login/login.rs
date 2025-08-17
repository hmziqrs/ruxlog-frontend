use std::rc::Rc;

use crate::components::{use_toast, ToastOptions};
use dioxus::html::geometry::euclid::Rect;
use dioxus::{logger::tracing, prelude::*};

use super::form::{use_login_form, LoginForm};
use crate::config::DarkMode;
use crate::hooks::use_previous;
use crate::ui::shadcn::Button;
use crate::{components::AppInput, store::use_auth};

#[component]
pub fn LoginScreen() -> Element {
    let mut ox_form = use_login_form(LoginForm::dev());
    let auth_store = use_auth();
    let login_status = auth_store.login_status.read();
    let is_loading = login_status.is_loading();
    let prev_loading = use_previous(is_loading);
    let toasts = use_toast();
    let mut mouse_pos = use_signal(|| (0, 0)); // Initialize at 0,0
    let mut card_ref = use_signal(|| None as Option<Rc<MountedData>>);
    let mut card_dimensions = use_signal(Rect::zero);

    let dark_mode = use_context::<Signal<DarkMode>>();
    let is_dark = dark_mode.read().0;

    let calculate = use_callback(move |_: ()| {
        spawn(async move {
            let read = card_ref.read();
            let client_rect = read.as_ref().map(|el| el.get_client_rect());
            if let Some(client_rect) = client_rect {
                if let Ok(rect) = client_rect.await {
                    card_dimensions.set(rect);
                    tracing::info!("dimensions: {:?} | {:?}", rect.origin, rect.size);
                }
            }
        });
    });

    use_effect(use_reactive!(|(is_loading,)| {
        if prev_loading != Some(is_loading) {
            toasts.info("Hello".to_string(), ToastOptions::new().description(""));
        }
    }));

    rsx! {
        div { class: "relative flex items-center justify-center min-h-screen overflow-hidden transition-colors duration-300",
            // Container for the card with visible overflow for the moving blob effect
            div { class: "relative w-full max-w-md",
                // Blob that follows mouse position using div with radial gradient
                div {
                    class: "absolute pointer-events-none transition-all duration-300 ease-out opacity-50",
                    style: format!(
                        "left: {}px; top: {}px; transform: translate(-50%, -50%); width: 300px; height: 300px; border-radius: 50%; background: radial-gradient(circle, {} 0%, {} 70%); filter: blur(20px); z-index: 0;",
                        mouse_pos().0,
                        mouse_pos().1,
                        if is_dark { "rgba(244,244,245,0.3)" } else { "rgba(39,39,42,0.5)" },
                        if is_dark { "rgba(113,113,122,0)" } else { "rgba(212,212,216,0)" },
                    ),
                }
                // Card with proper mouse tracking
                div {
                    onmounted: move |cx| {
                        card_ref.set(Some(cx.data()));
                        calculate.call(());
                    },
                    onresize: move |_| {
                        calculate.call(());
                    },
                    class: "relative w-full overflow-visible rounded-2xl bg-zinc-200/40 dark:bg-zinc-950/60 backdrop-blur-md shadow-xl transition-colors duration-300",
                    onmousemove: move |evt| {
                        if let Some(_) = &*card_ref.read() {
                            let d = card_dimensions.peek().origin;
                            let x = evt.data.client_coordinates().x as f64 - d.x;
                            let y = evt.data.client_coordinates().y as f64 - d.y;
                            mouse_pos.set((x as i32, y as i32));
                        }
                    },
                    // Base border - always visible but subtle
                    div {
                        class: "absolute inset-0 rounded-2xl pointer-events-none",
                        style: if is_dark { "background: transparent; box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.08);" } else { "background: transparent; box-shadow: inset 0 0 0 1px rgba(39, 39, 42, 0.08);" },
                    }
                    // Radial border highlight that follows mouse - much lighter now
                    div {
                        class: "absolute inset-0 rounded-2xl pointer-events-none overflow-hidden",
                        style: format!(
                            "mask: radial-gradient(circle 100px at {}px {}px, white, transparent); -webkit-mask: radial-gradient(circle 100px at {}px {}px, white, transparent); box-shadow: inset 0 0 0 1px {}; transition: opacity 0.15s; opacity: {};",
                            mouse_pos().0,
                            mouse_pos().1,
                            mouse_pos().0,
                            mouse_pos().1,
                            if is_dark { "rgba(244,244,244,0.5)" } else { "rgba(39,39,42,0.4)" },
                            if mouse_pos().0 > 0 { "1" } else { "0" },
                        ),
                    }
                    // Card content
                    div { class: "relative z-20 p-8 space-y-6",
                        // Logo or icon placeholder
                        div { class: "flex justify-center mb-2",
                            img {
                                class: "h-26 w-26",
                                src: asset!("/assets/logo.png"),
                                alt: "Logo",
                            }
                        }
                        h1 { class: "text-3xl font-extrabold text-center text-zinc-800 dark:text-zinc-100 tracking-tight transition-colors duration-300",
                            "Admin Login"
                        }
                        p { class: "text-center text-zinc-600 dark:text-zinc-400 text-sm mb-4 transition-colors duration-300",
                            "Sign in to your admin dashboard"
                        }
                        form { class: "space-y-5",
                            AppInput {
                                name: "email",
                                form: ox_form,
                                label: "Email",
                                placeholder: "Enter your email",
                            }
                            AppInput {
                                name: "password",
                                form: ox_form,
                                label: "Password",
                                placeholder: "Enter your password",
                                r#type: "password",
                            }
                            div { class: "flex items-center justify-between text-xs text-zinc-600 dark:text-zinc-400 transition-colors duration-300",
                                label { class: "flex items-center gap-2 select-none cursor-pointer",
                                    input {
                                        class: "checkbox checkbox-primary",
                                        r#type: "checkbox",
                                    }
                                    span { "Remember me" }
                                }
                                a {
                                    class: "hover:underline text-zinc-700 dark:text-zinc-300 font-medium hover:text-zinc-900 dark:hover:text-white transition-colors duration-150",
                                    href: "#",
                                    "Forgot password?"
                                }
                            }
                            Button {
                                class: "w-full",
                                disabled: login_status.is_loading(),
                                onclick: move |e: Event<MouseData>| {
                                    e.prevent_default();
                                    ox_form
                                        .write()
                                        .on_submit(move |val| {
                                            spawn(async move {
                                                let email = val.email.clone();
                                                let password = val.password.clone();
                                                auth_store.login(email, password).await;
                                            });
                                        });
                                },
                                if login_status.is_loading() {
                                    div { class: "loading loading-spinner loading-xs" }
                                }
                                span { "Login" }
                            }
                        }
                        p { class: "text-sm text-center text-zinc-600 dark:text-zinc-400 mt-4 transition-colors duration-300",
                            "Don't have an account? "
                            a {
                                class: "text-zinc-700 dark:text-zinc-300 font-semibold hover:text-zinc-900 dark:hover:text-zinc-100 transition-colors duration-150",
                                href: "#",
                                "Sign up"
                            }
                        }
                    }
                }
            }
        }
    }
}
