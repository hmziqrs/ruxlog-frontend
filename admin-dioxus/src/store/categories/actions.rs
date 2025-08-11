use super::{Category, CategoryAddPayload, CategoryEditPayload, CategoryListQuery, CategoryState};
use crate::services::http_client;
use crate::store::{PaginatedList, StateFrame};
use std::collections::HashMap;
use serde_json;

impl CategoryState {
    pub async fn add(&self, payload: CategoryAddPayload) {
        self.add.write().set_loading(None);
        let result = http_client::post("/category/v1/create", &payload).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Category>().await {
                        Ok(category) => {
                            self.add.write().set_success(None, None);
                            // Refresh the simple list so pickers are up-to-date
                            drop(category);
                            self.list().await;
                        }
                        Err(e) => {
                            self.add.write().set_failed(Some(format!("Failed to parse category: {}", e)));
                        }
                    }
                } else {
                    self.add.write().set_api_error(&response).await;
                }
            }
            Err(e) => {
                self.add.write().set_failed(Some(format!("Network error: {}", e)));
            }
        }
    }

    pub async fn edit(&self, id: i32, payload: CategoryEditPayload) {
        let mut edit_map = self.edit.write();
        edit_map.entry(id).or_insert_with(StateFrame::new).set_loading(None);
        let result = http_client::post(&format!("/category/v1/update/{}", id), &payload).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Category>().await {
                        Ok(category) => {
                            edit_map.entry(id).or_insert_with(StateFrame::new).set_success(None, None);
                            // Update paged cache if present
                            let mut lp = self.list_with_query.write();
                            if let Some(mut tmp_list) = lp.data.clone() {
                                if let Some(item) = tmp_list.data.iter_mut().find(|c| c.id == id) {
                                    *item = category.clone();
                                }
                                lp.set_success(Some(tmp_list), None);
                            }
                            drop(lp);
                            // Update simple cache if present
                            let mut ls = self.list.write();
                            if let Some(mut vec) = ls.data.clone() {
                                if let Some(item) = vec.iter_mut().find(|c| c.id == id) {
                                    *item = category.clone();
                                }
                                ls.set_success(Some(vec), None);
                            }
                            // Update view cache to keep detail in sync
                            let mut view_map = self.view.write();
                            view_map
                                .entry(id)
                                .or_insert_with(StateFrame::new)
                                .set_success(Some(Some(category.clone())), None);
                        }
                        Err(e) => {
                            edit_map.entry(id).or_insert_with(StateFrame::new).set_failed(Some(format!("Failed to parse category: {}", e)));
                        }
                    }
                } else {
                    edit_map.entry(id).or_insert_with(StateFrame::new).set_api_error(&response).await;
                }
            }
            Err(e) => {
                edit_map.entry(id).or_insert_with(StateFrame::new).set_failed(Some(format!("Network error: {}", e)));
            }
        }
    }

    pub async fn remove(&self, id: i32) {
        let mut remove_map = self.remove.write();
        remove_map.entry(id).or_insert_with(StateFrame::new).set_loading(None);
        let result = http_client::post(&format!("/category/v1/delete/{}", id), &()).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    remove_map.entry(id).or_insert_with(StateFrame::new).set_success(None, None);
                    // Release guard before awaiting
                    drop(remove_map);
                    // Refresh simple list for parity with add()
                    self.list().await;
                } else {
                    remove_map.entry(id).or_insert_with(StateFrame::new).set_api_error(&response).await;
                }
            }
            Err(e) => {
                remove_map.entry(id).or_insert_with(StateFrame::new).set_failed(Some(format!("Network error: {}", e)));
            }
        }
    }

    pub async fn list(&self) {
        self.list.write().set_loading(None);
        let result = http_client::get("/category/v1/list").send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Vec<Category>>().await {
                        Ok(categories) => {
                            self.list.write().set_success(Some(categories.clone()), None);
                        }
                        Err(e) => {
                            self.list.write().set_failed(Some(format!("Failed to parse categories: {}", e)));
                        }
                    }
                } else {
                    self.list.write().set_api_error(&response).await;
                }
            }
            Err(e) => {
                self.list.write().set_failed(Some(format!("Network error: {}", e)));
            }
        }
    }

    pub async fn list_with_query(&self, query: CategoryListQuery) {
        self.list_with_query.write().set_loading(None);
        let result = http_client::post("/category/v1/list/query", &query).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<PaginatedList<Category>>().await {
                        Ok(categories) => {
                            self.list_with_query.write().set_success(Some(categories.clone()), None);
                        }
                        Err(e) => {
                            self.list_with_query.write().set_failed(Some(format!("Failed to parse categories: {}", e)));
                        }
                    }
                } else {
                    self.list_with_query.write().set_api_error(&response).await;
                }
            }
            Err(e) => {
                self.list_with_query.write().set_failed(Some(format!("Network error: {}", e)));
            }
        }
    }

    pub async fn view(&self, id: i32) {
        let mut view_map = self.view.write();
        view_map.entry(id).or_insert_with(StateFrame::new).set_loading(None);
        let result = http_client::get(&format!("/category/v1/view/{}", id)).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Category>().await {
                        Ok(category) => {
                            view_map.entry(id).or_insert_with(StateFrame::new).set_success(Some(Some(category.clone())), None);
                        }
                        Err(e) => {
                            view_map.entry(id).or_insert_with(StateFrame::new).set_failed(Some(format!("Failed to parse category: {}", e)));
                        }
                    }
                } else {
                    view_map.entry(id).or_insert_with(StateFrame::new).set_api_error(&response).await;
                }
            }
            Err(e) => {
                view_map.entry(id).or_insert_with(StateFrame::new).set_failed(Some(format!("Network error: {}", e)));
            }
        }
    }

    pub fn reset(&self) {
        *self.add.write() = StateFrame::new();
        *self.edit.write() = HashMap::new();
        *self.remove.write() = HashMap::new();
        *self.list.write() = StateFrame::new();
        *self.list_with_query.write() = StateFrame::new();
        *self.view.write() = HashMap::new();
    }
}
