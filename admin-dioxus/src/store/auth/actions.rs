use super::{AuthState, AuthUser, LoginPayload, UserRole};
use crate::{services::http_client, store::StateFrame};
use dioxus::{logger::tracing, prelude::*};

impl AuthUser {
    pub fn new(id: i32, name: String, email: String, role: UserRole, is_verified: bool) -> Self {
        AuthUser {
            id,
            name,
            email,
            avatar: None,
            role,
            is_verified,
        }
    }

    pub fn dev() -> Self {
        AuthUser::new(
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
            login_status: GlobalSignal::new(|| StateFrame::new()),
            logout_status: GlobalSignal::new(|| StateFrame::new()),
            init_status: GlobalSignal::new(|| StateFrame::new()),
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
                    match response.json::<AuthUser>().await {
                        Ok(user) => {
                            if !user.is_verified || !user.is_admin() {
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
                    // Unauthorized, no user logged in
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
                    match response.json::<AuthUser>().await {
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
        *self.login_status.write() = StateFrame::new();
        *self.logout_status.write() = StateFrame::new();
        *self.init_status.write() = StateFrame::new();
    }
}
