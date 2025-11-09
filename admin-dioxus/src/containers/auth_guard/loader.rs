use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct AuthGuardLoaderProps {
    title: String,
    copy: String,
    #[props(default = false)]
    overlay: bool,
    #[props(default = true)]
    show: bool,
}

#[component]
pub fn AuthGuardLoader(props: AuthGuardLoaderProps) -> Element {
    let container_class = if props.overlay {
        "fixed inset-0 z-50 bg-background/80 backdrop-blur-sm flex items-center justify-center px-4"
    } else {
        "min-h-screen bg-background flex items-center justify-center px-4"
    };

    rsx! {
        div { class: "{container_class}",
            div { class: "w-full max-w-sm text-center space-y-6",
                div { class: "relative mx-auto h-24 w-24 flex items-center justify-center",
                    div { class: "absolute inset-0 rounded-full border-4 border-primary/20 border-t-primary animate-spin" }
                    img {
                        class: "h-12 w-12 relative",
                        src: asset!("/assets/logo.png"),
                        alt: "Ruxlog",
                    }
                }
                div { class: "space-y-2",
                    p { class: "text-lg font-semibold text-foreground", "{props.title}" }
                    p { class: "text-sm text-muted-foreground", "{props.copy}" }
                }
            }
        }
    }
}
