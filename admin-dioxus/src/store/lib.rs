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
pub struct StateFrame<D: Clone = (), M: Clone = ()> {
    pub status: StateFrameStatus,
    pub message: Option<String>,
    pub data: Option<D>,
    pub meta: Option<M>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PaginatedList<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

impl<T> PaginatedList<T> {
    pub fn has_next_page(&self) -> bool {
        self.page * self.per_page < self.total
    }

    pub fn has_previous_page(&self) -> bool {
        self.page > 1
    }
}

impl<T> std::ops::Deref for PaginatedList<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> std::ops::DerefMut for PaginatedList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> IntoIterator for PaginatedList<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T: Clone, Q: Clone> Default for StateFrame<T, Q> {
    fn default() -> Self {
        Self {
            status: StateFrameStatus::Init,
            data: None,
            message: None,
            meta: None,
        }
    }
}

impl<T: Clone, Q: Clone> StateFrame<T, Q> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_loading(message: Option<String>) -> Self {
        Self {
            status: StateFrameStatus::Loading,
            data: None,
            message,
            meta: None,
        }
    }

    pub fn new_with_data(data: Option<T>) -> Self {
        Self {
            status: StateFrameStatus::Success,
            data,
            message: None,
            meta: None,
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

    pub fn set_meta(&mut self, meta: Option<Q>) {
        self.meta = meta;
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
