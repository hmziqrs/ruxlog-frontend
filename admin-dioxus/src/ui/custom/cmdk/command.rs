use std::rc::Rc;

use super::command_score::command_score;
use crate::ui::custom::AppPortal;
use dioxus::prelude::*;
use hmziq_dioxus_free_icons::{icons::ld_icons::LdX, Icon};
use wasm_bindgen::JsValue;

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
        return rsx! {};
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
    let mut list_ref = use_signal(|| None as Option<Rc<MountedData>>);
    let mut list_inner_ref = use_signal(|| None as Option<Rc<MountedData>>);
    let base_class =
        "flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground";
    let mut class = vec![base_class];

    if let Some(custom_class) = &props.class {
        class.push(custom_class);
    }

    // let on_keydown = move |e: Event<KeyboardData>| {
    //     if e.modifiers().ctrl_key() && props.vim_bindings {
    //         match e.key() {
    //             "n" | "j" => {
    //                 e.prevent_default();
    //                 update_selected_by_item(&state, 1, props.loop_nav);
    //             }
    //             "p" | "k" => {
    //                 e.prevent_default();
    //                 update_selected_by_item(&state, -1, props.loop_nav);
    //             }
    //             _ => {}
    //         }
    //     } else {
    //         match e.key() {
    //             "ArrowDown" => {
    //                 e.prevent_default();
    //                 if e.modifiers().alt_key() {
    //                     update_selected_by_group(&state, 1, props.loop_nav);
    //                 } else {
    //                     update_selected_by_item(&state, 1, props.loop_nav);
    //                 }
    //             }
    //             "ArrowUp" => {
    //                 e.prevent_default();
    //                 if e.modifiers().alt_key() {
    //                     update_selected_by_group(&state, -1, props.loop_nav);
    //                 } else {
    //                     update_selected_by_item(&state, -1, props.loop_nav);
    //                 }
    //             }
    //             "Home" => {
    //                 e.prevent_default();
    //                 update_selected_to_index(&state, 0);
    //             }
    //             "End" => {
    //                 e.prevent_default();
    //                 update_selected_to_last(&state);
    //             }
    //             "Enter" => {
    //                 e.prevent_default();
    //                 if let Some(value) = state.read().value.clone() {
    //                     if let Some(handler) = &props.onvalue_change {
    //                         handler.call(value);
    //                     }
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
    // };

    rsx! {
        div {
            "data-slot": "command",
            "role": "combobox",
            tabindex: "-1",
            "aria-label": props.label.clone().unwrap_or_else(|| "Command Menu".to_string()),
            class: class.join(" "),
            // onkeydown: on_keydown,
            {props.children}
        }
    }
}

// Helper functions for selection management
fn update_selected_by_item(mut state: Signal<CommandState>, change: i32, should_loop: bool) {
    let items = get_valid_items();
    let current_value = state.read().value.clone();

    let current_index = items.iter().position(|i| Some(i) == Some(&current_value));

    let new_index = match current_index {
        Some(idx) => {
            let new_idx = (idx as i32 + change) as i32;
            if should_loop {
                if new_idx < 0 {
                    items.len() - 1
                } else if new_idx >= items.len() as i32 {
                    0
                } else {
                    new_idx as usize
                }
            } else {
                new_idx.max(0).min(items.len() as i32 - 1) as usize
            }
        }
        None => {
            if change > 0 {
                0
            } else {
                items.len() - 1
            }
        }
    };

    if let Some(value) = items.get(new_index) {
        state.write().value = value.clone();
        scroll_item_into_view(value);
    }
}

fn update_selected_by_group(mut state: Signal<CommandState>, change: i32, should_loop: bool) {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // First find the current group
    let current_value = state.read().value.clone();
    let mut current_group = None;

    let selector = format!("[data-slot='command-item'][data-value='{}']", current_value);
    if let Some(element) = document.query_selector(&selector).unwrap() {
        current_group = element.closest("[cmdk-group]").ok().flatten();
    }
    // if let Some(value) = current_value {
    //     let selector = format!("[data-slot='command-item'][data-value='{}']", value);
    //     if let Some(element) = document.query_selector(&selector).unwrap() {
    //         current_group = element.closest("[cmdk-group]").ok().flatten();
    //     }
    // }

    // Get all groups
    let groups = document.query_selector_all("[cmdk-group]").unwrap();
    let mut group_vec = vec![];
    for i in 0..groups.length() {
        if let Some(group) = groups.get(i) {
            group_vec.push(group);
        }
    }

    // Find the next/previous group
    if let Some(current) = current_group {
        let current_idx = group_vec
            .iter()
            .position(|g| *g.dyn_ref::<web_sys::Element>().unwrap() == current);
        if let Some(idx) = current_idx {
            let new_idx = if should_loop {
                (idx as i32 + change).rem_euclid(group_vec.len() as i32) as usize
            } else {
                ((idx as i32 + change) as usize).min(group_vec.len() - 1)
            };

            // Get the first valid item in the new group
            if let Some(new_group) = group_vec.get(new_idx) {
                let element = new_group
                    .dyn_ref::<web_sys::Element>()
                    .ok_or_else(|| JsValue::from_str("Node is not an Element"))
                    .unwrap();

                if let Some(items) = element
                    .query_selector_all("[data-slot='command-item']")
                    .ok()
                {
                    for i in 0..items.length() {
                        if let Some(item) = items.get(i) {
                            if let Some(element) = item.dyn_ref::<web_sys::HtmlElement>() {
                                if element.get_attribute("data-disabled")
                                    != Some("true".to_string())
                                {
                                    if let Some(value) = element.get_attribute("data-value") {
                                        state.write().value = value.clone();
                                        scroll_item_into_view(&state.read().value);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn update_selected_to_index(mut state: Signal<CommandState>, index: usize) {
    let items = get_valid_items();
    if let Some(value) = items.get(index) {
        state.write().value = value.clone();
        scroll_item_into_view(value);
    }
}

fn update_selected_to_last(mut state: Signal<CommandState>) {
    let items = get_valid_items();
    if let Some(value) = items.last() {
        state.write().value = value.clone();
        scroll_item_into_view(value);
    }
}

fn get_valid_items() -> Vec<String> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Get all command items that aren't disabled
    let selector = "[data-slot='command-item']:not([data-disabled='true'])";
    let items = document.query_selector_all(selector).unwrap();

    let mut valid_items = Vec::new();
    for i in 0..items.length() {
        if let Some(item) = items.get(i) {
            if let Some(element) = item.dyn_ref::<web_sys::HtmlElement>() {
                if let Some(value) = element.get_attribute("data-value") {
                    valid_items.push(value);
                }
            }
        }
    }

    valid_items
}

fn scroll_item_into_view(value: &str) {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let selector = format!("[data-slot='command-item'][data-value='{}']", value);
    if let Some(element) = document.query_selector(&selector).unwrap() {
        if let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
            html_element.scroll_into_view_with_bool(true);

            // Also scroll the group heading into view if this item is in a group
            if let Some(group) = element.closest("[cmdk-group-items]").unwrap() {
                if let Some(heading) = group
                    .parent_element()
                    .and_then(|g| g.query_selector("[cmdk-group-heading]").ok())
                    .flatten()
                {
                    if let Some(html_heading) = heading.dyn_ref::<web_sys::HtmlElement>() {
                        html_heading.scroll_into_view_with_bool(true);
                    }
                }
            }
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
        return rsx! {};
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
        return rsx! {};
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

#[derive(Props, PartialEq, Clone)]
pub struct CommandRadioGroupProps {
    children: Element,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    value: Option<String>,
    #[props(default)]
    onvalue_change: Option<EventHandler<String>>,
}

#[component]
pub fn CommandRadioGroup(props: CommandRadioGroupProps) -> Element {
    let selected = use_signal(|| props.value.clone());

    rsx! {
        div { "data-slot": "command-radio-group", role: "radiogroup", {props.children} }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandRadioItemProps {
    children: Element,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    value: String,
    #[props(default)]
    disabled: bool,
}

#[component]
pub fn CommandRadioItem(props: CommandRadioItemProps) -> Element {
    let base_class = "relative flex cursor-default select-none items-center rounded-sm py-1.5 pr-2 pl-8 text-sm outline-none aria-selected:bg-accent aria-selected:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
    let mut class = vec![base_class];

    if let Some(custom_class) = &props.class {
        class.push(custom_class);
    }

    let state = use_context::<Signal<CommandState>>();
    let selected = state.read().value == props.value;

    rsx! {
        div {
            "data-slot": "command-radio-item",
            "data-value": props.value.clone(),
            "data-disabled": props.disabled.to_string(),
            role: "radio",
            class: class.join(" "),
            "aria-checked": selected.to_string(),
            span { class: "pointer-events-none absolute left-2 flex h-3.5 w-3.5 items-center justify-center",
                if selected {
                    Icon {
                        icon: hmziq_dioxus_free_icons::icons::ld_icons::LdCircle {
                        },
                        class: "h-2 w-2 fill-current",
                    }
                }
            }
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandCheckboxItemProps {
    children: Element,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    checked: bool,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    onvalue_change: Option<EventHandler<bool>>,
}

#[component]
pub fn CommandCheckboxItem(props: CommandCheckboxItemProps) -> Element {
    let base_class = "relative flex cursor-default select-none items-center rounded-sm py-1.5 pr-2 pl-8 text-sm outline-none aria-selected:bg-accent aria-selected:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
    let mut class = vec![base_class];

    if let Some(custom_class) = &props.class {
        class.push(custom_class);
    }

    rsx! {
        div {
            "data-slot": "command-checkbox-item",
            "data-disabled": props.disabled.to_string(),
            role: "checkbox",
            class: class.join(" "),
            "aria-checked": props.checked.to_string(),
            onclick: move |_| {
                if !props.disabled {
                    if let Some(handler) = &props.onvalue_change {
                        handler.call(!props.checked);
                    }
                }
            },
            span { class: "pointer-events-none absolute left-2 flex h-3.5 w-3.5 items-center justify-center",
                if props.checked {
                    Icon {
                        icon: hmziq_dioxus_free_icons::icons::ld_icons::LdCheck {
                        },
                        class: "h-4 w-4",
                    }
                }
            }
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandLoadingProps {
    children: Element,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    progress: Option<i32>,
    #[props(default = String::from("Loading..."))]
    label: String,
}

#[component]
pub fn CommandLoading(props: CommandLoadingProps) -> Element {
    let base_class = "py-6 text-center text-sm";
    let mut class = vec![base_class];

    if let Some(custom_class) = &props.class {
        class.push(custom_class);
    }

    rsx! {
        div {
            class: class.join(" "),
            "data-slot": "command-loading",
            role: "progressbar",
            "aria-valuenow": props.progress.map(|p| p.to_string()),
            "aria-valuemin": "0",
            "aria-valuemax": "100",
            "aria-label": props.label,
            div { "aria-hidden": "true", {props.children} }
        }
    }
}
