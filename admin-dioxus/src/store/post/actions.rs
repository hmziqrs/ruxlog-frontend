use super::{Post, PostCreatePayload, PostEditPayload, PostFilters, PostState};
use crate::services::http_client;
use crate::store::StateFrame;
use std::collections::HashMap;

impl PostState {
    pub async fn list(&self) {
        self.list.write().set_loading(None);
        let result = http_client::post("/post/v1/list/query", &serde_json::json!({})).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Vec<Post>>().await {
                        Ok(posts) => {
                            self.list.write().set_success(Some(posts), None);
                        }
                        Err(e) => {
                            self.list.write().set_failed(Some(format!("Failed to parse posts: {}", e)));
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

    pub async fn add(&self, payload: PostCreatePayload) {
        self.add.write().set_loading(None);
        let result = http_client::post("/post/v1/create", &payload).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    self.add.write().set_success(None, None);
                } else {
                    self.add.write().set_api_error(&response).await;
                }
            }
            Err(e) => {
                self.add.write().set_failed(Some(format!("Network error: {}", e)));
            }
        }
    }

    pub async fn edit(&self, id: i32, payload: PostEditPayload) {
        let mut edit_map = self.edit.write();
        edit_map.entry(id).or_insert_with(StateFrame::new).set_loading(None);
        let result = http_client::post(&format!("/post/v1/update/{}", id), &payload).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Post>().await {
                        Ok(post) => {
                            edit_map.entry(id).or_insert_with(StateFrame::new).set_success(None, None);
                            // Optionally update list/view here if needed
                        }
                        Err(e) => {
                            edit_map.entry(id).or_insert_with(StateFrame::new).set_failed(Some(format!("Failed to parse post: {}", e)));
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
        let result = http_client::post(&format!("/post/v1/delete/{}", id), &()).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    remove_map.entry(id).or_insert_with(StateFrame::new).set_success(None, None);
                } else {
                    remove_map.entry(id).or_insert_with(StateFrame::new).set_api_error(&response).await;
                }
            }
            Err(e) => {
                remove_map.entry(id).or_insert_with(StateFrame::new).set_failed(Some(format!("Network error: {}", e)));
            }
        }
    }

    pub async fn bulk_remove(&self) {
        self.bulk_remove.write().set_loading(None);
        // Implement your API call here
        // let result = http_client::post("/post/v1/bulk_delete", &()).send().await;
        // ...handle result...
        self.bulk_remove.write().set_success(None, None);
    }

    pub async fn view(&self, id: i32) {
        let mut view_map = self.view.write();
        view_map.entry(id).or_insert_with(StateFrame::new).set_loading(None);
        let result = http_client::post(&format!("/post/v1/view/{}", id), &()).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Post>().await {
                        Ok(post) => {
                            view_map.entry(id).or_insert_with(StateFrame::new).set_success(Some(Some(post)), None);
                        }
                        Err(e) => {
                            view_map.entry(id).or_insert_with(StateFrame::new).set_failed(Some(format!("Failed to parse post: {}", e)));
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
        *self.view.write() = HashMap::new();
        *self.list.write() = StateFrame::new();
        *self.add.write() = StateFrame::new();
        *self.edit.write() = HashMap::new();
        *self.remove.write() = HashMap::new();
        *self.bulk_remove.write() = StateFrame::new();
        *self.filters.write() = PostFilters::default();
    }
}