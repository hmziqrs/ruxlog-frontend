use dioxus::prelude::*;

use super::form::{use_user_form, UserForm};
use crate::components::AppInput;
use crate::hooks::OxForm;
use crate::router::Route;
use crate::ui::custom::AppPortal;
use crate::ui::shadcn::{Button, ButtonVariant, Checkbox};

#[derive(Props, PartialEq, Clone)]
pub struct UserFormContainerProps {
    #[props(default)]
    pub initial: Option<UserForm>,
    pub on_submit: EventHandler<UserForm>,
    #[props(default)]
    pub title: Option<String>,
    #[props(default)]
    pub submit_label: Option<String>,
}

#[component]
pub fn UserFormContainer(props: UserFormContainerProps) -> Element {
    let nav = use_navigator();
    let initial_user_form = props.initial.clone().unwrap_or_else(UserForm::new);
    let reset_template = initial_user_form.clone();
    let user_form_hook = use_user_form(initial_user_form);
    let mut form = user_form_hook.form;
    let mut reset_dialog_open = use_signal(|| false);
    let is_form_dirty = form.read().is_dirty();
    let is_update = form.read().data.is_update;

    rsx! {
        div {
            if let Some(t) = props.title.clone() { h1 { class: "sr-only", {t} } }

            div { class: "grid grid-cols-1 gap-10 lg:grid-cols-3",
                div { class: "lg:col-span-2 space-y-8",
                    div { class: "rounded-xl border border-border/70 bg-transparent",
                        div { class: "px-6 py-6",
                            h2 { class: "text-lg font-semibold", "User details" }
                            p { class: "text-sm text-muted-foreground", "Basic information and authentication credentials." }
                        }
                        div { class: "px-6 py-6 space-y-6",
                            AppInput {
                                name: "name",
                                form,
                                label: "Name",
                                placeholder: "e.g. John Doe"
                            }

                            div { class: "h-px bg-border/60" }

                            AppInput {
                                name: "email",
                                form,
                                label: "Email",
                                r#type: "email",
                                placeholder: "user@example.com"
                            }

                            div { class: "h-px bg-border/60" }

                            div { class: "space-y-3",
                                label { class: "block text-sm font-medium text-foreground",
                                    {if is_update { "Password (optional)" } else { "Password" }}
                                }
                                AppInput {
                                    name: "password",
                                    form,
                                    r#type: "password",
                                    placeholder: if is_update { "Leave blank to keep current password" } else { "Enter password" }
                                }
                                if is_update {
                                    p { class: "text-xs text-muted-foreground", "Leave blank to keep the current password unchanged." }
                                }
                            }
                        }
                    }

                    div { class: "flex items-start gap-3 rounded-lg border border-border/60 bg-transparent p-5",
                        div { class: "mt-0.5 h-4 w-4 rounded-full border border-border/40" }
                        div { class: "space-y-1",
                            p { class: "text-sm font-medium text-foreground", "Security tip" }
                            p { class: "text-sm text-muted-foreground", "Ensure users verify their email and use strong passwords. Super admins have full system access." }
                        }
                    }
                }

                div { class: "space-y-8 lg:sticky lg:top-28 h-fit",
                    div { class: "rounded-xl border border-border/70 bg-transparent",
                        div { class: "px-6 pt-6",
                            h2 { class: "text-lg font-semibold", "Role & Permissions" }
                            p { class: "text-sm text-muted-foreground", "Assign role and set verification status." }
                        }
                        div { class: "px-6 py-6 space-y-6",
                            div { class: "space-y-3",
                                label { class: "block text-sm font-medium text-foreground", "Role" }
                                select {
                                    class: "w-full rounded-md border border-border/70 bg-transparent px-3 py-2 text-sm text-foreground transition-colors duration-200 focus:border-ring focus:ring-2 focus:ring-ring/40",
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
                                p { class: "text-xs text-muted-foreground", "Determines what actions this user can perform." }
                            }

                            div { class: "h-px bg-border/60" }

                            div { class: "flex items-center justify-between",
                                div { class: "space-y-0.5",
                                    label { class: "block text-sm font-medium text-foreground", "Email Verified" }
                                    p { class: "text-xs text-muted-foreground",
                                        if form.read().data.is_verified {
                                            "User has verified their email address."
                                        } else {
                                            "User has not verified their email address."
                                        }
                                    }
                                }
                                Checkbox {
                                    class: None,
                                    checked: form.read().data.is_verified,
                                    onchange: move |checked: bool| {
                                        form.write().update_field("is_verified", checked.to_string());
                                    }
                                }
                            }
                        }
                    }

                    div { class: "flex gap-3 pt-4",
                        Button { class: "flex-1 w-auto", variant: ButtonVariant::Outline,
                            onclick: move |_| {
                                if form.peek().is_dirty() {
                                    reset_dialog_open.set(true);
                                } else {
                                    nav.push(Route::UsersListScreen {});
                                }
                            },
                            {if is_form_dirty { "Reset" } else { "Cancel" }}
                        }
                        Button { class: "flex-1 w-auto",
                            onclick: move |_| {
                                let submit = props.on_submit.clone();
                                form.write().on_submit(move |val| { submit.call(val); });
                            },
                            {props.submit_label.clone().unwrap_or_else(|| "Save User".to_string())}
                        }
                    }
                }
            }
        }
        if reset_dialog_open() {
            AppPortal {
                class: "bg-black/20 backdrop-blur-sm flex items-center justify-center px-4",
                div { class: "w-full max-w-md rounded-lg border border-border/60 bg-background p-6 shadow-lg",
                    div { class: "space-y-2",
                        h2 { class: "text-lg font-semibold", "Reset form?" }
                        p { class: "text-sm text-muted-foreground", "All changes will be cleared and the form will return to its default state." }
                    }
                    div { class: "mt-6 flex justify-end gap-2",
                        Button { variant: ButtonVariant::Outline,
                            onclick: move |_| {
                                reset_dialog_open.set(false);
                            },
                            "Cancel"
                        }
                        Button { variant: ButtonVariant::Destructive,
                            onclick: move |_| {
                                form.set(OxForm::new(reset_template.clone()));
                                reset_dialog_open.set(false);
                            },
                            "Reset form"
                        }
                    }
                }
            }
        }
    }
}
