use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AppPortalProps {
    /// The content to be displayed inside the portal
    pub children: Element,
    // #[props(default = 0)]
    // pub index: i32,
}

static PORTAL_INDEX: std::sync::OnceLock<GlobalSignal<i32>> = std::sync::OnceLock::new();

pub fn use_index() -> &'static GlobalSignal<i32> {
    PORTAL_INDEX.get_or_init(|| GlobalSignal::new(|| 0))
}

#[component]
pub fn AppPortal(props: AppPortalProps) -> Element {
    let portal_index=  use_index();
    let mut z_index = use_signal(|| 50);


    use_effect(move || {
        let index = portal_index() + 1;
        *portal_index.write() = index;
        z_index.set(z_index() + index);
    });



    rsx! {
        div {
            class: "fixed top-0 left-0 w-full h-full",
            style: format!("z-index: {}", z_index.read()),
            {props.children}
        }
    }
}