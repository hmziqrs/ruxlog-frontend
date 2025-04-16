use gloo_net::http::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StateFrameStatus {
    Init,
    Loading,
    Success,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateFrame<T: Clone> {
    pub status: StateFrameStatus,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T: Clone> Default for StateFrame<T> {
    fn default() -> Self {
        Self {
            status: StateFrameStatus::Init,
            data: None,
            message: None,
        }
    }
}

impl<T: Clone> StateFrame<T> {
    pub fn new() -> Self {
        Self {
            status: StateFrameStatus::Init,
            data: None,
            message: None,
        }
    }

    pub fn new_with_loading(message: Option<String>) -> Self {
        Self {
            status: StateFrameStatus::Loading,
            data: None,
            message,
        }
    }

    pub fn new_with_data(data: Option<T>) -> Self {
        Self {
            status: StateFrameStatus::Success,
            data,
            message: None,
        }
    }

    pub fn is_init(&self) -> bool {
        self.status == StateFrameStatus::Init
    }

    pub fn is_loading(&self) -> bool {
        self.status == StateFrameStatus::Loading
    }

    pub fn is_success(&self) -> bool {
        self.status == StateFrameStatus::Success
    }

    pub fn is_failed(&self) -> bool {
        self.status == StateFrameStatus::Failed
    }

    pub fn set_loading(&mut self, message: Option<String>) {
        self.status = StateFrameStatus::Loading;
        self.message = message;
    }

    pub fn set_success(&mut self, data: Option<T>, message: Option<String>) {
        self.status = StateFrameStatus::Success;
        self.data = data;
        self.message = message;
    }

    pub fn set_failed(&mut self, message: Option<String>) {
        self.status = StateFrameStatus::Failed;
        self.message = message;
    }

    pub async fn set_api_error(&mut self, response: &Response) {
        match response.json::<ApiError>().await {
            Ok(api_error) => {
                self.set_failed(Some(api_error.message));
            }
            Err(_) => {
                self.set_failed(Some("API error".to_string()));
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub status: Option<u16>,
}