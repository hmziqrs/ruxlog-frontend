use super::{Tag, TagsAddPayload, TagsEditPayload, TagsListQuery, TagsState};
use crate::services::http_client;
use crate::store::{exec_json_to_state, PaginatedList, StateFrame};
use std::collections::HashMap;

impl TagsState {
    pub async fn add(&self, payload: TagsAddPayload) {
        self.add.write().set_loading_meta(Some(payload.clone()), None);
        let result = http_client::post("/tag/v1/create", &payload).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Tag>().await {
                        Ok(_) => {
                            self.add.write().set_success(None, None);
                            self.list().await;
                        }
                        Err(e) => {
                            self.add.write().set_failed(Some(format!("Failed to parse tag: {}", e)));
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

    pub async fn edit(&self, id: i32, payload: TagsEditPayload) {
        let mut edit_map = self.edit.write();
        edit_map.entry(id).or_insert_with(StateFrame::new).set_loading(None);
        let result = http_client::post(&format!("/tag/v1/update/{}", id), &payload).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Tag>().await {
                        Ok(tag) => {
                            edit_map.entry(id).or_insert_with(StateFrame::new).set_success(None, None);
                            // Update list cache
                            let mut list = self.list.write();
                            let mut tmp_list = list.data.clone().unwrap();
                            if let Some(item) = tmp_list.data.iter_mut().find(|t| t.id == id) {
                                *item = tag.clone();
                            }
                            list.set_success(Some(tmp_list), None);
                            drop(list);
                            // Update view cache to keep detail in sync
                            let mut view_map = self.view.write();
                            view_map
                                .entry(id)
                                .or_insert_with(StateFrame::new)
                                .set_success(Some(tag.clone()), None);
                        }
                        Err(e) => {
                            edit_map.entry(id).or_insert_with(StateFrame::new).set_failed(Some(format!("Failed to parse tag: {}", e)));
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
        let result = http_client::post(&format!("/tag/v1/delete/{}", id), &()).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    remove_map.entry(id).or_insert_with(StateFrame::new).set_success(None, None);
                    // Release the write guard before awaiting to refresh the list
                    drop(remove_map);
                    // Keep list in sync after deletion (parity with add())
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
        let empty_body = "{}".to_string();
        let _ = exec_json_to_state::<PaginatedList<Tag>>(
            &self.list,
            http_client::post("/tag/v1/list/query", &empty_body),
            "tags",
        )
        .await;
    }

    pub async fn list_with_query(&self, query: TagsListQuery) {
        self.list.write().set_loading(None);
        let result = http_client::post("/tag/v1/list/query", &query).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<PaginatedList<Tag>>().await {
                        Ok(tags) => {
                            self.list.write().set_success(Some(tags.clone()), None);
                        }
                        Err(e) => {
                            self.list.write().set_failed(Some(format!("Failed to parse tags: {}", e)));
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

    pub async fn view(&self, id: i32) {
        let mut view_map = self.view.write();
        view_map.entry(id).or_insert_with(StateFrame::new).set_loading(None);
        let result = http_client::post(&format!("/tag/v1/view/{}", id), &()).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Tag>().await {
                        Ok(tag) => {
                            view_map.entry(id).or_insert_with(StateFrame::new).set_success(Some(tag.clone()), None);
                        }
                        Err(e) => {
                            view_map.entry(id).or_insert_with(StateFrame::new).set_failed(Some(format!("Failed to parse tag: {}", e)));
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
        *self.view.write() = HashMap::new();
    }
}
