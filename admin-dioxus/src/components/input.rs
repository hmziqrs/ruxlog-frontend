use crate::hooks::{OxForm, OxFormModel};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Eq, Clone)]
pub struct AppInputProps<T: OxFormModel + 'static> {
    form: Signal<OxForm<T>>,
    name: String,
    #[props(default = None)]
    label: Option<String>,
    #[props(default = None)]
    placeholder: Option<String>,
    #[props(default = String::from("text"))]
    r#type: String,
}

#[component]
pub fn AppInput<T>(mut props: AppInputProps<T>) -> Element
where
    T: OxFormModel + 'static,
{
    let read = props.form.read();
    let field = read.get_field(&props.name).unwrap();

    let name_on = props.name.clone();
    let name_ob = props.name.clone();
    let name_of = props.name.clone();

    rsx! {
        div {
            if props.label.is_some() {
                label {
                    class: "block text-sm font-medium text-primary, mb-2",
                    {props.label.unwrap()}
                }
            }
            input {
                r#type: props.r#type,
                class: "w-full px-4 py-2 input",
                value: field.value.clone(),
                placeholder: if props.placeholder.is_some() { props.placeholder.unwrap() },
                onchange: move |event| {
                    props.form.write().update_field(&name_on, event.value());
                },
                onblur: move |_| {
                    props.form.write().blur_field(&name_ob);
                },
                onfocus: move |_| {
                    props.form.write().focus_field(&name_of);
                }
            }
            if field.error.is_some() {
                p {
                    class: "my-2 text-sm text-error",
                    {field.error.as_ref().unwrap().to_string()}
                }
            }
        }
    }
}
