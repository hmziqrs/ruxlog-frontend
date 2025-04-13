use crate::store::StateFrame;
use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use super::{AuthState, User};
use dioxus::prelude::*;


const CACHE_AUTH_KEY: &str = "app/auth_user";

impl User {
    pub fn new(id: i32, name: String, email: String) -> Self {
        User { id, name, email }
    }

    pub fn dev() -> Self {
        User::new(1, "Dev User".to_string(), "dev@example.com".to_string())
    }
}

impl AuthState {
    pub fn new() -> Self {
        AuthState {
            user: GlobalSignal::new(|| None),
            login_status: GlobalSignal::new(|| StateFrame::<bool>::new()),
            logout_status: GlobalSignal::new(|| StateFrame::<bool>::new()),
            signup_status: GlobalSignal::new(|| StateFrame::<bool>::new()),
            init_status: GlobalSignal::new(|| StateFrame::<bool>::new()),
        }
    }

    pub async fn logout(&self) {
        self.logout_status.write().set_loading(None);

        LocalStorage::delete(CACHE_AUTH_KEY);

        gloo_timers::future::TimeoutFuture::new(1000).await;

        self.logout_status.write().set_success(None, None);
        *self.user.write() = None;
    }

    pub async fn init(&self) {
        self.init_status.write().set_loading(None);

        let cache: Result<String, StorageError> = LocalStorage::get(CACHE_AUTH_KEY);

        gloo_timers::future::TimeoutFuture::new(1000).await;

        match cache {
            Ok(cached_user) => {
                let user: User = serde_json::from_str(&cached_user).unwrap();
                *self.user.write() = Some(user);
                self.init_status.write().set_success(None, None);
            }
            Err(_) => {
                self.init_status.write().set_failed(None);
            }
        }
    }

    pub async fn login(&self, email: String, password: String) {
        self.login_status.write().set_loading(None);

        let response = reqwest::get("http://localhost:3000").await;

        match response {
            Ok(_) => {
                let user = User::dev();

                let user_json = serde_json::to_string(&user).unwrap();
                LocalStorage::set(CACHE_AUTH_KEY, user_json).unwrap();

                *self.user.write() = Some(user);
                self.login_status.write().set_success(None, None);
            }
            Err(_) => {
                self.login_status.write().set_failed(None);
            }
        }
    }
}

