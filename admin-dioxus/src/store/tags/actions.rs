use dioxus::signals::ReadableExt;

use super::{Tag, TagsAddPayload, TagsEditPayload, TagsListQuery, TagsState};
use crate::services::http_client;
use crate::store::{
    list_state_abstraction, state_request_abstraction, view_state_abstraction, PaginatedList,
    StateFrame,
};
use std::collections::HashMap;

impl TagsState {
    pub async fn add(&self, payload: TagsAddPayload) {
        let meta_payload = payload.clone();
        let request = http_client::post("/tag/v1/create", &payload);
        let created = state_request_abstraction(
            &self.add,
            Some(meta_payload),
            request.send(),
            "tag",
            |_tag: &Tag| (None, None),
        )
        .await;

        if created.is_some() {
            self.list().await;
        }
    }

    pub async fn edit(&self, id: i32, payload: TagsEditPayload) {
        let mut edit_map = self.edit.write();
        edit_map
            .entry(id)
            .or_insert_with(StateFrame::new)
            .set_loading_meta(Some(payload.clone()), None);
        let result = http_client::post(&format!("/tag/v1/update/{}", id), &payload)
            .send()
            .await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    match response.json::<Tag>().await {
                        Ok(tag) => {
                            edit_map
                                .entry(id)
                                .or_insert_with(StateFrame::new)
                                .set_success(None, None);

                            if let Some(check_list_item) = &self.list.peek().data {
                                if !check_list_item.data.iter().any(|t| t.id == id) {
                                    let mut list_frame = self.list.write();
                                    if let Some(list) = &mut list_frame.data {
                                        if let Some(item) =
                                            list.data.iter_mut().find(|t| t.id == id)
                                        {
                                            *item = tag.clone();
                                        }
                                    }
                                    return;
                                }
                            }
                            if let Some(view_frame) = self.view.write().get_mut(&id) {
                                view_frame.set_success(Some(tag), None);
                            }
                        }
                        Err(e) => {
                            edit_map
                                .entry(id)
                                .or_insert_with(StateFrame::new)
                                .set_failed(Some(format!("Failed to parse tag: {}", e)));
                        }
                    }
                } else {
                    edit_map
                        .entry(id)
                        .or_insert_with(StateFrame::new)
                        .set_api_error(&response)
                        .await;
                }
            }
            Err(e) => {
                edit_map
                    .entry(id)
                    .or_insert_with(StateFrame::new)
                    .set_failed(Some(format!("Network error: {}", e)));
            }
        }
    }

    pub async fn remove(&self, id: i32) {
        let mut remove_map = self.remove.write();
        remove_map
            .entry(id)
            .or_insert_with(StateFrame::new)
            .set_loading(None);
        let result = http_client::post(&format!("/tag/v1/delete/{}", id), &())
            .send()
            .await;
        match result {
            Ok(response) => {
                if (200..300).contains(&response.status()) {
                    remove_map
                        .entry(id)
                        .or_insert_with(StateFrame::new)
                        .set_success(None, None);
                    // Release the write guard before awaiting to refresh the list
                    drop(remove_map);
                    // Keep list in sync after deletion (parity with add())
                    self.list().await;
                } else {
                    remove_map
                        .entry(id)
                        .or_insert_with(StateFrame::new)
                        .set_api_error(&response)
                        .await;
                }
            }
            Err(e) => {
                remove_map
                    .entry(id)
                    .or_insert_with(StateFrame::new)
                    .set_failed(Some(format!("Network error: {}", e)));
            }
        }
    }

    pub async fn list(&self) {
        let _ = list_state_abstraction::<PaginatedList<Tag>>(
            &self.list,
            http_client::post("/tag/v1/list/query", &serde_json::json!({})),
            "tags",
        )
        .await;
    }

    pub async fn list_with_query(&self, query: TagsListQuery) {
        let _ = list_state_abstraction::<PaginatedList<Tag>>(
            &self.list,
            http_client::post("/tag/v1/list/query", &query),
            "tags",
        )
        .await;
    }

    pub async fn view(&self, id: i32) {
        let _ = view_state_abstraction(
            &self.view,
            id,
            http_client::post(&format!("/tag/v1/view/{}", id), &()).send(),
            "tag",
            |tag: &Tag| tag.clone(),
        )
        .await;
    }

    pub fn reset(&self) {
        *self.add.write() = StateFrame::new();
        *self.edit.write() = HashMap::new();
        *self.remove.write() = HashMap::new();
        *self.list.write() = StateFrame::new();
        *self.view.write() = HashMap::new();
    }
}
