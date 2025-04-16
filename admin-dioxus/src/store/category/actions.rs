use super::{Category, CategoryAddPayload, CategoryEditPayload, CategoryState};
use crate::services::http_client;
use crate::store::StateFrame;
use std::collections::HashMap;

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
                            self.data_view.write().insert(category.id, category.clone());
                            let mut list = self.data_list.write();
                            list.insert(0, category);
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
                            self.data_view.write().insert(category.id, category.clone());
                            let mut list = self.data_list.write();
                            for item in list.iter_mut() {
                                if item.id == category.id {
                                    *item = category.clone();
                                }
                            }
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
        let result = http_client::get("/category/v1/list").send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Vec<Category>>().await {
                        Ok(categories) => {
                            self.list.write().set_success(Some(categories.clone()), None);
                            *self.data_list.write() = categories;
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
                            self.data_view.write().insert(category.id, category);
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
        *self.view.write() = HashMap::new();
        *self.data_add.write() = None;
        *self.data_edit.write() = None;
        *self.data_remove.write() = None;
        *self.data_list.write() = vec![];
        *self.data_view.write() = HashMap::new();
    }
}
