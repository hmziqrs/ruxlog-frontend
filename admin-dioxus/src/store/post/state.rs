use crate::store::{PaginatedList, StateFrame};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostAuthor {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostTag {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub excerpt: Option<String>,
    // #[serde(rename = "featured_image")]
    pub featured_image: Option<String>,
    pub published_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub author: PostAuthor,
    pub category: PostCategory,
    pub tags: Vec<PostTag>,
    pub likes_count: i32,
    pub view_count: i32,
    pub comment_count: i32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PostFilters {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub ascending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PostCreatePayload {
    pub title: String,
    pub content: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub featured_image_url: Option<String>,
    pub is_published: Option<bool>,
    pub category_id: Option<i32>,
    pub tag_ids: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PostEditPayload {
    pub title: Option<String>,
    pub content: Option<String>,
    pub slug: Option<String>,
    pub excerpt: Option<String>,
    pub featured_image_url: Option<String>,
    pub is_published: Option<bool>,
    pub category_id: Option<i32>,
    pub tag_ids: Option<Vec<i32>>,
}

pub struct PostState {
    pub view: GlobalSignal<std::collections::HashMap<i32, StateFrame<Option<Post>>>>,
    pub list: GlobalSignal<StateFrame<PaginatedList<Post>>>,
    pub add: GlobalSignal<StateFrame<()>>,
    pub edit: GlobalSignal<std::collections::HashMap<i32, StateFrame<()>>>,
    pub remove: GlobalSignal<std::collections::HashMap<i32, StateFrame<()>>>,
    pub bulk_remove: GlobalSignal<StateFrame<()>>,
    pub filters: GlobalSignal<PostFilters>,
}

impl PostState {
    pub fn new() -> Self {
        PostState {
            view: GlobalSignal::new(|| std::collections::HashMap::new()),
            list: GlobalSignal::new(|| StateFrame::new()),
            add: GlobalSignal::new(|| StateFrame::new()),
            edit: GlobalSignal::new(|| std::collections::HashMap::new()),
            remove: GlobalSignal::new(|| std::collections::HashMap::new()),
            bulk_remove: GlobalSignal::new(|| StateFrame::new()),
            filters: GlobalSignal::new(|| PostFilters::default()),
        }
    }
}

static POST_STATE: std::sync::OnceLock<PostState> = std::sync::OnceLock::new();

pub fn use_post() -> &'static PostState {
    POST_STATE.get_or_init(|| PostState::new())
}
