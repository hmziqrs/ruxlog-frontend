#![allow(non_snake_case)]
use crate::components::command::{
    context::{use_command_context, use_group_context, ItemData},
    utils::use_unique_id,
};
use dioxus::prelude::*;
use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[derive(Props, Clone, PartialEq)]
pub struct CommandItemProps {
    children: Element<'_>,
    #[props(default)]
    disabled: bool,
    on_select: Option<Callback<String>>, // Called with the item's value
    value: Option<String>, // Explicit value, otherwise inferred from children
    #[props(default)]
    keywords: Vec<String>,
    #[props(default)]
    force_mount: bool, // If true, always rendered regardless of filtering
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute<'_>>,
}

#[component]
pub fn CommandItem(props: CommandItemProps) -> Element {
    let cmdk_context = use_command_context();
    let group_context = use_group_context(); // Might be None if not in a group

    let item_id = use_unique_id();
    let item_ref = use_ref(|| Option::<MountedElement>::None);
    let inferred_value = use_state(String::new); // State to hold inferred value if needed

    // Determine the actual value (prop > inferred > "")
    let value = use_memo((props.value.clone(), inferred_value.get().clone()), |(prop_val, inferred_val)| {
        prop_val.unwrap_or_else(|| inferred_val.clone())
    });

    let is_selected = use_memo((cmdk_context.state.value, value.clone()), |(selected_val, current_val)| {
        *selected_val.read() == current_val
    });

    let group_id = group_context.id.read().clone();
    let is_forced = props.force_mount || group_context.force_mount.cloned().unwrap_or(false);

    // Determine if the item should be rendered based on filtering/force_mount
    let is_visible = use_memo(
        (
            is_forced,
            cmdk_context.should_filter,
            cmdk_context.state.search,
            cmdk_context.state.filtered,
            item_id,
        ),
        |(forced, should_filter, search, filtered, id)| {
            if forced || !*should_filter.read() || search.read().is_empty() {
                true
            } else {
                filtered.read().items.contains_key(&*id.read())
            }
        },
    );

    // Register/Unregister Item
    use_effect(&item_id, |id_signal| {
        let id = id_signal.read().clone();
        let data = ItemData {
            value: value.clone(),
            keywords: props.keywords.clone(),
            group_id: group_id.clone(),
            node_ref: None, // Dioxus handles node refs differently
        };
        cmdk_context.register_item.call((id.clone(), data));

        // Cleanup function
        let unregister_item = cmdk_context.unregister_item.clone();
        async move {
            unregister_item.call(id);
        }
    });

    // Update registration if value/keywords change
     use_effect((item_id, value.clone(), props.keywords.clone(), group_id.clone()), |(id_signal, val, kw, grp_id)| {
         let id = id_signal.read().clone();
         let data = ItemData {
             value: val.clone(),
             keywords: kw.clone(),
             group_id: grp_id.clone(),
             node_ref: None,
         };
         cmdk_context.register_item.call((id.clone(), data));
         async move {} // No cleanup needed here, main unregister handles it
     });


    // Effect to infer value from rendered children if `props.value` is not provided
    use_effect(&item_ref, |item_ref_state| {
        if props.value.is_none() {
            if let Some(mounted) = item_ref_state.read().as_ref() {
                 if let Ok(element) = mounted.get() {
                     if let Ok(html_element) = element.dyn_into::<HtmlElement>() {
                         if let Some(text) = html_element.text_content() {
                             let trimmed_text = text.trim().to_string();
                             if *inferred_value.get() != trimmed_text {
                                inferred_value.set(trimmed_text);
                                // Value change will trigger re-registration via the other effect
                             }
                         }
                     }
                 }
            }
        }
         async move {}
    });


    let handle_click = move |_| {
        if !props.disabled {
            let current_value = value.clone();
            // Use context's setter to handle controlled/uncontrolled state
            cmdk_context.set_value.call((current_value.clone(), false)); // Don't scroll on click
            if let Some(cb) = &props.on_select {
                cb.call(current_value);
            }
        }
    };

    let handle_pointer_move = move |_| {
        if !props.disabled && !*cmdk_context.disable_pointer_selection.read() {
            // Only set value, don't trigger on_select
             cmdk_context.set_value.call((value.clone(), false)); // Don't scroll on pointer move
        }
    };

    if !*is_visible {
        return None; // Don't render if filtered out and not force_mount
    }

    rsx! {
        div {
            ..props.attributes,
            key: "{item_id.read()}", // Use key for efficient updates
            id: "{item_id.read()}",
            "cmdk-item": "",
            role: "option",
            "aria-disabled": if props.disabled { "true" } else { "false" },
            "aria-selected": "{is_selected}",
            "data-value": "{value}", // Store value for keyboard nav lookup
            "data-disabled": if props.disabled { "true" } else { "false" }, // Optional data attribute
            "data-selected": if *is_selected { "true" } else { "false" }, // Optional data attribute
            onmounted: move |mount_event| item_ref.set(Some(mount_event.inner().clone())), // Update onmounted
            onclick: handle_click,
            onpointermove: handle_pointer_move,

            &props.children
        }
    }
}

