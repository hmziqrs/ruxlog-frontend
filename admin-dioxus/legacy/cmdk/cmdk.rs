#![allow(non_snake_case)]
use std::rc::Rc;

use crate::ui::custom::cmdk::state::{CommandGroupContext, CommandItemContext};

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
                ..props.attributes,
            }
            {props.children}
        }
    }
}

#[component]
pub fn CommandList(props: CommandListProps) -> Element {
    rsx! {
        div { role: "listbox", ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq)]
pub struct GroupIdContext(String);

#[component]
pub fn CommandGroup(props: CommandGroupProps) -> Element {
    let group_id: Signal<GroupIdContext> = use_context_provider(|| Signal::new(GroupIdContext(CommandGroupContext::generate_id())));
    let mut context: Signal<CommandContext> = use_context::<Signal<CommandContext>>();
    
    rsx! {
        div {
            id: group_id.read().0.as_ref(),
            onmounted: move |cx| {
                let group = CommandGroupContext::new(
                    group_id.peek().0.clone(),
                    cx.data().clone(),
                );
                context.write().add_group(group);
            },
            ..props.attributes,
            if let Some(heading) = props.heading {
                div { "cmdk-group-heading": "", "aria-hidden": "true", {heading} }
            }
            {props.children}
        }
    }
}

#[component]
pub fn CommandItem(props: CommandItemProps) -> Element {
    let group_id = use_context::<Signal<GroupIdContext>>();
    let item_id = use_signal(|| CommandItemContext::generate_id());
    let mut item_index = use_signal::<Option<i32>>(|| None);


    let mut context = use_context::<Signal<CommandContext>>();

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

    let index = item_index.read();

    rsx! {
        div {
            "cmdk-index": if index.is_some() { index.unwrap().to_string() },
            id: item_id.read().as_ref(),
            onmounted: move |cx| {
                let len = context.peek().item_indexer.len();
                item_index.set(Some(len.clone() as i32));
                context.write().item_indexer.push(len);
            },
            "emdata-disabled": if props.disabled { Some("") } else { None },
            role: "option",
            "aria-selected": is_active,
            tabindex: "0",
            // tabindex: if is_active { Some("0") } else { None },
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
            ..props.attributes,
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
