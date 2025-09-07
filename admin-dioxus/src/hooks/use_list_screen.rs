use dioxus::prelude::*;
use crate::store::SortParam;

#[derive(Debug, Clone)]
pub struct ListScreenConfig {
    pub default_sort_field: String,
    pub default_sort_order: String,
}

impl Default for ListScreenConfig {
    fn default() -> Self {
        Self {
            default_sort_field: "name".to_string(),
            default_sort_order: "asc".to_string(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ListScreenState {
    pub search_input: Signal<String>,
    pub sort_field: Signal<String>,
    pub sort_order: Signal<String>,
    pub reload_tick: Signal<u32>,
}

impl ListScreenState {
    pub fn new(config: &ListScreenConfig) -> Self {
        Self {
            search_input: use_signal(|| String::new()),
            sort_field: use_signal(|| config.default_sort_field.clone()),
            sort_order: use_signal(|| config.default_sort_order.clone()),
            reload_tick: use_signal(|| 0u32),
        }
    }

    // Direct signal access methods
    pub fn search_input(&self) -> String {
        self.search_input.read().clone()
    }

    pub fn sort_field(&self) -> String {
        self.sort_field.read().clone()
    }

    pub fn sort_order(&self) -> String {
        self.sort_order.read().clone()
    }

    pub fn reload_tick(&self) -> u32 {
        self.reload_tick.read().clone()
    }

    // Utility methods for common operations
    pub fn get_sort_params(&self) -> Vec<SortParam> {
        vec![SortParam {
            field: self.sort_field.read().clone(),
            order: self.sort_order.read().clone(),
        }]
    }

    pub fn handle_sort(&self, field: String) {
        let mut sort_field = self.sort_field;
        let mut sort_order = self.sort_order;
        
        let current_field = sort_field();
        let current_order = sort_order();
        
        if current_field == field {
            // Toggle order for same field
            let new_order = if current_order == "asc" { "desc" } else { "asc" };
            sort_order.set(new_order.to_string());
        } else {
            // New field, default to asc
            sort_field.set(field);
            sort_order.set("asc".to_string());
        }
    }

    pub fn set_search(&self, value: String) {
        let mut search_input = self.search_input;
        search_input.set(value);
    }

    pub fn clear_search(&self) {
        let mut search_input = self.search_input;
        search_input.set(String::new());
    }

    pub fn trigger_reload(&self) {
        let mut reload_tick = self.reload_tick;
        let current = reload_tick();
        reload_tick.set(current + 1);
    }
}

/// Generic hook for list screen logic
/// 
/// Usage:
/// ```rust
/// let list_state = use_list_screen(Some(ListScreenConfig::default()));
/// ```
pub fn use_list_screen(config: Option<ListScreenConfig>) -> ListScreenState {
    let config = config.unwrap_or_default();
    ListScreenState::new(&config)
}
