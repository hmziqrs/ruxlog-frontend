use dioxus::prelude::*;

use crate::ui::shadcn::Button;

#[derive(Props, PartialEq, Clone)]
pub struct ListErrorBannerProps {
    pub message: String,
    #[props(default = None)]
    pub retry_label: Option<String>,
    #[props(default = None)]
    pub on_retry: Option<EventHandler<()>>,
}

/// Simple error banner for list views, with optional Retry action.
#[component]
pub fn ListErrorBanner(props: ListErrorBannerProps) -> Element {
    rsx! {
        div { class: "mb-4 rounded-md border border-red-200 bg-red-50 p-3 text-sm text-red-700 dark:border-red-900/40 dark:bg-red-950/30 dark:text-red-300",
            div { class: "flex items-center justify-between gap-3",
                p { class: "leading-tight", "{props.message}" }
                if let (Some(label), Some(on_retry)) = (props.retry_label.clone(), props.on_retry.clone()) {
                    Button { onclick: move |_| { on_retry.call(()); }, class: "h-8 px-3", "{label}" }
                }
            }
        }
    }
}
