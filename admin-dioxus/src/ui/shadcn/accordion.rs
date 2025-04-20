use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub struct AccordionContext {
    pub open_value: Option<String>,
}

impl AccordionContext {
    pub fn new() -> Self {
        Self {
            open_value: None,
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AccordionProps {
    #[props(optional)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    use_context_provider(Signal::new(AccordionContext::new()));

    rsx! {
        div { {props.children} }
    }
}