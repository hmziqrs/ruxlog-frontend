use dioxus::{logger::tracing, prelude::*};

use super::form::{use_user_form, UserForm, UserRole};
use crate::components::AppInput;

#[component]
pub fn UserScreen() -> Element {
    let initial_user_form = UserForm::new();
    let mut form = use_user_form(initial_user_form);
    let is_update = form.read().data.is_update;

    rsx! {
        div { class: "p-8 max-w-4xl mx-auto",
            div { class: "bg-base-100 shadow-2xl rounded-xl p-8",
                h1 { class: "text-2xl font-bold mb-6 text-primary", "New User" }
                form { class: "space-y-6",
                    // Name field
                    AppInput {
                        name: "name",
                        form,
                        label: "Name",
                        placeholder: "User name",
                    }

                    // Email field
                    AppInput {
                        name: "email",
                        form,
                        label: "Email",
                        placeholder: "User email",
                    }

                    // Password field
                    AppInput {
                        name: "password",
                        form,
                        r#type: "password",
                        label: if is_update { "Password (optional)" } else { "Password" },
                        placeholder: "Password",
                    }

                    // Role select field
                    div { class: "space-y-2",
                        label { class: "block text-sm font-medium text-primary", "Role" }
                        select {
                            class: "select select-bordered w-full",
                            value: form.read().data.role.to_string(),
                            onchange: move |e| {
                                form.write().update_field("role", e.value());
                            },
                            option { value: "super-admin", "Super Admin" }
                            option { value: "admin", "Admin" }
                            option { value: "moderator", "Moderator" }
                            option { value: "author", "Author" }
                            option { value: "user", "User" }
                        }
                    }

                    // Is Verified toggle
                    div { class: "flex flex-row items-center justify-between border rounded-lg p-4",
                        div { class: "space-y-0.5",
                            label { class: "text-base", "Verified" }
                            div { class: "text-sm", "User has verified their email" }
                        }
                        div { class: "flex items-center",
                            input {
                                class: "checkbox checkbox-primary",
                                r#type: "checkbox",
                                checked: form.read().data.is_verified,
                                onchange: move |e| {
                                    form.write().update_field("is_verified", e.value());
                                },
                            }
                        }
                    }

                    // Form actions
                    div { class: "flex justify-end gap-4 pt-4",
                        button { class: "btn", r#type: "button", "Cancel" }
                        button {
                            class: "btn btn-primary",
                            onclick: move |e| {
                                e.prevent_default();
                                tracing::info!("Form submitted OK ??");
                                form.write()
                                    .on_submit(|val| {
                                        tracing::info!("User form submitted: {:?}", val);
                                    });
                            },
                            "Create User"
                        }
                    }
                }
            }
        }
    }
}