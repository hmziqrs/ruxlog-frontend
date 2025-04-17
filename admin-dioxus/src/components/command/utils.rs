use dioxus::prelude::*;
use gloo_timers::callback::Timeout;
use std::{cell::RefCell, collections::{HashMap, HashSet}, rc::Rc}; // Added Rc and RefCell
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use crate::components::command::{
    context::{GroupData, ItemData},
    state::{command_score, CommandFilter, FilteredState},
};

// Placeholder for command-score filtering logic
pub fn filter_and_sort_items(
    search: &str,
    items: &HashMap<String, ItemData>,
    groups: &HashMap<String, GroupData>,
    item_ids_order: &[String], // Original insertion order
    group_ids_order: &[String], // Original insertion order
    filter_fn: CommandFilter,
    should_filter: bool,
) -> (FilteredState, Vec<String>, Vec<String>) {
    let mut filtered_items = HashMap::new();
    let mut visible_groups = HashSet::new();
    let mut item_scores = HashMap::new();
    let mut group_max_scores = HashMap::new();

    if !should_filter || search.is_empty() {
        // If no search or filtering disabled, all items are visible with score 1.0
        for (id, item_data) in items.iter() {
            filtered_items.insert(id.clone(), 1.0);
            item_scores.insert(id.clone(), 1.0);
            if let Some(group_id) = &item_data.group_id {
                visible_groups.insert(group_id.clone());
                group_max_scores.insert(group_id.clone(), 1.0);
            }
        }
        for (id, _) in groups.iter() {
            if visible_groups.contains(id) {
                 // Keep original group order if not filtering/sorting by score
                 group_max_scores.entry(id.clone()).or_insert(1.0); // Ensure group is considered if it has items
            }
        }

        let sorted_item_ids = item_ids_order.to_vec(); // Keep original item order
        let sorted_group_ids = group_ids_order.iter()
            .filter(|id| visible_groups.contains(*id))
            .cloned()
            .collect(); // Keep original group order

        return (
            FilteredState {
                count: items.len(),
                items: filtered_items,
                groups: visible_groups,
            },
            sorted_item_ids,
            sorted_group_ids,
        );
    }

    // Calculate scores for each item
    for (id, item_data) in items.iter() {
        let score = filter_fn(&item_data.value, search, &item_data.keywords);
        if score > 0.0 {
            filtered_items.insert(id.clone(), score);
            item_scores.insert(id.clone(), score);
            if let Some(group_id) = &item_data.group_id {
                visible_groups.insert(group_id.clone());
                let max_score = group_max_scores.entry(group_id.clone()).or_insert(0.0);
                *max_score = max_score.max(score);
            }
        }
    }

    // Sort item IDs by score (descending)
    let mut sorted_item_ids: Vec<String> = filtered_items.keys().cloned().collect();
    sorted_item_ids.sort_by(|a, b| {
        item_scores
            .get(b)
            .unwrap_or(&0.0)
            .partial_cmp(item_scores.get(a).unwrap_or(&0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Sort group IDs by max score (descending)
    let mut sorted_group_ids: Vec<String> = visible_groups.iter().cloned().collect();
    sorted_group_ids.sort_by(|a, b| {
        group_max_scores
            .get(b)
            .unwrap_or(&0.0)
            .partial_cmp(group_max_scores.get(a).unwrap_or(&0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    (
        FilteredState {
            count: filtered_items.len(),
            items: filtered_items,
            groups: visible_groups,
        },
        sorted_item_ids,
        sorted_group_ids,
    )
}

// Helper to get valid, visible items from the DOM within the list_inner_ref
pub fn get_valid_items(list_inner_ref: &Option<MountedElement>) -> Vec<Element> {
    list_inner_ref
        .as_ref()
        .and_then(|mounted| {
            let element = mounted.get().ok()?;
            element
                .query_selector_all("[cmdk-item]:not([aria-disabled='true'])")
                .ok()
        })
        .map(|nodes| {
            let mut items = Vec::new();
            for i in 0..nodes.length() {
                if let Some(node) = nodes.item(i) {
                    if let Ok(element) = node.dyn_into::<Element>() {
                        // Check visibility (might need refinement)
                        if let Ok(html_element) = element.clone().dyn_into::<HtmlElement>() {
                            if html_element.offset_width() > 0 || html_element.offset_height() > 0 || html_element.get_bounding_client_rect().width() > 0.0 {
                                items.push(element);
                            }
                        } else {
                             items.push(element); // Assume visible if not an HTMLElement?
                        }
                    }
                }
            }
            items
        })
        .unwrap_or_default()
}

// Helper to get the currently selected item element
pub fn get_selected_item_element(list_inner_ref: &Option<MountedElement>) -> Option<Element> {
    list_inner_ref.as_ref().and_then(|mounted| {
        mounted
            .get()
            .ok()?
            .query_selector("[cmdk-item][aria-selected='true']")
            .ok()
            .flatten()
    })
}

// Helper to scroll an item into view
pub fn scroll_item_into_view(item_element: &Element) {
    // Basic nearest scrolling
    item_element.scroll_into_view_with_scroll_into_view_options(
        web_sys::ScrollIntoViewOptions::new().block(web_sys::ScrollLogicalPosition::Nearest),
    );

    // TODO: Add logic for scrolling group heading if it's the first item?
    // This requires more complex DOM traversal (finding closest group, then heading).
}

// Debounce implementation using gloo_timers and use_hook
pub fn use_debounce<F>(callback: F, delay_ms: u32) -> impl Fn()
where
    F: FnMut() + 'static,
{
    // Use use_hook to initialize and hold the Rc<RefCell<Option<Timeout>>>
    let timeout_state = use_hook(|| Rc::new(RefCell::new(None::<Timeout>)));
    // Use use_hook to initialize and hold the Rc<RefCell<F>> for the callback
    let callback_state = use_hook(|| Rc::new(RefCell::new(callback)));

    // Update the stored callback whenever the input callback changes
    // This effect runs when the identity of `callback` changes (though function identity comparison is tricky in Rust)
    // A safer approach might involve always updating if the hook re-runs, assuming the parent provides the latest callback.
    use_effect(use_reactive!(|(callback_state,)| {
        // *callback_state.borrow_mut() = callback; // This doesn't work as `callback` isn't captured reactively here.
        // We rely on the fact that use_hook re-runs if the component re-renders,
        // and we capture the *latest* callback provided during that render.
        // Let's update the ref *inside* the hook initialization if possible, or manage updates carefully.
        // A simpler way for this pattern is often just use_ref, but adhering to the request:
        // We'll update the callback ref *if* the hook re-runs due to parent re-render.
        // This relies on the parent passing the potentially new callback instance.
        // Note: This might not perfectly capture *every* change if the parent memoizes the callback improperly.
        *callback_state.borrow_mut() = callback; // Update the stored callback on re-render
    }));


    use_drop({
        let timeout_state = timeout_state.clone();
        move || {
            if let Some(t) = timeout_state.borrow_mut().take() {
                t.cancel();
            }
        }
    });

    move || {
        // Cancel previous timeout if it exists
        if let Some(t) = timeout_state.borrow_mut().take() {
            t.cancel();
        }

        // Clone the state Rc before moving into the closure
        let callback_state_clone = callback_state.clone();
        let timeout_state_clone = timeout_state.clone(); // Clone for setting the new timeout

        // Set new timeout
        let new_timeout = Timeout::new(delay_ms, move || {
            // Execute the callback stored in the RefCell
            (callback_state_clone.borrow_mut())();
        });
        *timeout_state_clone.borrow_mut() = Some(new_timeout);
    }
}

// Screen reader only style
pub const SR_ONLY_STYLE: &str = "position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border-width: 0;";

// Generate unique IDs (basic version)
pub fn use_unique_id() -> Signal<String> {
    use_signal(|| format!("cmdk-{}", rand::random::<u32>()))
}
