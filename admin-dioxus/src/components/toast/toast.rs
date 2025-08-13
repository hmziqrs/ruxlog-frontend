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
    
    // Themed base and subtle color variants according to toast type
    let base_class = "px-4 py-3 rounded-md border shadow-lg bg-background text-foreground border-border";
    let (variant_class, title_class) = match toast.toast_type {
        super::ToastType::Info => ("border-l-4 border-l-primary", "text-primary"),
        super::ToastType::Success => ("border-l-4 border-l-emerald-500 dark:border-l-emerald-400", "text-emerald-600 dark:text-emerald-400"),
        super::ToastType::Warning => ("border-l-4 border-l-amber-500 dark:border-l-amber-400", "text-amber-600 dark:text-amber-400"),
        super::ToastType::Error => ("border-l-4 border-l-destructive", "text-destructive"),
    };

    rsx! {
        div { class: "{base_class} {variant_class}",
            div { class: "font-semibold {title_class}", "{toast.title}" }
            div { class: "text-sm text-muted-foreground", "{toast.body}" }
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
                div { class: "fixed bottom-4 right-4 z-50 space-y-3",
                    for (_id , toast) in manager().toasts.iter() {
                        Toast { key: "{toast.id}", toast: toast.clone() }
                    }
                }
            }
        }
    }
}
