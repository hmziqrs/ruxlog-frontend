use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use dioxus::prelude::*;

use crate::store::StateFrame;

pub struct AuthState {
    pub user: GlobalSignal<Option<User>>,

    pub login_status: GlobalSignal<StateFrame<bool>>,
    pub logout_status: GlobalSignal<StateFrame<bool>>,

    pub init_status: GlobalSignal<StateFrame<bool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: String,
    pub is_verified: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

static AUTH_STATE: OnceLock<AuthState> = OnceLock::new();

pub fn use_auth() -> &'static AuthState {
    AUTH_STATE.get_or_init(|| AuthState::new())
}