#![allow(non_snake_case)]
use std::rc::Rc;

use dioxus::prelude::*;

// Context to share state between Command components
#[derive(Clone, PartialEq)]
struct CommandContext {
    search: String,
    is_open: bool,
}

impl CommandContext {
    fn new() -> Self {
        Self {
            search: String::new(),
            is_open: false,
        }
    }

    fn set_search(&mut self, search: String) {
        self.search = search;
    }

    fn set_open(&mut self, is_open: bool) {
        self.is_open = is_open;
    }

    fn toggle_open(&mut self) {
        self.is_open = !self.is_open;
    }
}

// Main Command component
#[component]
pub fn Command(children: Element) -> Element {
    use_context_provider(|| Signal::new(CommandContext::new()));

    rsx! {
        div {
            // Add necessary classes for styling
            class: "flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground",
            {children}
        }
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
    // let current_value: String = ;

    rsx! {
        div {
            // Add search icon if needed
            class: "flex items-center border-b px-3",
            // Icon placeholder:
            // svg { /* Search Icon */ }
            input {
                // Add necessary classes for styling
                class: "flex h-11 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50",
                placeholder: "{props.placeholder}",
                value: context.read().search.as_ref(),
                oninput: move |evt| {
                    let new_value = evt.value();
                    context.write().set_search(new_value.clone());
                },
            }
            {props.children}
        }
    }
}

// Command List component
#[component]
pub fn CommandList(children: Element) -> Element {
    rsx! {
        div {
            // Add necessary classes for styling
            class: "max-h-[300px] overflow-y-auto overflow-x-hidden",
            {children}
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

    rsx! {
        div {
            // Add necessary classes for styling, hover, selected states
            class: "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none aria-selected:bg-accent aria-selected:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
            "data-disabled": if props.disabled { Some("") } else { None },
            // Add accessibility attributes like role="option"
            onclick: move |_| {
                if !props.disabled {
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
        div {
            // Add necessary classes for styling
            class: "overflow-hidden p-1 text-foreground [&_[cmdk-group-heading]]:px-2 [&_[cmdk-group-heading]]:py-1.5 [&_[cmdk-group-heading]]:text-xs [&_[cmdk-group-heading]]:font-medium [&_[cmdk-group-heading]]:text-muted-foreground",
            // Add accessibility attributes like role="group"
            if let Some(h) = props.heading {
                div {
                    // Class for the heading itself
                    class: "[cmdk-group-heading]",
                    "{h}"
                }
            }
            {props.children}
        }
    }
}

// Command Separator component (optional)
#[component]
pub fn CommandSeparator() -> Element {
    rsx! {
        div {
            // Add necessary classes for styling
            class: "-mx-1 h-px bg-border",
                // Add accessibility attributes like role="separator"
        }
    }
}

// // Command Dialog Props
// #[derive(Props, PartialEq, Clone)]
// pub struct CommandDialogProps {
//     /// Content to display within the Command component inside the dialog
//     children: Element,
//     /// Controls the open state of the dialog
//     open: ReadOnlySignal<bool>,
//     /// Optional title for the dialog
//     title: Option<String>,
//     /// Optional description for the dialog
//     description: Option<String>,
// }

// // Command Dialog component (Wraps Command in a Dialog)
// #[component]
// pub fn CommandDialog(props: CommandDialogProps) -> Element {
//     // This component assumes a Dialog component structure like shadcn/ui.
//     // You might need to adjust based on your actual Dialog implementation.
//     rsx! {
//         // Dialog { // Replace with your actual Dialog component
//         //     open: props.open,
//         //     DialogContent {
//         //         class: "overflow-hidden p-0 shadow-lg", // Example classes
//         //         if let Some(t) = props.title {
//         //             DialogHeader {
//         //                 DialogTitle { "{t}" }
//         //                 if let Some(d) = props.description {
//         //                     DialogDescription { "{d}" }
//         //                 }
//         //             }
//         //         }
//         //         Command {
//         //             // Pass necessary props or context if needed
//         //             {props.children}
//         //         }
//         //     }
//         // }
//         // Placeholder implementation until Dialog is available:
//         if props.open.read() {
//             div {
//                 // Basic styling for a modal overlay
//                 class: "fixed inset-0 z-50 bg-black/50 flex items-center justify-center",
//                 onclick: move |_| props.open.set(false), // Close on overlay click
//                 div {
//                     class: "bg-background rounded-lg shadow-lg w-full max-w-lg p-0",
//                     onclick: |evt| evt.stop_propagation(), // Prevent closing when clicking inside dialog
//                     if let Some(t) = &props.title {
//                         h2 { class: "text-lg font-semibold p-4 border-b", "{t}" }
//                         if let Some(d) = &props.description {
//                             p { class: "text-sm text-muted-foreground p-4 pt-0", "{d}" }
//                         }
//                     }
//                     // Render the actual Command content passed as children
//                     {props.children}
//                 }
//             }
//         }
//     }
// }

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

// Example Usage (can be removed or moved to a different file/component)
/*
#[component]
fn App() -> Element {
    rsx! {
        Command {
            CommandInput {}
            CommandList {
                CommandEmpty { "No results found." }
                CommandGroup {
                    heading: "Suggestions",
                    CommandItem { value: "calendar", "Calendar" }
                    CommandItem { value: "search", "Search Emoji" }
                    CommandItem { value: "calculator", "Calculator" }
                }
                CommandSeparator {}
                CommandGroup {
                    heading: "Settings",
                    CommandItem { value: "profile", "Profile" }
                    CommandItem { value: "billing", "Billing" }
                    CommandItem { value: "settings", "Settings" }
                }
            }
        }
    }
}
*/

