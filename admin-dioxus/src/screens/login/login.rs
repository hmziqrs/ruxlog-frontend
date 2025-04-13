use dioxus::prelude::*;

use super::form::{use_login_form, LoginForm};
use crate::components::AppInput;

#[component]
pub fn LoginScreen() -> Element {
    let mut ox_form = use_login_form(LoginForm::dev());
    // let auth = use_auth();
    // let login_status = auth.login_status.read();
    // let is_loading = login_status.is_loading();
    // let prev_loading = use_previous(is_loading);

    rsx! {
        div {
            class: "flex items-center justify-center min-h-screen bg-base-200/30",
            div {
                class: "w-full max-w-md p-8 space-y-3 rounded-xl bg-base-100 shadow-2xl",
                h1 {
                    class: "text-2xl font-bold text-center text-primary",
                    "Login"
                }
                form {
                    class: "space-y-6",
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
                    }
                    div {
                        class: "flex items-center justify-between",
                        label {
                            class: "flex items-center",
                            input {
                                class: "checkbox checkbox-primary",
                                r#type: "checkbox",
                            }
                            span {
                                class: "ml-2 text-sm text-primary",
                                "Remember me"
                            }
                        }
                        a {
                            class: "text-sm text-primary hover:underline",
                            href: "#",
                            "Forgot password?"
                        }
                    }
                    button {
                        // disabled: is_loading,
                        class: "w-full btn btn-primary",
                        onclick: move |e| {
                            e.prevent_default();
                            ox_form.write().on_submit(move |val| {
                                // spawn(async move {
                                //     auth.login(val.email, val.password).await;
                                // });
                            });
                        },
                        // if is_loading {
                        //     div { class: "loading loading-spinner loading-xs" }
                        // }
                        span { "Login" },
                    }
                }
                p {
                    class: "text-sm text-center text-primary",
                    "Don't have an account? "
                    a {
                        class: "text-primary hover:underline",
                        href: "#",
                        "Sign up"
                    }
                }
            }
        }
    }
}
