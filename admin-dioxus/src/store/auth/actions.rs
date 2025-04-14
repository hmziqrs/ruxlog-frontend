use super::{ApiError, AuthState, LoginPayload, User};
use crate::config::Config;
use crate::store::StateFrame;
use dioxus::{logger::tracing, prelude::*};
// use gloo_storage::{LocalStorage, Storage};
#[cfg(target_arch = "wasm32")]
use wasm_cookies::{CookieOptions, SameSite};

const CACHE_AUTH_KEY: &str = "app/auth_user";
const USER_ID_COOKIE: &str = "ux_id";

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

        // Delete auth data
        Self::delete_id_cookie();

        // Make actual API call for logout
        let api_url = Config::api_base_url();
        let url = format!("{}/auth/v1/log_out", api_url);

        let client = reqwest::Client::new();
        let result = client.post(&url).send().await;

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
        if !Self::check_id_cookie_exist() {
            self.init_status
                .write()
                .set_failed(Some("User authentication cookie not found".to_string()));
            return;
        }

        // Try to fetch user data from API
        let api_url = Config::api_base_url();
        let url = format!("{}/user/v1/get", api_url);

        let client = reqwest::Client::new();
        let result = client.get(&url).send().await;

        match result {
            Ok(response) => {
                if response.status().is_success() {
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
        tracing::info!("Login attempt with email");
        self.login_status.write().set_loading(None);

        // Make actual API call for login
        let url = format!("http://{}/auth/v1/log_in", crate::env::APP_API_URL);

        let payload = LoginPayload { email, password };

        tracing::info!("Payload: {:?}", payload);

        let client = reqwest::Client::new();
        let result = client.post(&url).json(&payload).send().await;

        tracing::info!("Login result: {:?}", result);

        match result {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<User>().await {
                        Ok(user) => {
                            if !user.is_verified || user.role != "admin" {
                                Self::delete_id_cookie();
                                self.login_status.write().set_failed(Some(
                                    "User not allowed to access this page.".to_string(),
                                ));
                                return;
                            }

                            // Set auth cookie with proper options
                            #[cfg(target_arch = "wasm32")]
                            {
                                let options = CookieOptions {
                                    path: Some("/"),
                                    domain: None,
                                    secure: false, 
                                    same_site: SameSite::Lax,
                                    expires: None,
                                };


                                wasm_cookies::set(USER_ID_COOKIE, &user.id.to_string(), &options);
                            }

                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                tracing::warn!(
                                    "Cookie operations not available in non-wasm environment"
                                );
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
