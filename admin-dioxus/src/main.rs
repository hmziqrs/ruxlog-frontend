use dioxus::prelude::*;

pub mod router;
pub mod screens;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        Router::<crate::router::Route> {}
    }
}
