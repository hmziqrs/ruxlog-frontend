#![allow(non_snake_case)]
use crate::components::command::{
    context::{use_command_context, CommandContext, GroupData, ItemData},
    state::{command_score, CommandFilter, CommandState, FilteredState},
    utils::{
        filter_and_sort_items, get_selected_item_element, get_valid_items, scroll_item_into_view,
        use_unique_id, SR_ONLY_STYLE,
    },
};
use dioxus::prelude::*;
use dioxus_signals::*;
use gloo_console::log;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent};

#[derive(Props, Clone, PartialEq)]
pub struct CommandProps {
    children: Element<'_>,
    #[props(default)]
    value: Option<String>, // Controlled value
    #[props(default)]
    default_value: Option<String>,
    #[props(default)]
    on_value_change: Option<Callback<String>>,
    #[props(default = true)]
    should_filter: bool,
    #[props(default = command_score as CommandFilter)]
    filter: CommandFilter,
    #[props(default = false)]
    loop_selection: bool,
    #[props(default = false)]
    disable_pointer_selection: bool,
    #[props(default = true)]
    vim_bindings: bool,
    label: Option<String>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute<'_>>,
}

pub fn Command(props: CommandProps) -> Element<'_> {
    // --- State ---
    let initial_value = props
        .value
        .clone()
        .or(props.default_value.clone())
        .unwrap_or_default();
    let state = use_signal(cx, || CommandState {
        value: signal(initial_value.clone()),
        ..Default::default()
    });

    // --- Internal Registries (using Rc<RefCell<>> for shared mutability) ---
    let items = use_ref(cx, || Rc::new(RefCell::new(HashMap::<String, ItemData>::new())));
    let groups = use_ref(cx, || Rc::new(RefCell::new(HashMap::<String, GroupData>::new())));
    let item_ids_order = use_ref(cx, || Rc::new(RefCell::new(Vec::<String>::new())));
    let group_ids_order = use_ref(cx, || Rc::new(RefCell::new(Vec::<String>::new())));

    // --- Refs ---
    let list_inner_ref = use_signal(cx, || Option::<MountedElement>::None);
    let command_root_ref = use_ref(cx, || Option::<MountedElement>::None); // Ref for the root div

    // --- IDs ---
    let list_id = use_unique_id(cx);
    let label_id = use_unique_id(cx);
    let input_id = use_unique_id(cx);

    // --- Props Signals ---
    // Create signals for props that might change or are used in effects/callbacks
    let label_signal = use_signal(cx, || props.label.clone());
    let should_filter_signal = use_signal(cx, || props.should_filter);
    let filter_fn_signal = use_signal(cx, || props.filter);
    let loop_selection_signal = use_signal(cx, || props.loop_selection);
    let disable_pointer_selection_signal = use_signal(cx, || props.disable_pointer_selection);
    let vim_bindings_signal = use_signal(cx, || props.vim_bindings);
    let on_value_change_signal = use_signal(cx, || props.on_value_change.clone());
    let controlled_value_signal = use_signal(cx, || props.value.clone());

    // --- Callbacks for Context ---
    let register_item = use_callback(
        cx,
        move |(id, data): (String, ItemData)| {
            let mut items_borrow = items.write();
            let mut order_borrow = item_ids_order.write();
            if !items_borrow.contains_key(&id) {
                order_borrow.push(id.clone());
            }
            items_borrow.insert(id, data);
            // Trigger refilter/sort when items change
            needs_update(cx);
        },
    );

    let unregister_item = use_callback(
        cx,
        move |id: String| {
            let mut items_borrow = items.write();
            let mut order_borrow = item_ids_order.write();
            items_borrow.remove(&id);
            order_borrow.retain(|item_id| item_id != &id);
            // Trigger refilter/sort when items change
            needs_update(cx);
        },
    );

    let register_group = use_callback(
        cx,
        move |(id, data): (String, GroupData)| {
            let mut groups_borrow = groups.write();
            let mut order_borrow = group_ids_order.write();
             if !groups_borrow.contains_key(&id) {
                order_borrow.push(id.clone());
            }
            groups_borrow.insert(id, data);
            // Trigger refilter/sort when groups change (less critical than items)
             needs_update(cx);
        },
    );

    let unregister_group = use_callback(
        cx,
        move |id: String| {
            let mut groups_borrow = groups.write();
             let mut order_borrow = group_ids_order.write();
            groups_borrow.remove(&id);
            order_borrow.retain(|group_id| group_id != &id);
            // Trigger refilter/sort
             needs_update(cx);
        },
    );

    let set_search = use_callback(cx, move |new_search: String| {
        if *state.read().search.read() != new_search {
            state.write().search.set(new_search);
            // Filtering/sorting happens in the effect below
        }
    });

    let set_value = use_callback(
        cx,
        move |(new_value, should_scroll): (String, bool)| {
            let current_value = state.read().value.read().clone();
            if current_value != new_value {
                // If controlled, emit event
                if let Some(controlled) = controlled_value_signal.read().as_ref() {
                    if let Some(cb) = on_value_change_signal.read().as_ref() {
                        cb.call(new_value.clone());
                    }
                } else {
                    // If uncontrolled, update internal state
                    state.write().value.set(new_value.clone());
                }

                // Update selected item ID based on the new value
                let items_read = items.read();
                let new_selected_id = items_read
                    .iter()
                    .find(|(_, data)| data.value == new_value)
                    .map(|(id, _)| id.clone());
                state.write().selected_item_id.set(new_selected_id);

                // Schedule scroll if needed
                if should_scroll {
                    // Use task::spawn for async operation after render
                    cx.spawn({
                        let list_inner = list_inner_ref.read().clone();
                        async move {
                            // Small delay to ensure DOM is updated
                            gloo_timers::future::TimeoutFuture::new(10).await;
                            if let Some(item_element) = get_selected_item_element(&list_inner) {
                                scroll_item_into_view(&item_element);
                            }
                        }
                    });
                }
            }
        },
    );

    // --- Context Value ---
    let context_value = CommandContext {
        state: *state.read(),
        label: label_signal,
        should_filter: should_filter_signal,
        filter_fn: filter_fn_signal,
        loop_selection: loop_selection_signal,
        disable_pointer_selection: disable_pointer_selection_signal,
        vim_bindings: vim_bindings_signal,
        on_value_change: on_value_change_signal,
        items: items.read().clone(), // Clone the Rc
        groups: groups.read().clone(), // Clone the Rc
        item_ids_order: item_ids_order.read().clone(),
        group_ids_order: group_ids_order.read().clone(),
        list_id,
        label_id,
        input_id,
        list_inner_ref,
        register_item,
        unregister_item,
        register_group,
        unregister_group,
        set_search,
        set_value,
    };

    // --- Effects ---

    // Effect for Filtering and Sorting when search, items, or groups change
    use_effect(cx, (state.read().search, items, groups), |(search, items_ref, groups_ref)| {
        let items_read = items_ref.read().borrow().clone();
        let groups_read = groups_ref.read().borrow().clone();
        let item_order_read = item_ids_order.read().borrow().clone();
        let group_order_read = group_ids_order.read().borrow().clone();
        let filter_fn = *filter_fn_signal.read();
        let should_filter = *should_filter_signal.read();
        let current_value = state.read().value.read().clone();

        async move {
            let (filtered_state, sorted_items, _sorted_groups) = filter_and_sort_items(
                &search.read(),
                &items_read,
                &groups_read,
                &item_order_read,
                &group_order_read,
                filter_fn,
                should_filter,
            );

            // Update filtered state
            state.write().filtered.set(filtered_state);

            // Select first item if search changed and no value is selected or current value is filtered out
            let current_value_is_visible = state.read().filtered.read().items.values().any(|&score| score > 0.0) &&
                items_read.iter().any(|(id, data)| data.value == current_value && state.read().filtered.read().items.contains_key(id));

            if !current_value_is_visible || current_value.is_empty() {
                 // Find the first *visible* item based on the sorted order
                 let first_visible_item_value = sorted_items.iter()
                    .find(|id| state.read().filtered.read().items.contains_key(*id))
                    .and_then(|id| items_read.get(*id))
                    .map(|data| data.value.clone());

                 if let Some(first_val) = first_visible_item_value {
                     // Use the set_value callback to handle controlled/uncontrolled logic
                     set_value.call((first_val, false)); // Don't scroll on auto-select
                 } else {
                     // No visible items, clear value
                     set_value.call((String::new(), false));
                 }
            }
        }
    });

    // Effect to update internal state if controlled `value` prop changes
    use_effect(cx, &props.value, |new_value_prop| {
        let current_internal_value = state.read().value.read().clone();
        let new_value = new_value_prop.unwrap_or_default();
        if current_internal_value != new_value {
            state.write().value.set(new_value.clone());
            // Update selected item ID
            let items_read = items.read();
            let new_selected_id = items_read
                .iter()
                .find(|(_, data)| data.value == new_value)
                .map(|(id, _)| id.clone());
            state.write().selected_item_id.set(new_selected_id);
            // Potentially scroll into view if controlled value changes externally?
            // cx.spawn(...) similar to set_value if needed
        }
        async move {}
    });

    // --- Keyboard Navigation Logic ---
    let handle_keydown = move |evt: KeyboardEvent| {
        let items_elements = get_valid_items(&list_inner_ref.read());
        if items_elements.is_empty() {
            return;
        }

        let selected_element = get_selected_item_element(&list_inner_ref.read());
        let current_index = selected_element
            .as_ref()
            .and_then(|el| {
                items_elements
                    .iter()
                    .position(|item| item.eq(el))
            });

        let loop_enabled = *loop_selection_signal.read();
        let vim_enabled = *vim_bindings_signal.read();

        let get_new_index = |change: i32| -> Option<usize> {
            let len = items_elements.len();
            if len == 0 { return None; }

            match current_index {
                Some(idx) => {
                    let mut new_idx = idx as i32 + change;
                    if loop_enabled {
                        if new_idx < 0 {
                            new_idx = len as i32 - 1;
                        } else if new_idx >= len as i32 {
                            new_idx = 0;
                        }
                    } else {
                        new_idx = new_idx.clamp(0, len as i32 - 1);
                    }
                    Some(new_idx as usize)
                }
                None => {
                    // If nothing selected, select first or last depending on direction
                    if change > 0 { Some(0) } else { Some(len - 1) }
                }
            }
        };

        let select_index = |idx: usize| {
            if let Some(item) = items_elements.get(idx) {
                if let Some(value) = item.get_attribute("data-value") {
                    set_value.call((value, true)); // Scroll on keyboard nav
                }
            }
        };

        match evt.key().as_str() {
            "ArrowDown" => {
                evt.prevent_default();
                if let Some(idx) = get_new_index(1) { select_index(idx); }
            }
            "ArrowUp" => {
                evt.prevent_default();
                if let Some(idx) = get_new_index(-1) { select_index(idx); }
            }
            "n" if vim_enabled && evt.ctrl_key() => {
                 evt.prevent_default();
                 if let Some(idx) = get_new_index(1) { select_index(idx); }
            }
            "p" if vim_enabled && evt.ctrl_key() => {
                 evt.prevent_default();
                 if let Some(idx) = get_new_index(-1) { select_index(idx); }
            }
             "j" if vim_enabled && evt.ctrl_key() => { // Alias for down
                 evt.prevent_default();
                 if let Some(idx) = get_new_index(1) { select_index(idx); }
            }
            "k" if vim_enabled && evt.ctrl_key() => { // Alias for up
                 evt.prevent_default();
                 if let Some(idx) = get_new_index(-1) { select_index(idx); }
            }
            "Home" => {
                evt.prevent_default();
                select_index(0);
            }
            "End" => {
                evt.prevent_default();
                if !items_elements.is_empty() {
                    select_index(items_elements.len() - 1);
                }
            }
            "Enter" => {
                evt.prevent_default();
                if let Some(selected) = get_selected_item_element(&list_inner_ref.read()) {
                    // Simulate click or dispatch custom event
                     if let Ok(html_el) = selected.dyn_into::<HtmlElement>() {
                        html_el.click(); // Easiest way to trigger existing onclick
                    }
                }
            }
            _ => {}
        }
    };

    // --- Render ---
    cx.render(rsx! {
        div {
            ..props.attributes,
            "cmdk-root": "",
            tabindex: "-1", // Make it focusable programmatically if needed, but rely on input focus mostly
            onmounted: move |cx| command_root_ref.set(Some(cx.inner().clone())),
            onkeydown: handle_keydown,

            label {
                "cmdk-label": "",
                "for": "{context_value.input_id.read()}",
                id: "{context_value.label_id.read()}",
                style: "{SR_ONLY_STYLE}",
                "{context_value.label.read().as_deref().unwrap_or(\"Command Menu\")}"
            }

            // Provide context to children
            ContextProvider {
                context: context_value,
                &props.children
            }
        }
    })
}
