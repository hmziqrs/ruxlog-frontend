use crate::services::http_client::{HttpError, HttpRequest, HttpResponse};
use dioxus::prelude::GlobalSignal;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::hash::Hash;

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

    pub fn set_loading_meta(&mut self, meta: Option<Q>, message: Option<String>) {
        self.status = StateFrameStatus::Loading;
        self.meta = meta;
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

    pub async fn set_api_error(&mut self, response: &HttpResponse) {
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

/// Send a request, parse JSON into `T`, and update the provided `StateFrame<T>`.
/// Returns `Some(T)` on success to allow callers to perform cache-sync logic if needed.
pub async fn list_state_abstraction<T>(
    state: &GlobalSignal<StateFrame<T>>,
    req: HttpRequest,
    parse_label: &str,
) -> Option<T>
where
    T: DeserializeOwned + Clone + 'static,
{
    state.write().set_loading(None);
    match req.send().await {
        Ok(response) => {
            if (200..300).contains(&response.status()) {
                match response.json::<T>().await {
                    Ok(data) => {
                        state.write().set_success(Some(data.clone()), None);
                        Some(data)
                    }
                    Err(e) => {
                        state
                            .write()
                            .set_failed(Some(format!("Failed to parse {}: {}", parse_label, e)));
                        None
                    }
                }
            } else {
                state.write().set_api_error(&response).await;
                None
            }
        }
        Err(e) => {
            state
                .write()
                .set_failed(Some(format!("Network error: {}", e)));
            None
        }
    }
}

/// Shared helper to fetch a single record and hydrate a keyed `StateFrame` map.
/// Returns `Some(Parsed)` on success so callers can optionally sync additional caches.
pub async fn view_state_abstraction<K, StoreData, Parsed, F, MapFn>(
    state: &GlobalSignal<HashMap<K, StateFrame<StoreData>>>,
    id: K,
    send_future: F,
    parse_label: &str,
    map_to_store: MapFn,
) -> Option<Parsed>
where
    K: Eq + Hash + Copy + 'static,
    StoreData: Clone + 'static,
    Parsed: DeserializeOwned + Clone + 'static,
    F: Future<Output = Result<HttpResponse, HttpError>>,
    MapFn: Fn(&Parsed) -> StoreData,
{
    {
        let mut map = state.write();
        map.entry(id)
            .or_insert_with(StateFrame::new)
            .set_loading(None);
    }

    match send_future.await {
        Ok(response) => {
            if (200..300).contains(&response.status()) {
                match response.json::<Parsed>().await {
                    Ok(parsed) => {
                        let store_value = map_to_store(&parsed);
                        let mut map = state.write();
                        map.entry(id)
                            .or_insert_with(StateFrame::new)
                            .set_success(Some(store_value), None);
                        Some(parsed)
                    }
                    Err(e) => {
                        let mut map = state.write();
                        map.entry(id)
                            .or_insert_with(StateFrame::new)
                            .set_failed(Some(format!("Failed to parse {}: {}", parse_label, e)));
                        None
                    }
                }
            } else {
                let mut map = state.write();
                map.entry(id)
                    .or_insert_with(StateFrame::new)
                    .set_api_error(&response)
                    .await;
                None
            }
        }
        Err(e) => {
            let mut map = state.write();
            map.entry(id)
                .or_insert_with(StateFrame::new)
                .set_failed(Some(format!("Network error: {}", e)));
            None
        }
    }
}
