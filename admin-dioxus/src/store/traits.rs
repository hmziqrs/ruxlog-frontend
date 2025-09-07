use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::store::{StateFrame, PaginatedList};
use crate::types::SortParam;
use chrono::{DateTime, Utc};

/// Generic trait for list query parameters
pub trait ListQuery: Clone + Default + Serialize + for<'de> Deserialize<'de> + PartialEq {
    /// Create a new instance with default values (typically page = 1)
    fn new() -> Self;
    
    /// Get the current page number
    fn page(&self) -> u64;
    
    /// Set the page number
    fn set_page(&mut self, page: u64);
    
    /// Get the search query
    fn search(&self) -> Option<String>;
    
    /// Set the search query
    fn set_search(&mut self, search: Option<String>);
    
    /// Get the sort parameters
    fn sorts(&self) -> Option<Vec<SortParam>>;
    
    /// Set the sort parameters
    fn set_sorts(&mut self, sorts: Option<Vec<SortParam>>);
}

/// Generic trait for store list operations
pub trait ListStore<T, Q>
where
    T: Clone + PartialEq + 'static,
    Q: ListQuery,
{
    /// Get the list state frame
    fn list_frame(&self) -> &GlobalSignal<StateFrame<PaginatedList<T>>>;
    
    /// Load the list with default query
    async fn fetch_list(&self);
    
    /// Load the list with a specific query
    async fn fetch_list_with_query(&self, query: Q);
}

/// Base structure for common list query fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BaseListQuery {
    pub page: u64,
    pub search: Option<String>,
    pub sorts: Option<Vec<SortParam>>,
    pub created_at_gt: Option<DateTime<Utc>>,
    pub created_at_lt: Option<DateTime<Utc>>,
    pub updated_at_gt: Option<DateTime<Utc>>,
    pub updated_at_lt: Option<DateTime<Utc>>,
}

impl Default for BaseListQuery {
    fn default() -> Self {
        Self {
            page: 1,
            search: None,
            sorts: None,
            created_at_gt: None,
            created_at_lt: None,
            updated_at_gt: None,
            updated_at_lt: None,
        }
    }
}

impl BaseListQuery {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ListQuery for BaseListQuery {
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
