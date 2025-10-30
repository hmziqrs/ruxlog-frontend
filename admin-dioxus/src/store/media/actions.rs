use super::{Media, MediaListQuery, MediaState, MediaUploadPayload, UploadStatus};
use crate::services::http_client;
use crate::store::{
    list_state_abstraction, remove_state_abstraction, view_state_abstraction, PaginatedList,
    StateFrame,
};
use std::collections::HashMap;
use web_sys::{Blob, FormData, Url};

impl MediaState {
    /// Hybrid upload: returns blob URL immediately, uploads in background
    pub async fn upload(&self, payload: MediaUploadPayload) -> Result<String, String> {
        // 1. Create blob URL immediately for instant preview
        let blob: &Blob = payload.file.as_ref();
        let blob_url = Url::create_object_url_with_blob(blob)
            .map_err(|_| "Failed to create blob URL".to_string())?;

        // 2. Initialize tracking state
        {
            let mut status_map = self.upload_status.write();
            status_map.insert(blob_url.clone(), UploadStatus::Uploading);
        }
        {
            let mut progress_map = self.upload_progress.write();
            progress_map.insert(blob_url.clone(), 0.0);
        }
        {
            let mut blob_map = self.blob_to_media.write();
            blob_map.insert(blob_url.clone(), None);
        }

        // 3. Prepare multipart form data
        let form_data = FormData::new().map_err(|_| "Failed to create FormData".to_string())?;

        form_data
            .append_with_blob("file", &payload.file)
            .map_err(|_| "Failed to append file".to_string())?;

        if let Some(ref_type) = &payload.reference_type {
            form_data
                .append_with_str("reference_type", &ref_type.to_string())
                .map_err(|_| "Failed to append reference_type".to_string())?;
        }

        if let Some(width) = payload.width {
            form_data
                .append_with_str("width", &width.to_string())
                .map_err(|_| "Failed to append width".to_string())?;
        }

        if let Some(height) = payload.height {
            form_data
                .append_with_str("height", &height.to_string())
                .map_err(|_| "Failed to append height".to_string())?;
        }

        // 4. Upload in background
        let blob_url_clone = blob_url.clone();

        wasm_bindgen_futures::spawn_local(async move {
            use super::use_media;
            let media_state = use_media();
            match http_client::post_multipart("/media/v1/create", &form_data) {
                Ok(request) => match request.send().await {
                    Ok(response) => {
                        if response.ok() {
                            match response.json::<Media>().await {
                                Ok(media) => {
                                    // Success: update tracking
                                    {
                                        let mut status_map = media_state.upload_status.write();
                                        status_map
                                            .insert(blob_url_clone.clone(), UploadStatus::Success);
                                    }
                                    {
                                        let mut progress_map = media_state.upload_progress.write();
                                        progress_map.insert(blob_url_clone.clone(), 100.0);
                                    }
                                    {
                                        let mut blob_map = media_state.blob_to_media.write();
                                        blob_map.insert(blob_url_clone.clone(), Some(media));
                                    }

                                    // Refresh list
                                    media_state.list().await;
                                }
                                Err(e) => {
                                    let mut status_map = media_state.upload_status.write();
                                    status_map.insert(
                                        blob_url_clone,
                                        UploadStatus::Error(format!("Failed to parse response: {:?}", e)),
                                    );
                                }
                            }
                        } else {
                            let mut status_map = media_state.upload_status.write();
                            status_map.insert(
                                blob_url_clone,
                                UploadStatus::Error(format!("Upload failed: {}", response.status())),
                            );
                        }
                    }
                    Err(e) => {
                        let mut status_map = media_state.upload_status.write();
                        status_map.insert(
                            blob_url_clone,
                            UploadStatus::Error(format!("Request failed: {:?}", e)),
                        );
                    }
                },
                Err(e) => {
                    let mut status_map = media_state.upload_status.write();
                    status_map.insert(blob_url_clone, UploadStatus::Error(e));
                }
            }
        });

        // 5. Return blob URL immediately
        Ok(blob_url)
    }

    pub async fn remove(&self, id: i32) {
        let _ = remove_state_abstraction(
            &self.remove,
            id,
            http_client::post(&format!("/media/v1/delete/{}", id), &()).send(),
            "media",
            Some(&self.list),
            Some(&self.view),
            |media: &Media| media.id,
            None::<fn()>,
        )
        .await;
    }

    pub async fn list(&self) {
        let _ = list_state_abstraction::<PaginatedList<Media>>(
            &self.list,
            http_client::post("/media/v1/list/query", &serde_json::json!({})),
            "media",
        )
        .await;
    }

    pub async fn list_with_query(&self, query: MediaListQuery) {
        let _ = list_state_abstraction::<PaginatedList<Media>>(
            &self.list,
            http_client::post("/media/v1/list/query", &query),
            "media",
        )
        .await;
    }

    pub async fn view(&self, id: i32) {
        let _ = view_state_abstraction(
            &self.view,
            id,
            http_client::get(&format!("/media/v1/view/{}", id)).send(),
            "media",
            |media: &Media| media.clone(),
        )
        .await;
    }

    pub fn reset(&self) {
        *self.upload.write() = StateFrame::new();
        *self.remove.write() = HashMap::new();
        *self.list.write() = StateFrame::new();
        *self.view.write() = HashMap::new();
        *self.upload_progress.write() = HashMap::new();
        *self.upload_status.write() = HashMap::new();
        *self.blob_to_media.write() = HashMap::new();
    }

    // Helper methods for upload tracking

    /// Get the upload status for a blob URL
    pub fn get_upload_status(&self, blob_url: &str) -> Option<UploadStatus> {
        (*self.upload_status)().get(blob_url).cloned()
    }

    /// Get the uploaded media for a blob URL (if upload succeeded)
    pub fn get_uploaded_media(&self, blob_url: &str) -> Option<Media> {
        (*self.blob_to_media)()
            .get(blob_url)
            .and_then(|opt| opt.clone())
    }

    /// Get the upload progress percentage (0.0 - 100.0) for a blob URL
    pub fn get_upload_progress(&self, blob_url: &str) -> f64 {
        (*self.upload_progress)()
            .get(blob_url)
            .copied()
            .unwrap_or(0.0)
    }

    /// Check if an upload is complete (success or error)
    pub fn is_upload_complete(&self, blob_url: &str) -> bool {
        matches!(
            self.get_upload_status(blob_url),
            Some(UploadStatus::Success) | Some(UploadStatus::Error(_))
        )
    }

    /// Clean up tracking data for a blob URL (call after use)
    pub fn cleanup_blob(&self, blob_url: &str) {
        self.upload_progress.write().remove(blob_url);
        self.upload_status.write().remove(blob_url);
        self.blob_to_media.write().remove(blob_url);

        // Revoke the blob URL to free memory
        Url::revoke_object_url(blob_url).ok();
    }
}
