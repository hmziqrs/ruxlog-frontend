use dioxus_radio::{
    self,
    hooks::{ChannelSelection, DataAsyncReducer, Radio, RadioAsyncReducer, RadioChannel},
};

use crate::store::StateFrame;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthState {
    // pub user: GlobalSignal<Option<User>>,
    pub login_status: StateFrame<bool>,
    // pub logout_status: GlobalSignal<StateFrame<bool>>,

    // pub init_status: GlobalSignal<StateFrame<bool>>,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum AuthStateChannel {
    Main,
}

pub enum AuthStateAction {
    // with username and password
    Login(String, String),
}

impl RadioChannel<AuthState> for AuthStateChannel {}
