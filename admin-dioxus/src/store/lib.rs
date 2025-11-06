use crate::services::http_client::{HttpError, HttpRequest, HttpResponse};
use dioxus::logger::tracing;
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
                let msg = api_error.message.clone().or_else(|| {
                    // Fallback to a generic message when server omits message in production
                    Some(format!(
                        "Request failed{} (status {})",
                        api_error
                            .code
                            .as_ref()
                            .map(|c| format!(" with code {}", c))
                            .unwrap_or_default(),
                        api_error.status
                    ))
                });
                self.set_failed(msg);
            }
            Err(_) => {
                self.set_failed(Some("API error".to_string()));
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    /// Error type code string coming from server under the JSON key "type" (e.g., "AUTH_001")
    #[serde(rename = "type")]
    pub code: Option<String>,
    /// Human-readable message (may be omitted in production builds of the server)
    pub message: Option<String>,
    /// HTTP status code echoed by the backend
    pub status: u16,
    /// Optional detailed description (present only in development on the server)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// Optional additional structured context for the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    /// Optional Retry-After seconds if the request is rate-limited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
    /// Optional request id for tracing/correlation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
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
                        // Try to get the raw response text for better debugging
                        let response_text = response.text().await.unwrap_or_default();
                        tracing::error!("Failed to parse {}: {:?}\nResponse: {}", parse_label, e, response_text);
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

/// Generic helper for request/response cycles that update a single `StateFrame`.
/// Returns `Some(Parsed)` on success so callers can chain follow-up actions.
pub async fn state_request_abstraction<Data, Meta, Parsed, F, OnSuccess>(
    state: &GlobalSignal<StateFrame<Data, Meta>>,
    meta: Option<Meta>,
    send_future: F,
    parse_label: &str,
    on_success: OnSuccess,
) -> Option<Parsed>
where
    Data: Clone + 'static,
    Meta: Clone + 'static,
    Parsed: DeserializeOwned + Clone + 'static,
    F: Future<Output = Result<HttpResponse, HttpError>>,
    OnSuccess: Fn(&Parsed) -> (Option<Data>, Option<String>),
{
    {
        let mut frame = state.write();
        frame.set_loading_meta(meta, None);
    }

    match send_future.await {
        Ok(response) => {
            if (200..300).contains(&response.status()) {
                match response.json::<Parsed>().await {
                    Ok(parsed) => {
                        let (data, message) = on_success(&parsed);
                        state.write().set_success(data, message);
                        Some(parsed)
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

/// Specialized version for updating items in a PaginatedList cache
pub async fn edit_state_abstraction<K, T, Payload, F, GetId, OnSuccess>(
    state: &GlobalSignal<HashMap<K, StateFrame<(), Payload>>>,
    id: K,
    payload: Payload,
    send_future: F,
    parse_label: &str,
    sync_list_cache: Option<&GlobalSignal<StateFrame<PaginatedList<T>>>>,
    sync_view_cache: Option<&GlobalSignal<HashMap<K, StateFrame<T>>>>,
    get_id: GetId,
    on_success: Option<OnSuccess>,
) -> Option<T>
where
    K: Eq + Hash + Copy + 'static,
    T: DeserializeOwned + Clone + PartialEq + 'static,
    Payload: Clone + 'static,
    F: Future<Output = Result<HttpResponse, HttpError>>,
    GetId: Fn(&T) -> K,
    OnSuccess: FnOnce(&T),
{
    {
        let mut map = state.write();
        map.entry(id)
            .or_insert_with(StateFrame::new)
            .set_loading_meta(Some(payload), None);
    }

    match send_future.await {
        Ok(response) => {
            if (200..300).contains(&response.status()) {
                match response.json::<T>().await {
                    Ok(parsed) => {
                        {
                            let mut map = state.write();
                            map.entry(id)
                                .or_insert_with(StateFrame::new)
                                .set_success(None, None);
                        }

                        // Sync list cache if provided
                        if let Some(list_cache) = sync_list_cache {
                            let mut list_frame = list_cache.write();
                            if let Some(list) = &mut list_frame.data {
                                if let Some(item) = list.data.iter_mut().find(|i| get_id(i) == id) {
                                    *item = parsed.clone();
                                }
                            }
                        }

                        // Sync view cache if provided
                        if let Some(view_cache) = sync_view_cache {
                            let mut view_map = view_cache.write();
                            view_map
                                .entry(id)
                                .or_insert_with(StateFrame::new)
                                .set_success(Some(parsed.clone()), None);
                        }

                        // Call optional success callback for custom logic
                        if let Some(callback) = on_success {
                            callback(&parsed);
                        }

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

/// Specialized version for removing items and syncing caches
/// Use this when sync_view_cache stores `StateFrame<T>` (not wrapped in Option)
pub async fn remove_state_abstraction<K, T, F, GetId, OnSuccess>(
    state: &GlobalSignal<HashMap<K, StateFrame>>,
    id: K,
    send_future: F,
    _parse_label: &str,
    sync_list_cache: Option<&GlobalSignal<StateFrame<PaginatedList<T>>>>,
    sync_view_cache: Option<&GlobalSignal<HashMap<K, StateFrame<T>>>>,
    get_id: GetId,
    on_success: Option<OnSuccess>,
) -> bool
where
    K: Eq + Hash + Copy + 'static,
    T: Clone + PartialEq + 'static,
    F: Future<Output = Result<HttpResponse, HttpError>>,
    GetId: Fn(&T) -> K,
    OnSuccess: FnOnce(),
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
                {
                    let mut map = state.write();
                    map.entry(id)
                        .or_insert_with(StateFrame::new)
                        .set_success(None, None);
                }

                // Sync list cache if provided - remove the item
                if let Some(list_cache) = sync_list_cache {
                    let mut list_frame = list_cache.write();
                    if let Some(list) = &mut list_frame.data {
                        list.data.retain(|item| get_id(item) != id);
                        // Update total count
                        if list.total > 0 {
                            list.total -= 1;
                        }
                    }
                }

                // Sync view cache if provided - remove the entry
                if let Some(view_cache) = sync_view_cache {
                    let mut view_map = view_cache.write();
                    view_map.remove(&id);
                }

                // Call optional success callback for custom logic
                if let Some(callback) = on_success {
                    callback();
                }

                true
            } else {
                let mut map = state.write();
                map.entry(id)
                    .or_insert_with(StateFrame::new)
                    .set_api_error(&response)
                    .await;
                false
            }
        }
        Err(e) => {
            let mut map = state.write();
            map.entry(id)
                .or_insert_with(StateFrame::new)
                .set_failed(Some(format!("Network error: {}", e)));
            false
        }
    }
}

/// Variant for Vec-based lists instead of PaginatedList
/// Use this when sync_view_cache stores `StateFrame<Option<T>>`
pub async fn remove_state_abstraction_vec<K, T, F, GetId, OnSuccess>(
    state: &GlobalSignal<HashMap<K, StateFrame>>,
    id: K,
    send_future: F,
    _parse_label: &str,
    sync_list_cache: Option<&GlobalSignal<StateFrame<Vec<T>>>>,
    sync_view_cache: Option<&GlobalSignal<HashMap<K, StateFrame<Option<T>>>>>,
    get_id: GetId,
    on_success: Option<OnSuccess>,
) -> bool
where
    K: Eq + Hash + Copy + 'static,
    T: Clone + PartialEq + 'static,
    F: Future<Output = Result<HttpResponse, HttpError>>,
    GetId: Fn(&T) -> K,
    OnSuccess: FnOnce(),
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
                {
                    let mut map = state.write();
                    map.entry(id)
                        .or_insert_with(StateFrame::new)
                        .set_success(None, None);
                }

                // Sync list cache if provided - remove the item
                if let Some(list_cache) = sync_list_cache {
                    let mut list_frame = list_cache.write();
                    if let Some(list) = &mut list_frame.data {
                        list.retain(|item| get_id(item) != id);
                    }
                }

                // Sync view cache if provided - remove the entry
                if let Some(view_cache) = sync_view_cache {
                    let mut view_map = view_cache.write();
                    view_map.remove(&id);
                }

                // Call optional success callback for custom logic
                if let Some(callback) = on_success {
                    callback();
                }

                true
            } else {
                let mut map = state.write();
                map.entry(id)
                    .or_insert_with(StateFrame::new)
                    .set_api_error(&response)
                    .await;
                false
            }
        }
        Err(e) => {
            let mut map = state.write();
            map.entry(id)
                .or_insert_with(StateFrame::new)
                .set_failed(Some(format!("Network error: {}", e)));
            false
        }
    }
}
