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
        
        tracing::info!("prev: {:?} | current: {:?}", prev_loading, is_loading);
        if  prev_loading != Some(is_loading) {
            toast.write().popup(ToastInfo::simple("Hello"));

        }
    }));

    rsx! {
        div { class: "flex items-center justify-center min-h-screen bg-base-200/30",
            div { class: "w-full max-w-md p-8 space-y-3 rounded-xl bg-base-100 shadow-2xl",
                h1 { class: "text-2xl font-bold text-center text-primary", "Login" }
                form { class: "space-y-6",
                    AppInput {
                        name: "email",
                        form: ox_form,
                        label: "Email",
                        placeholder: "Please input your email",
                    }
                    AppInput {
                        name: "password",
                        form: ox_form,
                        label: "Password",
                        placeholder: "Please input your password",
                        r#type: "password",
                    }
                    div { class: "flex items-center justify-between",
                        label { class: "flex items-center",
                            input {
                                class: "checkbox checkbox-primary",
                                r#type: "checkbox",
                            }
                            span { class: "ml-2 text-sm text-primary", "Remember me" }
                        }
                        a {
                            class: "text-sm text-primary hover:underline",
                            href: "#",
                            "Forgot password?"
                        }
                    }
                    button {
                        disabled: login_status.is_loading(),
                        class: "w-full btn btn-primary",
                        onclick: move |e| {
                            e.prevent_default();
                            ox_form
                                .write()
                                .on_submit(move |val| {
                                    spawn(async move {
                                        let email = val.email.clone();
                                        let password = val.password.clone();
                                        tracing::info!(
                                            "Login with email: {} and password: {}", email, password
                                        );
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
                p { class: "text-sm text-center text-primary",
                    "Don't have an account? "
                    a { class: "text-primary hover:underline", href: "#", "Sign up" }
                }
            }
        }
    }
}
