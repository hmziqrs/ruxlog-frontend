use serde::{Deserialize, Serialize};
use dioxus::prelude::*;
use crate::store::{StateFrame, PaginatedList};
use crate::types::SortParam;
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

impl Tag {
    pub fn row_keys(&self) -> Vec<String> {
        vec!["id", "name", "slug", "created_at", "updated_at", "description", "color", "text_color", "is_active"].into_iter().map(|s| s.to_string()).collect()
    }

    pub fn row_values(&self) -> Vec<String> {
        vec![self.id.to_string(), self.name.clone(), self.slug.clone(), self.created_at.to_string(), self.updated_at.to_string(), self.description.clone().unwrap_or("".to_string()), self.color.clone(), self.text_color.clone(), self.is_active.to_string()].into_iter().map(|s| s.to_string()).collect()
    }
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
    pub page: u64,
    pub search: Option<String>,
    pub sorts: Option<Vec<SortParam>>, // [{ field, order }]
    pub is_active: Option<bool>,
    pub created_at_gt: Option<DateTime<Utc>>,
    pub created_at_lt: Option<DateTime<Utc>>,
    pub updated_at_gt: Option<DateTime<Utc>>,
    pub updated_at_lt: Option<DateTime<Utc>>,
}

impl TagsListQuery {
    pub fn new() -> Self {
        Self {
            page: 1,
            ..Default::default()
        }
    }
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
    pub add: GlobalSignal<StateFrame<(), TagsAddPayload>>,
    pub edit: GlobalSignal<HashMap<i32, StateFrame<(), TagsEditPayload>>>,
    pub remove: GlobalSignal<HashMap<i32, StateFrame>>,
    pub list: GlobalSignal<StateFrame<PaginatedList<Tag>>>,
    pub view: GlobalSignal<HashMap<i32, StateFrame<Tag>>>,
}

impl TagsState {
    pub fn new() -> Self {
        Self {
            add: GlobalSignal::new(|| StateFrame::new()),
            edit: GlobalSignal::new(|| HashMap::new()),
            remove: GlobalSignal::new(|| HashMap::new()),
            list: GlobalSignal::new(|| StateFrame::new()),
            view: GlobalSignal::new(|| HashMap::new()),
        }
    }
}

static TAG_STATE: std::sync::OnceLock<TagsState> = std::sync::OnceLock::new();

pub fn use_tag() -> &'static TagsState {
    TAG_STATE.get_or_init(|| TagsState::new())
}
