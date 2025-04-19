use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};

// Placeholder for command-score logic
pub fn command_score(value: &str, search: &str, keywords: &[String]) -> f32 {
    // Basic placeholder implementation
    if search.is_empty() {
        return 1.0;
    }
    let value_lower = value.to_lowercase();
    let search_lower = search.to_lowercase();
    if value_lower.contains(&search_lower) {
        return 0.8; // Arbitrary score
    }
    for keyword in keywords {
        if keyword.to_lowercase().contains(&search_lower) {
            return 0.6; // Arbitrary score
        }
    }
    0.0
}

pub type CommandFilter = fn(value: &str, search: &str, keywords: &[String]) -> f32;

#[derive(Clone, Debug, Default)]
pub struct FilteredState {
    pub count: usize,
    pub items: HashMap<String, f32>, // item_id -> score
    pub groups: HashSet<String>,     // group_id
}

#[derive(Clone, Debug)]
pub struct CommandState {
    pub search: Signal<String>,
    pub value: Signal<String>, // selected item value
    pub selected_item_id: Signal<Option<String>>,
    pub filtered: Signal<FilteredState>,
}

impl Default for CommandState {
    fn default() -> Self {
        Self {
            search: signal(String::new()),
            value: signal(String::new()),
            selected_item_id: signal(None),
            filtered: signal(FilteredState::default()),
        }
    }
}

// Store equivalent - using Signals directly within context for simplicity in Dioxus
// We'll manage item/group registration and filtering within the main Command component's effects.
