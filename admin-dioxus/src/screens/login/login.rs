use std::time::Duration;

use dioxus::{logger::tracing, prelude::*};
use dioxus_toast::{ToastInfo, ToastManager};
// use dioxus_toast::ToastManager;

use super::form::{use_login_form, LoginForm};
use crate::components::AnimatedBlob;
use crate::{components::AppInput, store::use_auth};
use crate::hooks::use_previous;

#[component]
pub fn LoginScreen() -> Element {
    let mut ox_form = use_login_form(LoginForm::dev());
    let auth_store = use_auth();
    let login_status = auth_store.login_status.read();
    // let mut toast: Signal<ToastManager> = use_context();
    let is_loading = login_status.is_loading();
    let prev_loading = use_previous(is_loading);
    let mut toast: Signal<ToastManager> = use_context();
    let mut mouse_pos = use_signal(|| (200, 200)); // default center

    use_effect(use_reactive!(|(is_loading,)| {
        
        // tracing::info!("prev: {:?} | current: {:?}", prev_loading, is_loading);
        if  prev_loading != Some(is_loading) {
            toast.write().popup(ToastInfo::simple("Hello"));

        }
    }));

    rsx! {
        div { class: "relative flex items-center justify-center min-h-screen bg-neutral-950 overflow-visible",
            // Animated neutral blob
            AnimatedBlob {}

            // Card with border highlight
            div {
                class: "relative w-full max-w-md overflow-visible rounded-2xl border border-neutral-800 bg-neutral-900/70 backdrop-blur-md shadow-xl",
                onmousemove: move |evt| {
                    let cords = evt.data.coordinates();
                    let c_cords = evt.data.client_coordinates();
                    let e_cords = evt.data.element_coordinates();
                    tracing::info!("Mouse position: {:?} | {:?} | {:?}", cords, c_cords, e_cords);
                    mouse_pos.set((e_cords.x as i32, e_cords.y as i32));
                },
                // Border highlight overlay
                div {
                    class: "pointer-events-none absolute inset-0 rounded-2xl transition-all duration-300 z-10",
                    style: format!(
                        "background:radial-gradient(300px circle at {}px {}px,rgba(244,244,245,0.10),transparent 60%);border:1px solid rgba(244,244,245,0.10);",
                        mouse_pos().0,
                        mouse_pos().1,
                    ),
                }
                // ...existing card content...
                div { class: "relative z-20 p-8 space-y-6",
                    // Logo or icon placeholder
                    div { class: "flex justify-center mb-2",
                        img {
                            class: "h-26 w-26 rounded-full shadow-md border border-neutral-800 bg-neutral-900",
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
                                class: "hover:underline text-primary font-medium transition-colors duration-150",
                                href: "#",
                                "Forgot password?"
                            }
                        }
                        button {
                            disabled: login_status.is_loading(),
                            class: "w-full btn btn-primary btn-lg shadow-md hover:shadow-lg transition-all duration-150 flex items-center justify-center gap-2",
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
                            class: "text-primary font-semibold hover:underline transition-colors duration-150",
                            href: "#",
                            "Sign up"
                        }
                    }
                }
            }
        }
    }
}
