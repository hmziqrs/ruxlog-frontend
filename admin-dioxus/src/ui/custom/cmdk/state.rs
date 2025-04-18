#![allow(non_snake_case)]
use std::{fmt::format, rc::Rc};

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
    pub ids: ContextIds,
}

impl CommandContext {
    pub fn new() -> Self {
        // randomly generate a unique id for the input
        Self {
            search: String::new(),
            is_open: false,
            active_index: 0,
            ids: ContextIds::default(),
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



#[derive(Clone, PartialEq)]
pub struct ContextIds {
    pub root: String,
    pub input: String,
    pub list: String,
    pub empty: String,
    pub loading: String,
    pub heading: String,
}

impl ContextIds {
    pub fn default() ->Self {
        let id = uuid::Uuid::new_v4();
        Self {
            root: format!("root-{}", id),
            input: format!("input-{}", id),
            list: format!("list-{}", id),
            empty: format!("empty-{}", id),
            loading: format!("loading-{}", id),
            heading: format!("heading-{}", id),
        }
    }
}

pub fn default_filter(value: &str, search: &str, keywords: Option<&str>) -> i32 {
    let mapped_keywords = keywords.map(|k| vec![k.to_string()]);
    command_score(value, search, mapped_keywords.as_deref()) as i32
}
