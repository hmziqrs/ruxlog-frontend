#![allow(non_snake_case)]
use std::rc::Rc;

use super::props::*;
use super::state::CommandContext;
use dioxus::logger::tracing;
use dioxus::prelude::*;

#[component]
pub fn Command(props: CommandRootProps) -> Element {
    use_context_provider(|| Signal::new(CommandContext::new()));

    let mut context = use_context::<Signal<CommandContext>>();

    let on_key = move |evt: KeyboardEvent| {
        tracing::info!("Key pressed: {:?}", evt.key());
        if evt.key() == Key::Escape {
            context.write().set_open(false);
        }
    };

    rsx! {
        div { onkeydown: on_key, "cmdk-root": "", ..props.attributes,
            if props.label.is_some() {
                label {
                    style: "position:absolute;width:1px;height:1px;padding:0;margin:-1px;overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;border-width:0",
                    id: context.read().ids.label.as_ref(),
                    r#for: context.read().ids.input.as_ref(),
                    {props.label.unwrap()}
                }
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
    let list_id = context_read.ids.list.as_ref();
    let label_id = context_read.ids.label.as_ref();

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
                "cmdk-input": "",
                "auto-complete": "off",
                "auto-correct": "off",
                "spell-check": false,
                "aria-autocomplete": "list",
                role: "combobox",
                "aria-expanded": true,
                "aria-controls": list_id,
                "aria-labelledby": label_id,
                r#type: "text",
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
    let context = use_context::<Signal<CommandContext>>();
    let context_read = context.read();
    
    rsx! {
        div {..props.attributes,
            if let Some(heading) = props.heading {
                div { "cmdk-group-heading": "", "aria-hidden": "true", {heading} }
            }
            {props.children}
        }
    }
}

#[component]
pub fn CommandSeparator(props: CommandSeparatorProps) -> Element {
    rsx! {

        div { ..props.attributes }
    }
}

#[component]
pub fn CommandLoading(props: CommandLoadingProps) -> Element {
    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[component]
pub fn CommandEmpty(props: CommandEmptyProps) -> Element {
    rsx! {
        div { ..props.attributes,{props.children} }
    }
}
