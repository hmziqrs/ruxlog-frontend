use std::time::Duration;

use dioxus::{logger::tracing, prelude::*};
use dioxus_toast::{ToastInfo, ToastManager};
// use dioxus_toast::ToastManager;

use super::form::{use_login_form, LoginForm};
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


    use_effect(use_reactive!(|(is_loading,)| {
        
        // tracing::info!("prev: {:?} | current: {:?}", prev_loading, is_loading);
        if  prev_loading != Some(is_loading) {
            toast.write().popup(ToastInfo::simple("Hello"));

        }
    }));

    rsx! {
        div { class: "relative flex items-center justify-center min-h-screen bg-gradient-to-br from-base-200/60 to-base-100/80 overflow-hidden",
            // Animated gradient blob behind the card
            div { class: "absolute -z-10 left-1/2 top-1/2 w-[480px] h-[320px] -translate-x-1/2 -translate-y-1/2 bg-gradient-to-tr from-primary/30 via-secondary/20 to-accent/30 rounded-full blur-3xl opacity-70 animate-pulse" }
            div { class: "w-full max-w-md p-8 space-y-6 rounded-2xl bg-base-100/80 shadow-xl border border-base-200 backdrop-blur-md transition-all duration-300 hover:border-primary/70 hover:shadow-2xl focus-within:border-primary/90",
                // Logo or icon placeholder
                div { class: "flex justify-center mb-2",
                    img {
                        class: "h-26 w-26 rounded-full shadow-md border border-base-200 bg-base-100",
                        src: asset!("/assets/logo.png"),
                        alt: "Logo",
                                        // fallback: use a placeholder if logo not available
                    // onerror: "this.style.display='none'",
                    }
                }
                h1 { class: "text-3xl font-extrabold text-center text-primary tracking-tight",
                    "Admin Login"
                }
                p { class: "text-center text-base-content/70 text-sm mb-4",
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
                    div { class: "flex items-center justify-between text-xs text-base-content/70",
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
                p { class: "text-sm text-center text-base-content/70 mt-4",
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
