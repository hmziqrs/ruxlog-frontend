use serde::{Deserialize, Serialize};
use dioxus::prelude::*;
use crate::store::{StateFrame, PaginatedList};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub created_at: String,
    pub updated_at: String,
    pub color: String,
    pub text_color: String,
    pub is_active: bool,
    pub cover_image: Option<String>,
    pub description: Option<String>,
    pub logo_image: Option<String>,
    pub parent_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CategoryAddPayload {
    pub name: String,
    pub slug: String,
    pub color: String,
    pub text_color: Option<String>,
    pub is_active: Option<bool>,
    pub cover_image: Option<String>,
    pub description: Option<String>,
    pub logo_image: Option<String>,
    pub parent_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CategoryEditPayload {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub parent_id: Option<Option<i32>>,
    pub description: Option<Option<String>>,
    pub cover_image: Option<Option<String>>,
    pub logo_image: Option<Option<String>>,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CategoryListQuery {
    pub page: Option<u64>,
    pub search: Option<String>,
    pub sort_order: Option<String>,
    pub parent_id: Option<i32>,
}

pub struct CategoryState {
    pub add: GlobalSignal<StateFrame<()>>,
    pub edit: GlobalSignal<HashMap<i32, StateFrame<()>>>,
    pub remove: GlobalSignal<HashMap<i32, StateFrame<()>>>,
    pub list: GlobalSignal<StateFrame<PaginatedList<Category>>>,
    pub view: GlobalSignal<HashMap<i32, StateFrame<Option<Category>>>>,
}

impl CategoryState {
    pub fn new() -> Self {
        CategoryState {
            add: GlobalSignal::new(|| StateFrame::new()),
            edit: GlobalSignal::new(|| HashMap::new()),
            remove: GlobalSignal::new(|| HashMap::new()),
            list: GlobalSignal::new(|| StateFrame::new()),
            view: GlobalSignal::new(|| HashMap::new()),
        }
    }
}

static CATEGORY_STATE: std::sync::OnceLock<CategoryState> = std::sync::OnceLock::new();

pub fn use_category() -> &'static CategoryState {
    CATEGORY_STATE.get_or_init(|| CategoryState::new())
}
