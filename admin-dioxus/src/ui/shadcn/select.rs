use std::rc::Rc;

use dioxus::{logger::tracing, prelude::*};
use hmziq_dioxus_free_icons::{
    icons::ld_icons::{LdCheck, LdChevronDown, LdChevronUp},
    Icon,
};

use crate::ui::custom::AppPortal;

/// Signal for managing select open/close state
#[derive(PartialEq)]
pub struct SelectContext {
    pub is_open: bool,
    pub selected_value: Option<String>,
    pub selected_label: Option<String>,
    pub position_x: f64,
    pub position_y: f64,
    pub width: f64,
}

impl SelectContext {
    fn new() -> Self {
        Self {
            is_open: false,
            selected_value: None,
            selected_label: None,
            position_x: 0.0,
            position_y: 0.0,
            width: 0.0,
        }
    }
}

/// Properties for all select components
#[derive(Props, PartialEq, Clone)]
pub struct SelectProps {
    /// Child elements to render inside the component
    pub children: Element,
    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,
    /// Default selected value
    #[props(default)]
    pub value: Option<String>,
    /// Event handler for value changes
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
}

/// Properties for select group
#[derive(Props, PartialEq, Clone)]
pub struct SelectGroupProps {
    /// Child elements to render inside the component
    pub children: Element,
    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,
}

/// Properties for select trigger
#[derive(Props, PartialEq, Clone)]
pub struct SelectTriggerProps {
    /// Child elements to render inside the component
    pub children: Element,
    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,
    /// Size of the trigger
    #[props(default = String::from("default"))]
    pub size: String,
    /// Placeholder text when no value is selected
    #[props(default = String::from("Select an option"))]
    pub placeholder: String,
}

/// Properties for select item
#[derive(Props, PartialEq, Clone)]
pub struct SelectItemProps {
    /// Child elements to render inside the component (label)
    pub children: Element,
    /// Value of the item
    pub value: String,
    /// Whether the item is disabled
    #[props(default)]
    pub disabled: bool,
    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,
}

/// Properties for select label
#[derive(Props, PartialEq, Clone)]
pub struct SelectLabelProps {
    /// Child elements to render inside the component
    pub children: Element,
    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,
}

/// Root Select component
#[component]
pub fn Select(props: SelectProps) -> Element {
    let context = SelectContext::new();
    
    // Initialize with default value if provided
    let mut initial_context = context;
    if let Some(value) = &props.value {
        initial_context.selected_value = Some(value.clone());
    }
    
    use_context_provider(|| Signal::new(initial_context));
    
    let class_str = props.class.clone().unwrap_or_default();
    
    rsx! {
        div { "data-slot": "select", class: class_str, {props.children} }
    }
}

/// SelectGroup component
#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    let mut class = vec![];

    if let Some(custom_class) = &props.class {
        class.push(custom_class.clone());
    }
    
    rsx! {
        div { "data-slot": "select-group", class: class.join(" "), {props.children} }
    }
}

/// SelectValue component (used within trigger)
#[derive(Props, PartialEq, Clone)]
pub struct SelectValueProps {
    /// Placeholder text when no value is selected
    #[props(default = String::from("Select an option"))]
    pub placeholder: String,
    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,
}

/// SelectValue component
#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
    let context = use_context::<Signal<SelectContext>>();
    let selected_label = context.read().selected_label.clone();
    
    let mut class = vec!["line-clamp-1 flex items-center gap-2".to_string()];
    if let Some(custom_class) = &props.class {
        class.push(custom_class.clone());
    }
    
    rsx! {
        span { "data-slot": "select-value", class: class.join(" "),
            if let Some(label) = selected_label {
                {label}
            } else {
                span { class: "text-muted-foreground", {props.placeholder} }
            }
        }
    }
}

/// SelectTrigger component
#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
    let mut context = use_context::<Signal<SelectContext>>();
    let mut trigger_ref = use_signal::<Option<Rc<MountedData>>>(|| None);
    
    let mut class = vec![
        "border-input flex w-fit items-center justify-between gap-2 rounded-md border bg-transparent px-3 py-2 text-sm whitespace-nowrap shadow-xs".to_string(),
        "transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50".to_string(),
        "[&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4".to_string(),
        "focus-visible:border-ring focus-visible:ring-ring/50".to_string(),
        "dark:bg-input/30 dark:hover:bg-input/50".to_string(),
    ];

    // Add size-specific classes
    match props.size.as_str() {
        "sm" => class.push("h-8".to_string()),
        _ => class.push("h-9".to_string()), // default
    }

    if let Some(custom_class) = &props.class {
        class.push(custom_class.clone());
    }

    rsx! {
        button {
            "data-slot": "select-trigger",
            "data-state": if context.read().is_open { "open" } else { "closed" },
            "data-size": props.size,
            r#type: "button",
            class: class.join(" "),
            "aria-haspopup": "true",
            "aria-expanded": if context.read().is_open { "true" } else { "false" },
            onmounted: move |evt| {
                trigger_ref.set(Some(evt.data()));
            },
            onclick: move |_| {
                let toggled = !context.peek().is_open;
                context.write().is_open = toggled;
                if context.peek().is_open {
                    if let Some(node) = trigger_ref.read().as_ref().cloned() {
                        spawn(async move {
                            if let Ok(rect) = node.get_client_rect().await {
                                context.write().position_x = rect.origin.x;
                                context.write().position_y = rect.origin.y;
                                context.write().width = rect.width();
                            }
                        });
                    }
                }
            },
            {props.children}
            div { class: "size-4 opacity-50",
                Icon { icon: LdChevronDown }
            }
        }
    }
}

/// Properties for select content
#[derive(Props, PartialEq, Clone)]
pub struct SelectContentProps {
    /// Child elements to render inside the component
    pub children: Element,
    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,
    /// Position strategy for the content
    #[props(default = String::from("popper"))]
    pub position: String,
}

/// SelectContent component
#[component]
pub fn SelectContent(props: SelectContentProps) -> Element {
    let mut context = use_context::<Signal<SelectContext>>();
    let is_open = context.read().is_open;
    let position_x = context.read().position_x;
    let position_y = context.read().position_y;
    let trigger_width = context.read().width;
    
    let mut class = vec![
        "bg-popover text-popover-foreground relative z-50 min-w-[8rem] origin-[var(--radix-select-content-transform-origin)] overflow-hidden rounded-md border shadow-md".to_string(),
        "data-[state=open]:animate-in data-[state=closed]:animate-out".to_string(),
        "data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0".to_string(),
        "data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95".to_string(),
        "data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2".to_string(),
        "data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2".to_string(),
    ];

    // Add position-specific classes
    if props.position == "popper" {
        class.push("data-[side=bottom]:translate-y-1 data-[side=left]:-translate-x-1 data-[side=right]:translate-x-1 data-[side=top]:-translate-y-1".to_string());
    }

    if let Some(custom_class) = &props.class {
        class.push(custom_class.clone());
    }

    // Calculate position
    let position_style = format!(
        "position: fixed; top: {}px; left: {}px; min-width: {}px; max-height: 300px;",
        position_y, position_x, trigger_width
    );

    if !is_open {
        return rsx!{};
    }

    rsx! {
        AppPortal {
            onclick: move |_| {
                context.write().is_open = false;
            },
            class: "block".to_string(),
        }
        div {
            "data-slot": "select-content",
            "data-state": if is_open { "open" } else { "closed" },
            class: class.join(" "),
            style: position_style,
            onclick: move |e| {
                e.stop_propagation();
            },
            // Scroll up button (optional in real use)
            SelectScrollUpButton {}
            // Viewport for items
            div { class: "p-1 max-height-[300px] overflow-y-auto", {props.children} }
            // Scroll down button (optional in real use)
            SelectScrollDownButton {}
        }
    }
}

/// SelectLabel component
#[component]
pub fn SelectLabel(props: SelectLabelProps) -> Element {
    let mut class = vec!["text-muted-foreground px-2 py-1.5 text-xs".to_string()];

    if let Some(custom_class) = &props.class {
        class.push(custom_class.clone());
    }

    rsx! {
        div { "data-slot": "select-label", class: class.join(" "), {props.children} }
    }
}

/// SelectItem component
#[component]
pub fn SelectItem(props: SelectItemProps) -> Element {
    let mut context = use_context::<Signal<SelectContext>>();
    let is_selected = context.read().selected_value.as_ref() == Some(&props.value);
    
    let mut class = vec![
        "focus:bg-accent focus:text-accent-foreground relative flex w-full cursor-default items-center gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm outline-hidden select-none".to_string(),
        "[&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4".to_string(),
    ];

    if props.disabled {
        class.push("data-[disabled]:pointer-events-none data-[disabled]:opacity-50".to_string());
    }

    if let Some(custom_class) = &props.class {
        class.push(custom_class.clone());
    }

    // Get child text to use as label
    let label_text = if let Ok(element) = rsx!(
        {props.children.clone()}
    ) {
        // Attempt to extract text from the Element, simplified approach
        "".to_string() // In real implementation, extract text from props.children
    } else {
        "".to_string()
    };

    rsx! {
        div {
            "data-slot": "select-item",
            "data-value": props.value,
            "data-disabled": if props.disabled { "true" } else { "false" },
            "aria-selected": if is_selected { "true" } else { "false" },
            role: "option",
            tabindex: "0",
            class: class.join(" "),
            onclick: move |e| {
                if !props.disabled {
                    e.stop_propagation();
                    context.write().is_open = false;
                }
            },
            // Check mark for selected item
            span { class: "absolute right-2 flex size-3.5 items-center justify-center",
                if is_selected {
                    Icon { icon: LdCheck, class: "size-4" }
                }
            }
            // Item text
            span { "data-slot": "select-item-text", {props.children} }
        }
    }
}

/// SelectSeparator component
#[derive(Props, PartialEq, Clone)]
pub struct SelectSeparatorProps {
    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,
}

/// SelectSeparator component
#[component]
pub fn SelectSeparator(props: SelectSeparatorProps) -> Element {
    let mut class = vec!["bg-border pointer-events-none -mx-1 my-1 h-px".to_string()];

    if let Some(custom_class) = &props.class {
        class.push(custom_class.clone());
    }

    rsx! {
        div { "data-slot": "select-separator", class: class.join(" ") }
    }
}

/// SelectScrollUpButton component
#[derive(Props, PartialEq, Clone)]
pub struct SelectScrollButtonProps {
    /// Additional CSS classes to apply
    #[props(default)]
    pub class: Option<String>,
}

/// SelectScrollUpButton component
#[component]
pub fn SelectScrollUpButton(props: SelectScrollButtonProps) -> Element {
    let mut class = vec!["flex cursor-default items-center justify-center py-1".to_string()];

    if let Some(custom_class) = &props.class {
        class.push(custom_class.clone());
    }

    rsx! {
        div { "data-slot": "select-scroll-up-button", class: class.join(" "),
            Icon { icon: LdChevronUp, class: "size-4" }
        }
    }
}

/// SelectScrollDownButton component
#[component]
pub fn SelectScrollDownButton(props: SelectScrollButtonProps) -> Element {
    let mut class = vec!["flex cursor-default items-center justify-center py-1".to_string()];

    if let Some(custom_class) = &props.class {
        class.push(custom_class.clone());
    }

    rsx! {
        div { "data-slot": "select-scroll-down-button", class: class.join(" "),
            Icon { icon: LdChevronDown, class: "size-4" }
        }
    }
}