use serde::{Deserialize, Serialize};
use dioxus::prelude::*;
use crate::store::{StateFrame, PaginatedList};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub description: Option<String>,
    pub color: String,
    pub text_color: String,
    pub is_active: bool,
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            slug: String::new(),
            created_at: DateTime::<Utc>::from_timestamp(0, 0).unwrap_or_else(|| Utc::now()),
            updated_at: DateTime::<Utc>::from_timestamp(0, 0).unwrap_or_else(|| Utc::now()),
            description: None,
            color: "#3b82f6".to_string(),
            text_color: "#ffffff".to_string(),
            is_active: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TagsListQuery {
    pub page: Option<u64>,
    pub search: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TagsAddPayload {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TagsEditPayload {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub is_active: Option<bool>,
}

pub struct TagsState {
    pub add: GlobalSignal<StateFrame<()>>,
    pub edit: GlobalSignal<HashMap<i32, StateFrame<()>>>,
    pub remove: GlobalSignal<HashMap<i32, StateFrame<()>>>,
    pub list: GlobalSignal<StateFrame<PaginatedList<Tag>>>,
    pub view: GlobalSignal<HashMap<i32, StateFrame<Option<Tag>>>>,
    // pub data_add: GlobalSignal<Option<()>>,
    // pub data_edit: GlobalSignal<Option<()>>,
    // pub data_remove: GlobalSignal<Option<()>>,
    // pub data_list: GlobalSignal<Vec<Tag>>,
    // pub data_view: GlobalSignal<HashMap<i32, Tag>>,
}

impl TagsState {
    pub fn new() -> Self {
        Self {
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

static TAG_STATE: std::sync::OnceLock<TagsState> = std::sync::OnceLock::new();

pub fn use_tag() -> &'static TagsState {
    TAG_STATE.get_or_init(|| TagsState::new())
}
