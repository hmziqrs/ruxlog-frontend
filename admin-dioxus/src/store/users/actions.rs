use super::{User, UsersAddPayload, UsersEditPayload, UsersState};
use crate::services::http_client;
use crate::store::StateFrame;
use std::collections::HashMap;

impl UsersState {
    pub async fn add(&self, payload: UsersAddPayload) {
        self.add.write().set_loading(None);
        let result = http_client::post("/admin/user/v1/create", &payload).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<User>().await {
                        Ok(user) => {
                            self.add.write().set_success(None, None);
                            let mut tmp = self.list.write();
                            let mut existing = tmp.data.clone().unwrap_or_default();
                            existing.insert(0, user);
                            tmp.set_success(Some(existing), None);
                        }
                        Err(e) => {
                            self.add.write().set_failed(Some(format!("Failed to parse user: {}", e)));
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

    pub async fn edit(&self, id: i32, payload: UsersEditPayload) {
        let mut edit_map = self.edit.write();
        edit_map.entry(id).or_insert_with(StateFrame::new).set_loading(None);
        let result = http_client::post(&format!("/admin/user/v1/update/{}", id), &payload).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<User>().await {
                        Ok(user) => {
                            edit_map.entry(id).or_insert_with(StateFrame::new).set_success(None, None);
                            let mut list = self.list.write();
                            let mut existing = list.data.clone().unwrap_or_default();
                            if let Some(item) = existing.iter_mut().find(|u| u.id == id) {
                                *item = user.clone();
                            }
                            list.set_success(Some(existing), None);
                        }
                        Err(e) => {
                            edit_map.entry(id).or_insert_with(StateFrame::new).set_failed(Some(format!("Failed to parse user: {}", e)));
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
        let result = http_client::post(&format!("/admin/user/v1/delete/{}", id), &()).send().await;
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

    pub async fn list(&self) {
        self.list.write().set_loading(None);
        let result = http_client::post("/admin/user/v1/list", &()).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Vec<User>>().await {
                        Ok(users) => {
                            self.list.write().set_success(Some(users.clone()), None);
                        }
                        Err(e) => {
                            self.list.write().set_failed(Some(format!("Failed to parse users: {}", e)));
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
        let result = http_client::get(&format!("/admin/user/v1/view/{}", id)).send().await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<User>().await {
                        Ok(user) => {
                            view_map.entry(id).or_insert_with(StateFrame::new).set_success(Some(Some(user.clone())), None);
                        }
                        Err(e) => {
                            view_map.entry(id).or_insert_with(StateFrame::new).set_failed(Some(format!("Failed to parse user: {}", e)));
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
