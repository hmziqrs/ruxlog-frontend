#![allow(non_snake_case)]
use std::rc::Rc;

use super::state::CommandContext;
use super::props::*;
use dioxus::prelude::*;


#[component]
pub fn Command(props: CommandRootProps) -> Element {
    use_context_provider(|| Signal::new(CommandContext::new()));

    let context = use_context::<Signal<CommandContext>>();

    rsx! {
        div { "cmdk-root": "", ..props.attributes,
            if props.label.is_some() {
                label { r#for: context.read().ids.input.as_ref(), {props.label.unwrap()} }
            }
            {props.children}
        }
    }
}




#[component]
pub fn CommandInput(props: CommandInputProps) -> Element {
    let mut context = use_context::<Signal<CommandContext>>();
    let mut input_ref = use_signal(|| None as Option<Rc<MountedData>>);
    let context_read = context.read();
    let input_id = context_read.ids.input.as_ref(); 

    // Focus input when Command is opened
    use_effect(move || {
        if let Some(input) = input_ref() {
            spawn(async move {
                _ = input.set_focus(true).await;
            });
        }
    });

    rsx! {
        div { class: "flex items-center border-b px-3",
            input {
                id: input_id,
                onmounted: move |cx| {
                    input_ref.set(Some(cx.data()));
                },
                placeholder: "{props.placeholder}",
                value: context_read.search.as_ref(),
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

#[component]
pub fn CommandList(children: Element) -> Element {
    rsx! {
        div { role: "listbox", {children} }
    }
}



#[component]
pub fn CommandItem(props: CommandItemProps) -> Element {
    let context = use_context::<Signal<CommandContext>>();
    let search_term = context.read().search.clone();

    let display_item = if let Some(val) = &props.value {
        search_term.is_empty() || val.to_lowercase().contains(&search_term.to_lowercase())
    } else {
        search_term.is_empty()
    };

    if !display_item {
        return rsx! {
            div {}
        };
    }

    let is_active = false;

    rsx! {
        div {
            class: "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none aria-selected:bg-accent aria-selected:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
            "data-disabled": if props.disabled { Some("") } else { None },
            role: "option",
            "aria-selected": is_active,
            tabindex: if is_active { Some("0") } else { None },
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

#[component]
pub fn CommandSeparator() -> Element {
    rsx! {
        div { class: "-mx-1 h-px bg-border" }
    }
}

#[component]
pub fn CommandLoading(children: Element) -> Element {
    rsx! {
        div { class: "py-6 text-center text-sm", {children} }
    }
}


#[component]
pub fn CommandEmpty(children: Element) -> Element {
    rsx! {
        div { class: "py-6 text-center text-sm", {children} }
    }
}