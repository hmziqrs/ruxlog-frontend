use serde::{Deserialize, Serialize};
use dioxus::prelude::*;
use crate::store::StateFrame;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub created_at: String,
    pub updated_at: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TagAddPayload {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TagEditPayload {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
}

pub struct TagState {
    pub add: GlobalSignal<StateFrame<()>>,
    pub edit: GlobalSignal<HashMap<i32, StateFrame<()>>>,
    pub remove: GlobalSignal<HashMap<i32, StateFrame<()>>>,
    pub list: GlobalSignal<StateFrame<Vec<Tag>>>,
    pub view: GlobalSignal<HashMap<i32, StateFrame<Option<Tag>>>>,
    // pub data_add: GlobalSignal<Option<()>>,
    // pub data_edit: GlobalSignal<Option<()>>,
    // pub data_remove: GlobalSignal<Option<()>>,
    // pub data_list: GlobalSignal<Vec<Tag>>,
    // pub data_view: GlobalSignal<HashMap<i32, Tag>>,
}

impl TagState {
    pub fn new() -> Self {
        TagState {
            add: GlobalSignal::new(|| StateFrame::new()),
            edit: GlobalSignal::new(|| HashMap::new()),
            remove: GlobalSignal::new(|| HashMap::new()),
            list: GlobalSignal::new(|| StateFrame::new()),
            view: GlobalSignal::new(|| HashMap::new()),
            // data_add: GlobalSignal::new(|| None),
            // data_edit: GlobalSignal::new(|| None),
            // data_remove: GlobalSignal::new(|| None),
            // data_list: GlobalSignal::new(|| vec![]),
            // data_view: GlobalSignal::new(|| HashMap::new()),
        }
    }
}

static TAG_STATE: std::sync::OnceLock<TagState> = std::sync::OnceLock::new();

pub fn use_tag() -> &'static TagState {
    TAG_STATE.get_or_init(|| TagState::new())
}
