#![allow(non_snake_case)]
use std::rc::Rc;

use dioxus::prelude::*;

use super::score::command_score;

enum CommandUIType {
    Normal,
    Dialog,
}

#[derive(Clone, PartialEq)]
pub struct CommandContext {
    pub search: String,
    pub is_open: bool,
    pub active_index: usize,
    pub input_id: String,
}

impl CommandContext {
    pub fn new() -> Self {
        // randomly generate a unique id for the input
        let input_id = format!("cmdk-input-{}", uuid::Uuid::new_v4());
        Self {
            search: String::new(),
            is_open: false,
            active_index: 0,
            input_id,
        }
    }

    pub fn set_search(&mut self, search: String) {
        self.search = search;
    }

    pub fn set_open(&mut self, is_open: bool) {
        self.is_open = is_open;
    }

    pub fn toggle_open(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn set_active_index(&mut self, idx: usize) {
        self.active_index = idx;
    }
}


pub fn default_filter(value: &str, search: &str, keywords: Option<&str>) -> i32 {
    let mapped_keywords = keywords.map(|k| vec![k.to_string()]);
    command_score(value, search, mapped_keywords.as_deref()) as i32
}
