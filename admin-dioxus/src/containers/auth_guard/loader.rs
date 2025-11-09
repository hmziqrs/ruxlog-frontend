use dioxus::prelude::*;

#[component]
pub fn AuthGuardLoader(title: String, copy: String) -> Element {
    rsx! {
        div { class: "min-h-screen bg-background flex items-center justify-center px-4",
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
                    p { class: "text-lg font-semibold text-foreground", "{title}" }
                    p { class: "text-sm text-muted-foreground", "{copy}" }
                }
            }
        }
    }
}
