#![allow(non_snake_case)]
use std::{fmt::format, rc::Rc};

use dioxus::{prelude::*, web::WebEventExt};
use im::HashMap;

use super::score::command_score;

enum CommandUIType {
    Normal,
    Dialog,
}

#[derive(Clone)]
pub struct MountedDataWrapper(Rc<MountedData>);

impl PartialEq for MountedDataWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_web_event().id() == other.0.as_web_event().id()
    }
}

    

#[derive(Clone, PartialEq)]
pub struct CommandContext {
    pub search: String,
    pub is_open: bool,
    pub active_index: usize,
    pub ids: ContextIds,
    pub groups: HashMap<String, CommandGroupContext>,
    pub item_indexer: Vec<usize>,
}

impl CommandContext {
    pub fn new() -> Self {
        // randomly generate a unique id for the input
        Self {
            search: String::new(),
            is_open: false,
            active_index: 0,
            ids: ContextIds::default(),
            groups: HashMap::new(),
            item_indexer: Vec::new(),
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

    pub fn add_group(&mut self, group: CommandGroupContext) {
        self.groups.insert(group.id.clone(), group);
    }
}


#[derive(Clone, PartialEq)]
pub struct CommandGroupContext {
    pub id: String,
    pub items: Vec<CommandItemContext>,
    pub node: MountedDataWrapper,
}

impl CommandGroupContext {
    pub fn default(node: Rc<MountedData>) -> Self {
        
        Self {
            id: CommandGroupContext::generate_id(),
            items: Vec::new(),
            node: MountedDataWrapper(node),
        }
    }

    pub fn generate_id() -> String {
        let id = uuid::Uuid::new_v4();
        format!("cmdk-group-{}", id)
    }

    pub fn new(id: String, node: Rc<MountedData>) -> Self {
        Self {
            id,
            items: Vec::new(),
            node: MountedDataWrapper(node),
        }
    }

    pub fn add_item(&mut self, item: CommandItemContext) {
        self.items.push(item);
    }
}

#[derive(Clone, PartialEq)]
pub struct CommandItemContext {
    pub id: String,
    pub index: usize,
    pub is_filtered: bool, // true if the item is filtered out
    pub node: MountedDataWrapper,
}

impl CommandItemContext {
    pub fn new(id: String, index: usize, node: MountedData) -> Self {
        // let 
        Self {
            id,
            index,
            is_filtered: false,
            node: MountedDataWrapper(Rc::new(node)),
        }
    }

    pub fn generate_id() -> String {
        let id = uuid::Uuid::new_v4();
        format!("cmdk-item-{}", id)
    }

    pub fn set_filtered(&mut self, is_filtered: bool) {
        self.is_filtered = is_filtered;
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
    pub label: String,
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
            label: format!("heading-{}", id),
        }
    }
}

pub fn default_filter(value: &str, search: &str, keywords: Option<&str>) -> i32 {
    let mapped_keywords = keywords.map(|k| vec![k.to_string()]);
    command_score(value, search, mapped_keywords.as_deref()) as i32
}
