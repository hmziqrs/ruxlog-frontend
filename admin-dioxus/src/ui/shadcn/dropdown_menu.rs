use dioxus::prelude::*;
use hmziq_dioxus_free_icons::{
    icons::ld_icons::{LdCheck, LdChevronRight, LdCircle},
    Icon,
};

/// Signal for managing dropdown menu open/close state
#[derive(PartialEq)]
pub struct DropdownContext(pub bool);

/// Properties for all dropdown menu components
#[derive(Props, PartialEq, Clone)]
pub struct DropdownMenuProps {
    /// Child elements to render inside the component
    children: Element,
    /// Additional CSS classes to apply
    #[props(default)]
    class: Option<String>,
}

/// Properties for DropdownMenuItem with variant support
#[derive(Props, PartialEq, Clone)]
pub struct DropdownMenuItemProps {
    children: Element,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    inset: bool,
    #[props(default = String::from("default"))]
    variant: String,
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
}

/// Properties for checkbox and radio items
#[derive(Props, PartialEq, Clone)]
pub struct DropdownMenuToggleItemProps {
    children: Element,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    checked: bool,
    #[props(default)]
    onchange: Option<EventHandler<MouseEvent>>,
}

/// Root DropdownMenu component
#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    use_context_provider(|| Signal::new(DropdownContext(false)));
    rsx! {
        div { "data-slot": "dropdown-menu", {props.children} }
    }
}

/// DropdownMenuTrigger component
#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuProps) -> Element {
    let mut open = use_context::<Signal<DropdownContext>>();
    rsx! {
        button {
            "data-slot": "dropdown-menu-trigger",
            class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50 data-[state=open]:bg-accent/50",
            onclick: move |_| {
                let is_open = open.peek().0;
                open.set(DropdownContext(!is_open));
            },
            {props.children}
        }
    }
}

/// DropdownMenuContent component
#[component]
pub fn DropdownMenuContent(props: DropdownMenuProps) -> Element {
    let open = use_context::<Signal<DropdownContext>>();
    let mut class = vec!["bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 min-w-[8rem] overflow-hidden rounded-md border p-1 shadow-md absolute".to_string()];

    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        div {
            "data-slot": "dropdown-menu-content",
            class: class.join(" "),
            style: format!("display: {}", if open.read().0 { "block" } else { "none" }),
            {props.children}
        }
    }
}

/// DropdownMenuItem component
#[component]
pub fn DropdownMenuItem(props: DropdownMenuItemProps) -> Element {
    let mut class = vec!["focus:bg-accent focus:text-accent-foreground relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4".to_string()];

    if props.variant == "destructive" {
        class.push("text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:!text-destructive".to_string());
    }

    if props.inset {
        class.push("pl-8".to_string());
    }

    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        div {
            "data-slot": "dropdown-menu-item",
            "data-inset": props.inset.to_string(),
            "data-variant": props.variant,
            class: class.join(" "),
            onclick: move |e| {
                if let Some(handler) = &props.onclick {
                    handler.call(e);
                }
            },
            {props.children}
        }
    }
}

/// DropdownMenuCheckboxItem component
#[component]
pub fn DropdownMenuCheckboxItem(props: DropdownMenuToggleItemProps) -> Element {
    let mut class = vec!["focus:bg-accent focus:text-accent-foreground relative flex cursor-default items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4".to_string()];

    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        div {
            "data-slot": "dropdown-menu-checkbox-item",
            class: class.join(" "),
            onclick: move |e| {
                if let Some(handler) = &props.onchange {
                    handler.call(e);
                }
            },
            span { class: "pointer-events-none absolute left-2 flex size-3.5 items-center justify-center",
                if props.checked {
                    Icon { icon: LdCheck, class: "size-4" }
                }
            }
            {props.children}
        }
    }
}

/// DropdownMenuRadioItem component
#[component]
pub fn DropdownMenuRadioItem(props: DropdownMenuToggleItemProps) -> Element {
    let mut class = vec!["focus:bg-accent focus:text-accent-foreground relative flex cursor-default items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4".to_string()];

    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        div {
            "data-slot": "dropdown-menu-radio-item",
            class: class.join(" "),
            onclick: move |e| {
                if let Some(handler) = &props.onchange {
                    handler.call(e);
                }
            },
            span { class: "pointer-events-none absolute left-2 flex size-3.5 items-center justify-center",
                if props.checked {
                    Icon { icon: LdCircle, class: "size-2" }
                }
            }
            {props.children}
        }
    }
}

/// DropdownMenuLabel component
#[component]
pub fn DropdownMenuLabel(props: DropdownMenuProps) -> Element {
    let mut class = vec!["px-2 py-1.5 text-sm font-medium".to_string()];

    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        div { "data-slot": "dropdown-menu-label", class: class.join(" "), {props.children} }
    }
}

/// DropdownMenuSeparator component
#[component]
pub fn DropdownMenuSeparator(props: DropdownMenuProps) -> Element {
    let mut class = vec!["bg-border -mx-1 my-1 h-px".to_string()];

    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        div { "data-slot": "dropdown-menu-separator", class: class.join(" "), {props.children} }
    }
}

/// DropdownMenuShortcut component
#[component]
pub fn DropdownMenuShortcut(props: DropdownMenuProps) -> Element {
    let mut class = vec!["text-muted-foreground ml-auto text-xs tracking-widest".to_string()];

    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        span { "data-slot": "dropdown-menu-shortcut", class: class.join(" "), {props.children} }
    }
}

/// DropdownMenuSubTrigger component
#[component]
pub fn DropdownMenuSubTrigger(props: DropdownMenuProps) -> Element {
    let mut class = vec!["focus:bg-accent focus:text-accent-foreground data-[state=open]:bg-accent data-[state=open]:text-accent-foreground flex cursor-default items-center rounded-sm px-2 py-1.5 text-sm outline-hidden select-none".to_string()];

    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        div { "data-slot": "dropdown-menu-sub-trigger", class: class.join(" "),
            {props.children}
            Icon { icon: LdChevronRight, class: "ml-auto size-4" }
        }
    }
}

/// DropdownMenuSubContent component
#[component]
pub fn DropdownMenuSubContent(props: DropdownMenuProps) -> Element {
    let mut class = vec!["bg-popover text-popover-foreground z-50 min-w-[8rem] overflow-hidden rounded-md border p-1 shadow-lg data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2".to_string()];

    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        div { "data-slot": "dropdown-menu-sub-content", class: class.join(" "), {props.children} }
    }
}
