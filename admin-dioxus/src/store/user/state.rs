use serde::{Deserialize, Serialize};
use dioxus::prelude::*;
use crate::store::StateFrame;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum UserRole {
    SuperAdmin,
    Admin,
    Moderator,
    Author,
    User,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub avatar: Option<String>,
    pub created_at: String,
    pub email: String,
    pub id: i32,
    pub is_verified: bool,
    pub name: String,
    pub role: UserRole,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UserAddPayload {
    pub avatar: Option<String>,
    pub email: String,
    pub is_verified: bool,
    pub name: String,
    pub role: UserRole,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UserEditPayload {
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub is_verified: Option<bool>,
    pub name: Option<String>,
    pub role: Option<UserRole>,
}

pub struct UserState {
    pub add: GlobalSignal<StateFrame<()>>,
    pub edit: GlobalSignal<HashMap<i32, StateFrame<()>>>,
    pub remove: GlobalSignal<HashMap<i32, StateFrame<()>>>,
    pub list: GlobalSignal<StateFrame<Vec<User>>>,
    pub view: GlobalSignal<HashMap<i32, StateFrame<Option<User>>>>,
    pub data_add: GlobalSignal<Option<()>>,
    pub data_edit: GlobalSignal<Option<()>>,
    pub data_remove: GlobalSignal<Option<()>>,
    pub data_list: GlobalSignal<Vec<User>>,
    pub data_view: GlobalSignal<HashMap<i32, Option<User>>>,
}

impl UserState {
    pub fn new() -> Self {
        UserState {
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

static USER_STATE: std::sync::OnceLock<UserState> = std::sync::OnceLock::new();

pub fn use_user() -> &'static UserState {
    USER_STATE.get_or_init(|| UserState::new())
}
