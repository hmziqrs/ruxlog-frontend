use super::{AuthState, LoginPayload, User, UserRole};
use crate::{
    services::http_client,
    store::{ApiError, StateFrame},
};
use dioxus::{logger::tracing, prelude::*};
use gloo_net::http::Response;
#[cfg(target_arch = "wasm32")]
use wasm_cookies::{CookieOptions, SameSite};

const USER_ID_COOKIE: &str = "id";

impl User {
    pub fn new(id: i32, name: String, email: String, role: UserRole, is_verified: bool) -> Self {
        User {
            id,
            name,
            email,
            avatar: None,
            role,
            is_verified,
        }
    }

    pub fn dev() -> Self {
        User::new(
            1,
            "Dev User".to_string(),
            "dev@example.com".to_string(),
            UserRole::Admin,
            true,
        )
    }
}

impl AuthState {
    pub fn new() -> Self {
        AuthState {
            user: GlobalSignal::new(|| None),
            login_status: GlobalSignal::new(|| StateFrame::<bool>::new()),
            logout_status: GlobalSignal::new(|| StateFrame::<bool>::new()),
            init_status: GlobalSignal::new(|| StateFrame::<bool>::new()),
        }
    }

    fn check_id_cookie_exist() -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            wasm_cookies::get(USER_ID_COOKIE).is_some()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Fallback for non-wasm targets (like tests)
            tracing::warn!("Cookie operations not available in non-wasm environment");
            false
        }
    }

    fn delete_id_cookie() {
        #[cfg(target_arch = "wasm32")]
        {
            wasm_cookies::delete(USER_ID_COOKIE);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            tracing::warn!("Cookie operations not available in non-wasm environment");
        }
    }

    pub async fn logout(&self) {
        self.logout_status.write().set_loading(None);
        let empty_body = {};
        let result = http_client::post("/auth/v1/log_out", &empty_body)
            .send()
            .await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    self.logout_status.write().set_success(None, None);
                    *self.user.write() = None;
                } else {
                    self.logout_status.write().set_api_error(&response).await;
                    *self.user.write() = None;
                }
            }
            Err(e) => {
                self.logout_status.write().set_failed(Some(e.to_string()));
                *self.user.write() = None;
            }
        }
    }

    pub async fn init(&self) {
        // self.init_status.write().set_success(None, None);
        // *self.user.write() = Some(User::dev());
        self.init_status.write().set_loading(None);
        let result = http_client::get("/user/v1/get").send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<User>().await {
                        Ok(user) => {
                            if !user.is_verified || !user.is_admin() {
                                Self::delete_id_cookie();
                                self.init_status.write().set_failed(Some(
                                    "User not allowed to access this page.".to_string(),
                                ));
                                return;
                            }
                            *self.user.write() = Some(user);
                            self.init_status.write().set_success(None, None);
                        }
                        Err(e) => {
                            tracing::error!("Failed to parse user data: {}", e);
                            self.init_status
                                .write()
                                .set_failed(Some(format!("Failed to parse user data: {}", e)));
                        }
                    }
                } else if response.status() == 401 {
                    self.init_status.write().set_success(None, None);
                } else {
                    self.init_status.write().set_api_error(&response).await;
                }
            }
            Err(e) => {
                self.init_status
                    .write()
                    .set_failed(Some(format!("Network error: {}", e)));
            }
        }
    }

    pub async fn login(&self, email: String, password: String) {
        self.login_status.write().set_loading(None);
        let payload = LoginPayload { email, password };
        let result = http_client::post("/auth/v1/log_in", &payload).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<User>().await {
                        Ok(user) => {
                            if !user.is_verified || !user.is_admin() {
                                self.login_status.write().set_failed(Some(
                                    "User not allowed to access this page.".to_string(),
                                ));
                                return;
                            }
                            *self.user.write() = Some(user);
                            self.login_status.write().set_success(None, None);
                        }
                        Err(e) => {
                            eprintln!("Failed to parse user data: {}", e);
                            self.login_status
                                .write()
                                .set_failed(Some(format!("Failed to parse user data: {}", e)));
                        }
                    }
                } else {
                    self.login_status.write().set_api_error(&response).await;
                }
            }
            Err(e) => {
                self.login_status
                    .write()
                    .set_failed(Some(format!("Network error: {}", e)));
            }
        }
    }

    pub fn reset(&self) {
        *self.user.write() = None;
        *self.login_status.write() = StateFrame::<bool>::new();
        *self.logout_status.write() = StateFrame::<bool>::new();
        *self.init_status.write() = StateFrame::<bool>::new();
    }
}
