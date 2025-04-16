use crate::hooks::{OxForm, OxFormModel};
use dioxus::{logger::tracing, prelude::*};

#[derive(Props, PartialEq, Clone)]
pub struct AppInputProps<T: OxFormModel + 'static> {
    form: Signal<OxForm<T>>,
    name: String,
    #[props(default = None)]
    label: Option<String>,
    #[props(default = None)]
    placeholder: Option<String>,
    #[props(default = String::from("text"))]
    r#type: String,
    #[props(default = None)]
    onchange: Option<EventHandler<String>>,
    #[props(default = None)]
    onblur: Option<EventHandler<()>>,
    #[props(default = None)]
    onfocus: Option<EventHandler<()>>,
    #[props(default = false)]
    disabled: bool,
    #[props(default = false)]
    readonly: bool,
}

#[component]
pub fn AppInput<T>(props: AppInputProps<T>) -> Element
where
    T: OxFormModel + 'static,
{
    let mut form = props.form.clone();
    let name = props.name.clone();
    
    let field = {
        let read = form.read();
        read.get_field(&name).unwrap().clone()
    };

    let name_on = name.clone();
    let name_ob = name.clone();
    let name_of = name.clone();
    
    let onchange_handler = props.onchange.clone();
    let onblur_handler = props.onblur.clone();
    let onfocus_handler = props.onfocus.clone();

    rsx! {
        div {
            if let Some(label) = &props.label {
                label { class: "block text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-2 transition-colors duration-200",
                    {label.clone()}
                }
            }
            input {
                disabled: props.disabled,
                readonly: props.readonly,
                r#type: props.r#type.clone(),
                class: "w-full px-4 py-2 rounded-md border border-zinc-300 dark:border-zinc-600 bg-zinc-50 dark:bg-zinc-700 text-zinc-800 dark:text-zinc-200 placeholder:text-zinc-500 dark:placeholder:text-zinc-400 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200",
                value: field.value.clone(),
                placeholder: props.placeholder.clone(),
                onchange: move |event| {
                    let value = event.value();
                    if let Some(handler) = &onchange_handler {
                        handler.call(value.clone());
                    }
                    form.write().update_field(&name_on, value);
                },
                onblur: move |_| {
                    if let Some(handler) = &onblur_handler {
                        handler.call(());
                    }
                    form.write().blur_field(&name_ob);
                },
                onfocus: move |_| {
                    if let Some(handler) = &onfocus_handler {
                        handler.call(());
                    }
                    form.write().focus_field(&name_of);
                },
            }
            if let Some(error) = &field.error {
                p { class: "my-2 text-sm text-red-500 dark:text-red-400 transition-colors duration-200",
                    {error.clone()}
                }
            }
        }
    }
}
