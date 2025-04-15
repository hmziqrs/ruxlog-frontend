use super::{ApiError, AuthState, LoginPayload, User};
use crate::{services::http_client, store::StateFrame};
use dioxus::{logger::tracing, prelude::*};
#[cfg(target_arch = "wasm32")]
use wasm_cookies::{CookieOptions, SameSite};

const USER_ID_COOKIE: &str = "id";

impl User {
    pub fn new(id: i32, name: String, email: String, role: String, is_verified: bool) -> Self {
        User {
            id,
            name,
            email,
            role,
            is_verified,
        }
    }

    pub fn dev() -> Self {
        User::new(
            1,
            "Dev User".to_string(),
            "dev@example.com".to_string(),
            "admin".to_string(),
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

        // Delete auth cookie
        Self::delete_id_cookie();

        // Make API call for logout using our singleton reqwest service
        let result = http_client::delete("/auth/v1/log_out").send().await;

        match result {
            Ok(_) => {
                self.logout_status.write().set_success(None, None);
                *self.user.write() = None;
            }
            Err(e) => {
                // Even if API fails, we still clear local auth
                self.logout_status.write().set_failed(Some(e.to_string()));
                *self.user.write() = None;
            }
        }
    }

    pub async fn init(&self) {
        self.init_status.write().set_loading(None);

        // Check if ID cookie exists
        if (!Self::check_id_cookie_exist()) {
            self.init_status
                .write()
                .set_success(None, None);
            return;
        }

        // Try to fetch user data from API using our singleton http_client service
        let result = http_client::get("/user/v1/get").send().await;

        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<User>().await {
                        Ok(user) => {
                            if !user.is_verified || user.role != "admin" {
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
                            self.init_status
                                .write()
                                .set_failed(Some(format!("Failed to parse user data: {}", e)));
                        }
                    }
                } else {
                    match response.json::<ApiError>().await {
                        Ok(api_error) => {
                            self.init_status.write().set_failed(Some(api_error.message));
                        }
                        Err(_) => {
                            self.init_status
                                .write()
                                .set_failed(Some(format!("API error ")));
                        }
                    }
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

        // Use our singleton http_client service for the login request
        let result = http_client::post("/auth/v1/log_in", &payload)
            .send()
            .await;

        tracing::info!("Login result: {:?}", result);

        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<User>().await {
                        Ok(user) => {
                            if !user.is_verified || user.role != "admin" {
                                // Self::delete_id_cookie();
                                self.login_status.write().set_failed(Some(
                                    "User not allowed to access this page.".to_string(),
                                ));
                                return;
                            }

                            *self.user.write() = Some(user);
                            self.login_status.write().set_success(None, None);
                        }
                        Err(e) => {
                            self.login_status
                                .write()
                                .set_failed(Some(format!("Failed to parse user data: {}", e)));
                        }
                    }
                } else {
                    match response.json::<ApiError>().await {
                        Ok(api_error) => {
                            self.login_status
                                .write()
                                .set_failed(Some(api_error.message));
                        }
                        Err(_) => {
                            self.login_status
                                .write()
                                .set_failed(Some(format!("API error")));
                        }
                    }
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
