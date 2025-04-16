use dioxus::prelude::*;

#[component]
pub fn HomeScreen() -> Element {
    rsx! {
        div { class: "",
            h1 { "Welcome to Dioxus!" }
            p { "This is a simple example of a Dioxus application." }
            button { class: "btn btn-primary", onclick: move |_| {}, "Simulate" }
        }
    }
}
