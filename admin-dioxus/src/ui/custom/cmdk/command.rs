use dioxus::prelude::*;
use crate::ui::custom::AppPortal;
use super::command_score::command_score;
use hmziq_dioxus_free_icons::{Icon, icons::ld_icons::LdX};

// Context and state types
#[derive(PartialEq, Clone)]
pub struct CommandState {
    search: String,
    value: String,
    selected_id: Option<String>,
    filtered: FilteredState,
}

#[derive(PartialEq, Clone)]
pub struct FilteredState {
    count: usize,
    items: im::HashMap<String, f32>,
    groups: im::HashSet<String>,
}

impl Default for CommandState {
    fn default() -> Self {
        Self {
            search: String::new(),
            value: String::new(),
            selected_id: None,
            filtered: FilteredState {
                count: 0,
                items: im::HashMap::new(),
                groups: im::HashSet::new(),
            },
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandProps {
    children: Element,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    label: Option<String>,
    #[props(default = true)]
    should_filter: bool,
    #[props(default)]
    filter: Option<fn(&str, &str, Option<&[String]>) -> f32>,
    #[props(default)]
    value: Option<String>,
    #[props(default)]
    default_value: Option<String>,
    #[props(default)]
    onvalue_change: Option<EventHandler<String>>,
    #[props(default)]
    loop_nav: bool,
    #[props(default)]
    disable_pointer: bool,
    #[props(default = true)]
    vim_bindings: bool,
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandDialogProps {
    children: Element,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    open: bool,
    #[props(default)]
    label: Option<String>,
}

#[component]
pub fn CommandDialog(props: CommandDialogProps) -> Element {
    let mut open = use_signal(|| props.open);

    if !*open.read() {
        return rsx!{};
    }

    rsx! {
        AppPortal {
            div {
                class: "fixed inset-0 z-50 bg-black/50",
                onclick: move |_| open.set(false),
            }
            div { class: "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg duration-200 sm:rounded-lg md:w-full",
                Command { class: props.class.clone(), label: props.label.clone(), {props.children} }
            }
        }
    }
}

#[component]
pub fn Command(props: CommandProps) -> Element {
    let state = use_signal(CommandState::default);
    let base_class = "flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground";
    let mut class = vec![base_class];
    
    if let Some(custom_class) = &props.class {
        class.push(custom_class);
    }

    rsx! {
        div {
            "data-slot": "command",
            "role": "combobox",
            "aria-label": props.label.clone().unwrap_or_else(|| "Command Menu".to_string()),
            class: class.join(" "),
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandInputProps {
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    value: Option<String>,
    #[props(default)]
    placeholder: Option<String>,
    #[props(default)]
    onvalue_change: Option<EventHandler<String>>,
}

#[component]
pub fn CommandInput(props: CommandInputProps) -> Element {
    let mut state = use_context::<Signal<CommandState>>();
    let base_class = "flex h-11 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50";
    let mut class = vec![base_class];

    if let Some(custom_class) = &props.class {
        class.push(custom_class);
    }

    let controlled = props.value.is_some();
    let value = if controlled {
        props.value.clone().unwrap_or_default()
    } else {
        state.read().search.clone()
    };

    rsx! {
        div {
            class: "flex items-center border-b px-3",
            "data-slot": "command-input",
            input {
                class: class.join(" "),
                "type": "text",
                role: "combobox",
                value,
                placeholder: props.placeholder.clone().unwrap_or_default(),
                oninput: move |e| {
                    let value = e.value();
                    if !controlled {
                        state.write().search = value.clone();
                    }
                    if let Some(handler) = &props.onvalue_change {
                        handler.call(value);
                    }
                },
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandListProps {
    children: Element,
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn CommandList(props: CommandListProps) -> Element {
    let base_class = "max-h-[300px] overflow-y-auto overflow-x-hidden";
    let mut class = vec![base_class];
    
    if let Some(custom_class) = &props.class {
        class.push(custom_class);
    }

    rsx! {
        div {
            class: class.join(" "),
            "data-slot": "command-list",
            role: "listbox",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandItemProps {
    children: Element,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    value: Option<String>,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    onselect: Option<EventHandler<String>>,
    #[props(default)]
    keywords: Option<Vec<String>>,
}

#[component]
pub fn CommandItem(props: CommandItemProps) -> Element {
    let mut state = use_context::<Signal<CommandState>>();
    let base_class = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none aria-selected:bg-accent aria-selected:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
    let mut class = vec![base_class];
    
    if let Some(custom_class) = &props.class {
        class.push(custom_class);
    }

    let value = props.value.clone().unwrap_or_else(|| "".to_string());
    let keywords = props.keywords.clone().unwrap_or_default();
    
    let score = if !state.read().search.is_empty() {
        command_score(&value, &state.read().search, Some(&keywords))
    } else {
        1.0
    };

    // Don't render if filtered out
    if score == 0.0 {
        return rsx!{};
    }

    let selected = state.read().value == value;

    rsx! {
        div {
            class: class.join(" "),
            "data-slot": "command-item",
            "data-value": value.clone(),
            "data-disabled": props.disabled.to_string(),
            role: "option",
            "aria-selected": selected.to_string(),
            onclick: move |_| {
                if !props.disabled {
                    state.write().value = value.clone();
                    if let Some(handler) = &props.onselect {
                        handler.call(value.clone());
                    }
                }
            },
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandEmptyProps {
    children: Element,
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn CommandEmpty(props: CommandEmptyProps) -> Element {
    let state = use_context::<Signal<CommandState>>();
    
    if state.read().filtered.count > 0 {
        return rsx!{};
    }

    let base_class = "py-6 text-center text-sm";
    let mut class = vec![base_class];
    
    if let Some(custom_class) = &props.class {
        class.push(custom_class);
    }

    rsx! {
        div { class: class.join(" "), "data-slot": "command-empty", {props.children} }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandGroupProps {
    children: Element,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    heading: Option<Element>,
    #[props(default)]
    value: Option<String>,
}

#[component]
pub fn CommandGroup(props: CommandGroupProps) -> Element {
    let base_class = "overflow-hidden p-1 [&_[cmdk-group-heading]]:px-2 [&_[cmdk-group-heading]]:py-1.5 [&_[cmdk-group-heading]]:text-xs [&_[cmdk-group-heading]]:font-medium [&_[cmdk-group-heading]]:text-muted-foreground";
    let mut class = vec![base_class];
    
    if let Some(custom_class) = &props.class {
        class.push(custom_class);
    }

    rsx! {
        div { class: class.join(" "), "data-slot": "command-group",
            if let Some(heading) = props.heading {
                div { "cmdk-group-heading": "", {heading} }
            }
            div { "cmdk-group-items": "", {props.children} }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandSeparatorProps {
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn CommandSeparator(props: CommandSeparatorProps) -> Element {
    let base_class = "-mx-1 h-px bg-border";
    let mut class = vec![base_class];
    
    if let Some(custom_class) = &props.class {
        class.push(custom_class);
    }

    rsx! {
        div {
            class: class.join(" "),
            "data-slot": "command-separator",
            role: "separator",
        }
    }
}