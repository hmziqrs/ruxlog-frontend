#![allow(non_snake_case)]
use std::rc::Rc;

use dioxus::prelude::*;
use super::state::CommandContext;

#[derive(Props, PartialEq, Clone)]
pub struct CommandRootProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    children: Element,
}

#[component]
pub fn Command(props: CommandRootProps) -> Element {
    use_context_provider(|| Signal::new(CommandContext::new()));

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

// Command Input Props
#[derive(Props, PartialEq, Clone)]
pub struct CommandInputProps {
    #[props(default = "Type a command or search...".to_string())]
    placeholder: String,

    #[props(optional)]
    children: Element,
}

// Command Input component
#[component]
pub fn CommandInput(props: CommandInputProps) -> Element {
    let mut context = use_context::<Signal<CommandContext>>();
    let mut input_ref = use_signal(|| None as Option<Rc<MountedData>>);

    // Focus input when Command is opened
    use_effect(move || {
        // if context.read().is_open {
            if let Some(input) = input_ref() {
                spawn(async move {
                    _ = input.set_focus(true).await;
                });
            // }
        }
    });

    rsx! {
        div { class: "flex items-center border-b px-3",
            input {
                onmounted: move |cx| {
                    input_ref.set(Some(cx.data()));
                },
                class: "flex h-11 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50",
                placeholder: "{props.placeholder}",
                value: context.read().search.as_ref(),
                oninput: move |evt| {
                    let new_value = evt.value();
                    context.write().set_search(new_value.clone());
                    context.write().set_active_index(0);
                },
                // Keyboard navigation
                onkeydown: move |evt| {
                    let key = evt.key();
                    let mut ctx = context.write();
                    match key {
                        Key::ArrowDown => {
                            ctx.active_index += 1;
                        }
                        Key::ArrowUp => {
                            if ctx.active_index > 0 {
                                ctx.active_index -= 1;
                            }
                        }
                        Key::Enter => {}
                        Key::Escape => {
                            ctx.set_open(false);
                        }
                        _ => {}
                    }
                },
            }
            {props.children}
        }
    }
}

// Command List component
#[component]
pub fn CommandList(children: Element) -> Element {
    // Provide a way for CommandItem to know its index
    rsx! {
        div {
            class: "max-h-[300px] overflow-y-auto overflow-x-hidden",
            role: "listbox",
            {children} // Render the potentially wrapped child
        }
    }
}

// Command Item Props
#[derive(Props, PartialEq, Clone)]
pub struct CommandItemProps {
    children: Element,
    value: Option<String>,
    
    #[props(optional)]
    on_select: Option<EventHandler<()>>, // Use EventHandler for callbacks
    
    #[props(default = false)]
    disabled: bool,
}

// Command Item component
#[component]
pub fn CommandItem(props: CommandItemProps) -> Element {
    let context = use_context::<Signal<CommandContext>>();
    let search_term = context.read().search.clone();

    // Basic filtering logic (can be improved)
    let display_item = if let Some(val) = &props.value {
        search_term.is_empty() || val.to_lowercase().contains(&search_term.to_lowercase())
    } else {
        // If no value, maybe check children text content? For now, show if search is empty.
        search_term.is_empty()
    };

    if !display_item {
        return rsx! {
            div {}
        };     }

    let is_active = false;

    rsx! {
        div {
            // Add necessary classes for styling, hover, selected states
            class: "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none aria-selected:bg-accent aria-selected:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
            "data-disabled": if props.disabled { Some("") } else { None },
            role: "option",
            "aria-selected": is_active,
            tabindex: if is_active { Some("0") } else { None },
            // Highlight/focus management
            autofocus: is_active,
            onclick: move |_| {
                if !props.disabled {
                    if let Some(handler) = &props.on_select {
                        handler.call(());
                    }
                }
            },
            // Keyboard selection (Enter)
            onkeydown: move |evt| {
                if is_active && evt.key() == Key::Enter && !props.disabled {
                    if let Some(handler) = &props.on_select {
                        handler.call(());
                    }
                }
            },
            {props.children}
        }
    }
}

// Command Empty component (shown when list is empty)
#[component]
pub fn CommandEmpty(children: Element) -> Element {
    // Logic to determine if the list is actually empty would be needed,
    // potentially by inspecting siblings or via context.
    // For now, it just renders its children.
    rsx! {
        div {
            // Add necessary classes for styling
            class: "py-6 text-center text-sm",
            {children}
        }
    }
    // Placeholder: Add logic to only render when no items match search
}

// Command Group Props
#[derive(Props, PartialEq, Clone)]
pub struct CommandGroupProps {
    /// Optional heading for the group
    heading: Option<String>,
    /// Items within the group
    children: Element,
}

// Command Group component
#[component]
pub fn CommandGroup(props: CommandGroupProps) -> Element {
    rsx! {
        div { class: "overflow-hidden p-1 text-foreground [&_[cmdk-group-heading]]:px-2 [&_[cmdk-group-heading]]:py-1.5 [&_[cmdk-group-heading]]:text-xs [&_[cmdk-group-heading]]:font-medium [&_[cmdk-group-heading]]:text-muted-foreground",
            if let Some(h) = props.heading {
                div { class: "[cmdk-group-heading]", "{h}" }
            }
            {props.children}
        }
    }
}

// Command Separator component (optional)
#[component]
pub fn CommandSeparator() -> Element {
    rsx! {
        div { class: "-mx-1 h-px bg-border" }
    }
}


// Command Loading component (Placeholder for loading state)
#[component]
pub fn CommandLoading(children: Element) -> Element {
    // Similar to CommandEmpty, logic is needed to determine when to show this.
    // This might involve checking a loading state signal in the context.
    rsx! {
        div {
            // Add necessary classes for styling
            class: "py-6 text-center text-sm",
            // Example: "Loading..." or a spinner component
            {children}
        }
    }
    // Placeholder: Add logic to only render during loading state
}

