use dioxus::prelude::*;
use hmziq_dioxus_free_icons::icons::ld_icons::LdArrowDown;
use hmziq_dioxus_free_icons::Icon;

#[derive(PartialEq, Clone)]
pub struct AccordionContext {
    pub open_value: Option<String>,
}

impl AccordionContext {
    pub fn new() -> Self {
        Self { open_value: None }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AccordionProps {
    #[props(optional)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    use_context_provider(|| Signal::new(AccordionContext::new()));

    rsx! {
        div { class: format!("data-slot-accordion {}", props.class), {props.children} }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AccordionItemProps {
    #[props(optional)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    rsx! {
        div { class: format!("data-slot-accordion-item border-b last:border-b-0 {}", props.class),
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AccordionTriggerProps {
    #[props(optional)]
    pub class: String,
    pub children: Element,
    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    rsx! {
        div { class: "flex",
            button {
                class: format!(
                    "data-slot-accordion-trigger flex flex-1 items-start justify-between gap-4 rounded-md py-4 text-left text-sm font-medium transition-all outline-none hover:underline focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 {}",
                    props.class,
                ),
                onclick: move |evt| {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                },
                {props.children}
                // Simple chevron icon replacement
                div { class: "pointer-events-none w-4 h-4 shrink-0 translate-y-0.5 transition-transform duration-200",
                    Icon { icon: LdArrowDown }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AccordionContentProps {
    #[props(optional)]
    pub class: String,
    pub children: Element,
    #[props(optional)]
    pub visible: Option<bool>,
}

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    let visible = props.visible.unwrap_or(false);
    let display = if visible { "block" } else { "hidden" };

    rsx! {
        div { class: "data-slot-accordion-content overflow-hidden text-sm {display}",
            div { class: "pt-0 pb-4", {props.children} }
        }
    }
}
