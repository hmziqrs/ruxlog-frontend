use std::rc::Rc;

use dioxus::html::geometry::euclid::Rect;
use dioxus::{logger::tracing, prelude::*};
use dioxus_toast::{ToastInfo, ToastManager};

use super::form::{use_login_form, LoginForm};
use crate::components::AnimatedBlob;
use crate::hooks::use_previous;
use crate::{components::AppInput, store::use_auth};

#[component]
pub fn LoginScreen() -> Element {
    let mut ox_form = use_login_form(LoginForm::dev());
    let auth_store = use_auth();
    let login_status = auth_store.login_status.read();
    let is_loading = login_status.is_loading();
    let prev_loading = use_previous(is_loading);
    let mut toast: Signal<ToastManager> = use_context();
    let mut mouse_pos = use_signal(|| (0, 0)); // Initialize at 0,0
    let mut card_ref = use_signal(|| None as Option<Rc<MountedData>>);
    let mut card_dimensions = use_signal(Rect::zero);

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
            toast.write().popup(ToastInfo::simple("Hello"));
        }
    }));

    rsx! {
        div { class: "relative flex items-center justify-center min-h-screen bg-neutral-950 overflow-hidden",
            // AnimatedBlob component remains outside the card for a background effect
            AnimatedBlob {}

            // Container for the card with visible overflow for the moving blob effect
            div { class: "relative w-full max-w-md",
                // Blob that follows mouse position
                div {
                    class: "absolute pointer-events-none",
                    style: format!(
                        "left: {}px; top: {}px; transform: translate(-50%, -50%); width: 300px; height: 300px; border-radius: 50%; background: radial-gradient(circle, rgba(244,244,245,0.6) 0%, rgba(113,113,122,0) 70%); filter: blur(60px); opacity: 0.6; z-index: 0;",
                        mouse_pos().0,
                        mouse_pos().1,
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
                    class: "relative w-full overflow-visible rounded-2xl bg-neutral-900/50 backdrop-blur-md shadow-xl",
                    onmousemove: move |evt| {
                        if let Some(_) = &*card_ref.read() {
                            let d = card_dimensions.peek().origin;
                            let x = evt.data.client_coordinates().x as f64 - d.x;
                            let y = evt.data.client_coordinates().y as f64 - d.y;
                            mouse_pos.set((x as i32, y as i32));
                        }
                    },
                    // Basic card outline border
                    div {
                        class: "absolute inset-0 rounded-2xl pointer-events-none",
                        style: "background: transparent; box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.1);",
                    }
                    // Radial border highlight that follows mouse
                    div {
                        class: "absolute inset-0 rounded-2xl pointer-events-none overflow-hidden",
                        style: format!(
                            "mask: radial-gradient(circle 100px at {}px {}px, white, transparent); -webkit-mask: radial-gradient(circle 100px at {}px {}px, white, transparent); box-shadow: inset 0 0 0 1px rgba(244,244,245,0.3); transition: opacity 0.15s; opacity: {};",
                            mouse_pos().0,
                            mouse_pos().1,
                            mouse_pos().0,
                            mouse_pos().1,
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
                        h1 { class: "text-3xl font-extrabold text-center text-neutral-100 tracking-tight",
                            "Admin Login"
                        }
                        p { class: "text-center text-neutral-400 text-sm mb-4",
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
                            div { class: "flex items-center justify-between text-xs text-neutral-400",
                                label { class: "flex items-center gap-2 select-none cursor-pointer",
                                    input {
                                        class: "checkbox checkbox-primary",
                                        r#type: "checkbox",
                                    }
                                    span { "Remember me" }
                                }
                                a {
                                    class: "hover:underline text-neutral-300 font-medium transition-colors duration-150",
                                    href: "#",
                                    "Forgot password?"
                                }
                            }
                            button {
                                disabled: login_status.is_loading(),
                                class: "w-full btn btn-primary btn-lg shadow-md hover:shadow-lg transition-all duration-150 flex items-center justify-center gap-2 ",
                                onclick: move |e| {
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
                        p { class: "text-sm text-center text-neutral-400 mt-4",
                            "Don't have an account? "
                            a {
                                class: "text-neutral-300 font-semibold hover:text-neutral-100 transition-colors duration-150",
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
