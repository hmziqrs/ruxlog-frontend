use super::{ToastManager, Toast as ToastData};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastProviderProps {
    pub children: Element,
}

#[derive(Props, Clone, PartialEq)]
pub struct ToastProps {
    pub toast: ToastData,
}

#[component]
fn Toast(props: ToastProps) -> Element {
    let toast = &props.toast;
    
    // Determine toast class based on toast type
    let toast_class = match toast.toast_type {
        super::ToastType::Info => "bg-blue-100 border-blue-500 text-blue-700",
        super::ToastType::Success => "bg-green-100 border-green-500 text-green-700",
        super::ToastType::Warning => "bg-yellow-100 border-yellow-500 text-yellow-700",
        super::ToastType::Error => "bg-red-100 border-red-500 text-red-700",
    };

    rsx! {
        div { class: "px-4 py-3 rounded border-l-4 shadow-md {toast_class}",
            div { class: "font-bold", "{toast.title}" }
            div { "{toast.body}" }
        }
    }
}

#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    #[allow(unused_mut)]
    let mut manager = use_signal(|| ToastManager::default());

    client! {
        let mut eval = document::eval(
            r#"
            setInterval(() => {
                dioxus.send("");
            }, 1000)
            "#,
        );

        use_hook(|| {
            spawn(async move {
                loop {
                    let _ = eval.recv::<String>().await;
                    manager.write().cleanup_expired();
                }
            })
        });
    }
    use_context_provider(|| manager);

    rsx! {
        div { class: "relative",
            {props.children}
            if manager().toasts.len() > 0 {
                div { class: "absolute bottom-4 right-4 space-y-4",
                    for (_id , toast) in manager().toasts.iter() {
                        Toast { key: "{toast.id}", toast: toast.clone() }
                    }
                }
            }
        }
    }
}