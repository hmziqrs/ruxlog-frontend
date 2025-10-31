use crate::store::auth::UserRole;
use crate::store::traits::{ListQuery, ListStore};
use crate::store::{PaginatedList, StateFrame};
use crate::types::SortParam;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub avatar: Option<String>,
    pub created_at: String,
    pub email: String,
    pub id: i32,
    pub is_verified: bool,
    pub name: String,
    pub role: UserRole,
    pub two_fa_backup_codes: Option<String>,
    pub two_fa_enabled: bool,
    pub two_fa_secret: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UsersAddPayload {
    pub avatar: Option<String>,
    pub email: String,
    pub is_verified: bool,
    pub name: String,
    pub role: UserRole,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UsersEditPayload {
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub is_verified: Option<bool>,
    pub name: Option<String>,
    pub role: Option<UserRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsersListQuery {
    pub page: u64,
    pub search: Option<String>,
    pub sorts: Option<Vec<SortParam>>,
    pub role: Option<UserRole>,
    pub is_verified: Option<bool>,
    pub created_at_gt: Option<DateTime<Utc>>,
    pub created_at_lt: Option<DateTime<Utc>>,
    pub updated_at_gt: Option<DateTime<Utc>>,
    pub updated_at_lt: Option<DateTime<Utc>>,
}

impl Default for UsersListQuery {
    fn default() -> Self {
        Self {
            page: 1,
            search: None,
            sorts: None,
            role: None,
            is_verified: None,
            created_at_gt: None,
            created_at_lt: None,
            updated_at_gt: None,
            updated_at_lt: None,
        }
    }
}

impl UsersListQuery {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ListQuery for UsersListQuery {
    fn new() -> Self {
        Self::new()
    }

    fn page(&self) -> u64 {
        self.page
    }

    fn set_page(&mut self, page: u64) {
        self.page = page;
    }

    fn search(&self) -> Option<String> {
        self.search.clone()
    }

    fn set_search(&mut self, search: Option<String>) {
        self.search = search;
    }

    fn sorts(&self) -> Option<Vec<SortParam>> {
        self.sorts.clone()
    }

    fn set_sorts(&mut self, sorts: Option<Vec<SortParam>>) {
        self.sorts = sorts;
    }
}

pub struct UsersState {
    pub add: GlobalSignal<StateFrame<(), UsersAddPayload>>,
    pub edit: GlobalSignal<HashMap<i32, StateFrame<(), UsersEditPayload>>>,
    pub remove: GlobalSignal<HashMap<i32, StateFrame>>,
    pub list: GlobalSignal<StateFrame<PaginatedList<User>>>,
    pub view: GlobalSignal<HashMap<i32, StateFrame<User>>>,
}

impl UsersState {
    pub fn new() -> Self {
        UsersState {
            add: GlobalSignal::new(|| StateFrame::new()),
            edit: GlobalSignal::new(|| HashMap::new()),
            remove: GlobalSignal::new(|| HashMap::new()),
            list: GlobalSignal::new(|| StateFrame::new()),
            view: GlobalSignal::new(|| HashMap::new()),
        }
    }
}

static USER_STATE: std::sync::OnceLock<UsersState> = std::sync::OnceLock::new();

pub fn use_user() -> &'static UsersState {
    USER_STATE.get_or_init(|| UsersState::new())
}

impl ListStore<User, UsersListQuery> for UsersState {
    fn list_frame(&self) -> &GlobalSignal<StateFrame<PaginatedList<User>>> {
        &self.list
    }

    async fn fetch_list(&self) {
        self.list().await;
    }

    async fn fetch_list_with_query(&self, query: UsersListQuery) {
        self.list_with_query(query).await;
    }
}
