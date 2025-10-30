use crate::store::{ListQuery, ListStore, PaginatedList, StateFrame};
use crate::types::SortParam;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use web_sys::File;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum MediaReference {
    Post,
    Category,
    User,
}

impl fmt::Display for MediaReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            MediaReference::Post => "Post",
            MediaReference::Category => "Category",
            MediaReference::User => "User",
        };
        f.write_str(label)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UploadStatus {
    Uploading,
    Success,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Media {
    pub id: i32,
    pub object_key: String,
    pub file_url: String,
    pub mime_type: String,
    pub size: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub extension: Option<String>,
    pub uploader_id: Option<i32>,
    pub reference_type: Option<MediaReference>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Media {
    fn default() -> Self {
        Self {
            id: 0,
            object_key: String::new(),
            file_url: String::new(),
            mime_type: String::new(),
            size: 0,
            width: None,
            height: None,
            extension: None,
            uploader_id: None,
            reference_type: None,
            created_at: DateTime::<Utc>::from_timestamp(0, 0).unwrap_or_else(|| Utc::now()),
            updated_at: DateTime::<Utc>::from_timestamp(0, 0).unwrap_or_else(|| Utc::now()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MediaListQuery {
    pub page: u64,
    pub search: Option<String>,
    pub sorts: Option<Vec<SortParam>>,
    pub reference_type: Option<MediaReference>,
    pub uploader_id: Option<i32>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub created_at_gt: Option<DateTime<Utc>>,
    pub created_at_lt: Option<DateTime<Utc>>,
    pub updated_at_gt: Option<DateTime<Utc>>,
    pub updated_at_lt: Option<DateTime<Utc>>,
}

impl MediaListQuery {
    pub fn new() -> Self {
        Self {
            page: 1,
            ..Default::default()
        }
    }
}

impl ListQuery for MediaListQuery {
    fn new() -> Self {
        Self::new()
    }

    fn page(&self) -> u64 {
        self.page
    }

    fn set_page(&mut self, page: u64) {
        self.page = page;
    }

    fn search(&self) -> Option<String> {
        self.search.clone()
    }

    fn set_search(&mut self, search: Option<String>) {
        self.search = search;
    }

    fn sorts(&self) -> Option<Vec<SortParam>> {
        self.sorts.clone()
    }

    fn set_sorts(&mut self, sorts: Option<Vec<SortParam>>) {
        self.sorts = sorts;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MediaUploadPayload {
    pub file: File,
    pub reference_type: Option<MediaReference>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

pub struct MediaState {
    pub upload: GlobalSignal<StateFrame<(), MediaUploadPayload>>,
    pub remove: GlobalSignal<HashMap<i32, StateFrame>>,
    pub list: GlobalSignal<StateFrame<PaginatedList<Media>>>,
    pub view: GlobalSignal<HashMap<i32, StateFrame<Media>>>,
    // Upload tracking
    pub upload_progress: GlobalSignal<HashMap<String, f64>>, // blob URL -> progress %
    pub upload_status: GlobalSignal<HashMap<String, UploadStatus>>, // blob URL -> status
    pub blob_to_media: GlobalSignal<HashMap<String, Option<Media>>>, // blob URL -> uploaded Media
}

impl MediaState {
    pub fn new() -> Self {
        Self {
            upload: GlobalSignal::new(|| StateFrame::new()),
            remove: GlobalSignal::new(|| HashMap::new()),
            list: GlobalSignal::new(|| StateFrame::new()),
            view: GlobalSignal::new(|| HashMap::new()),
            upload_progress: GlobalSignal::new(|| HashMap::new()),
            upload_status: GlobalSignal::new(|| HashMap::new()),
            blob_to_media: GlobalSignal::new(|| HashMap::new()),
        }
    }
}

static MEDIA_STATE: std::sync::OnceLock<MediaState> = std::sync::OnceLock::new();

impl ListStore<Media, MediaListQuery> for MediaState {
    fn list_frame(&self) -> &GlobalSignal<StateFrame<PaginatedList<Media>>> {
        &self.list
    }

    async fn fetch_list(&self) {
        self.list().await;
    }

    async fn fetch_list_with_query(&self, query: MediaListQuery) {
        self.list_with_query(query).await;
    }
}

pub fn use_media() -> &'static MediaState {
    MEDIA_STATE.get_or_init(|| MediaState::new())
}
