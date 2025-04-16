use super::{Tag, TagAddPayload, TagEditPayload, TagState};
use crate::services::http_client;
use crate::store::StateFrame;
use std::collections::HashMap;

impl TagState {
    pub async fn add(&self, payload: TagAddPayload) {
        self.add.write().set_loading(None);
        let result = http_client::post("/tag/v1/create", &payload).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Tag>().await {
                        Ok(tag) => {
                            self.add.write().set_success(None, None);
                            self.data_view.write().insert(tag.id, tag.clone());
                            let mut list = self.data_list.write();
                            list.insert(0, tag);
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

    pub async fn edit(&self, id: i32, payload: TagEditPayload) {
        let mut edit_map = self.edit.write();
        edit_map.entry(id).or_insert_with(StateFrame::new).set_loading(None);
        let result = http_client::post(&format!("/tag/v1/update/{}", id), &payload).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Tag>().await {
                        Ok(tag) => {
                            edit_map.entry(id).or_insert_with(StateFrame::new).set_success(None, None);
                            self.data_view.write().insert(tag.id, tag.clone());
                            let mut list = self.data_list.write();
                            for item in list.iter_mut() {
                                if item.id == tag.id {
                                    *item = tag.clone();
                                }
                            }
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
                    let mut list = self.data_list.write();
                    list.retain(|item| item.id != id);
                    self.data_view.write().clear();
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
        let result = http_client::get("/tag/v1/list").send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Vec<Tag>>().await {
                        Ok(tags) => {
                            self.list.write().set_success(Some(tags.clone()), None);
                            *self.data_list.write() = tags;
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
        let result = http_client::get(&format!("/tag/v1/view/{}", id)).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Tag>().await {
                        Ok(tag) => {
                            view_map.entry(id).or_insert_with(StateFrame::new).set_success(Some(Some(tag.clone())), None);
                            self.data_view.write().insert(tag.id, tag);
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
        *self.data_add.write() = None;
        *self.data_edit.write() = None;
        *self.data_remove.write() = None;
        *self.data_list.write() = vec![];
        *self.data_view.write() = HashMap::new();
    }
}
