use serde::{Deserialize, Serialize};
use dioxus::prelude::*;
use crate::store::StateFrame;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub created_at: String,
    pub updated_at: String,
    pub cover_image: Option<String>,
    pub description: Option<String>,
    pub logo_image: Option<String>,
    pub parent_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CategoryAddPayload {
    pub name: String,
    pub slug: String,
    pub cover_image: Option<String>,
    pub description: Option<String>,
    pub logo_image: Option<String>,
    pub parent_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CategoryEditPayload {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub cover_image: Option<String>,
    pub description: Option<String>,
    pub logo_image: Option<String>,
    pub parent_id: Option<i32>,
}

pub struct CategoryState {
    pub add: GlobalSignal<StateFrame<()>>,
    pub edit: GlobalSignal<HashMap<i32, StateFrame<()>>>,
    pub remove: GlobalSignal<HashMap<i32, StateFrame<()>>>,
    pub list: GlobalSignal<StateFrame<Vec<Category>>>,
    pub view: GlobalSignal<HashMap<i32, StateFrame<Option<Category>>>>,
    pub data_add: GlobalSignal<Option<()>>,
    pub data_edit: GlobalSignal<Option<()>>,
    pub data_remove: GlobalSignal<Option<()>>,
    pub data_list: GlobalSignal<Vec<Category>>,
    pub data_view: GlobalSignal<HashMap<i32, Category>>,
}

impl CategoryState {
    pub fn new() -> Self {
        CategoryState {
            add: GlobalSignal::new(|| StateFrame::new()),
            edit: GlobalSignal::new(|| HashMap::new()),
            remove: GlobalSignal::new(|| HashMap::new()),
            list: GlobalSignal::new(|| StateFrame::new()),
            view: GlobalSignal::new(|| HashMap::new()),
            data_add: GlobalSignal::new(|| None),
            data_edit: GlobalSignal::new(|| None),
            data_remove: GlobalSignal::new(|| None),
            data_list: GlobalSignal::new(|| vec![]),
            data_view: GlobalSignal::new(|| HashMap::new()),
        }
    }
}

static CATEGORY_STATE: std::sync::OnceLock<CategoryState> = std::sync::OnceLock::new();

pub fn use_category() -> &'static CategoryState {
    CATEGORY_STATE.get_or_init(|| CategoryState::new())
}
