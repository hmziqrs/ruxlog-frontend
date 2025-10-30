use super::{
    CompressParams, CropRegion, EditSession, EditorTool, ImageEditorState, ResizeParams,
    RotateParams,
};
use gloo_console;
use photon_rs::native::{open_image_from_bytes, save_image};
use photon_rs::transform::{resize, rotate};
use photon_rs::{PhotonImage, Rgb};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, File, HtmlCanvasElement, HtmlImageElement, Url};

impl ImageEditorState {
    /// Open the editor with a File or blob URL
    pub async fn open_editor(&self, file: Option<File>, blob_url: String) -> Result<(), String> {
        gloo_console::log!("[ImageEditor::open_editor] Opening editor with blob:", &blob_url);

        // Load the image to get dimensions
        let (width, height) = self.get_image_dimensions(&blob_url).await?;

        gloo_console::log!(
            "[ImageEditor::open_editor] Image dimensions:",
            width.to_string(),
            "x",
            height.to_string()
        );

        // Create edit session
        let session = EditSession {
            original_blob_url: blob_url.clone(),
            current_blob_url: blob_url.clone(),
            original_file: file,
            width,
            height,
        };

        // Update state
        *self.current_session.write() = Some(session);
        *self.is_open.write() = true;
        *self.active_tool.write() = EditorTool::None;
        *self.error_message.write() = None;

        // Initialize resize params with current dimensions
        let mut resize = self.resize_params.write();
        resize.width = width;
        resize.height = height;

        // Initialize crop region to cover the whole image
        let mut crop = self.crop_region.write();
        crop.x = 0;
        crop.y = 0;
        crop.width = width;
        crop.height = height;

        gloo_console::log!("[ImageEditor::open_editor] Editor opened successfully");
        Ok(())
    }

    /// Close the editor
    pub fn close_editor(&self) {
        gloo_console::log!("[ImageEditor::close_editor] Closing editor");

        *self.is_open.write() = false;
        *self.current_session.write() = None;
        *self.active_tool.write() = EditorTool::None;
        *self.error_message.write() = None;

        gloo_console::log!("[ImageEditor::close_editor] Editor closed");
    }

    /// Select a tool
    pub fn select_tool(&self, tool: EditorTool) {
        gloo_console::log!("[ImageEditor::select_tool] Selecting tool");
        *self.active_tool.write() = tool;
    }

    /// Apply crop operation
    pub async fn apply_crop(&self) -> Result<String, String> {
        gloo_console::log!("[ImageEditor::apply_crop] Starting crop operation");

        let session = (*self.current_session.write())
            .clone()
            .ok_or("No active editing session")?;
        let crop_region = (*self.crop_region.write()).clone();

        *self.is_processing.write() = true;

        let result = self
            .process_image(&session.current_blob_url, |mut img| {
                gloo_console::log!(
                    "[ImageEditor::apply_crop] Cropping:",
                    crop_region.x.to_string(),
                    crop_region.y.to_string(),
                    crop_region.width.to_string(),
                    crop_region.height.to_string()
                );

                // Use photon_rs crop
                img = photon_rs::transform::crop(
                    &mut img,
                    crop_region.x,
                    crop_region.y,
                    crop_region.width,
                    crop_region.height,
                );

                Ok(img)
            })
            .await;

        *self.is_processing.write() = false;

        match result {
            Ok(new_blob_url) => {
                gloo_console::log!("[ImageEditor::apply_crop] Crop applied successfully");
                let mut session_mut = self.current_session.write();
                if let Some(ref mut session) = *session_mut {
                    session.current_blob_url = new_blob_url.clone();
                    session.width = crop_region.width;
                    session.height = crop_region.height;
                }
                Ok(new_blob_url)
            }
            Err(e) => {
                gloo_console::error!("[ImageEditor::apply_crop] Failed:", &e);
                *self.error_message.write() = Some(e.clone());
                Err(e)
            }
        }
    }

    /// Apply resize operation
    pub async fn apply_resize(&self) -> Result<String, String> {
        gloo_console::log!("[ImageEditor::apply_resize] Starting resize operation");

        let session = (*self.current_session.write())
            .clone()
            .ok_or("No active editing session")?;
        let resize_params = (*self.resize_params.write()).clone();

        *self.is_processing.write() = true;

        let result = self
            .process_image(&session.current_blob_url, |mut img| {
                gloo_console::log!(
                    "[ImageEditor::apply_resize] Resizing to:",
                    resize_params.width.to_string(),
                    "x",
                    resize_params.height.to_string()
                );

                // Use photon_rs resize with SampleNearest algorithm
                img = resize(
                    &img,
                    resize_params.width,
                    resize_params.height,
                    photon_rs::transform::SamplingFilter::Nearest,
                );

                Ok(img)
            })
            .await;

        *self.is_processing.write() = false;

        match result {
            Ok(new_blob_url) => {
                gloo_console::log!("[ImageEditor::apply_resize] Resize applied successfully");
                let mut session_mut = self.current_session.write();
                if let Some(ref mut session) = *session_mut {
                    session.current_blob_url = new_blob_url.clone();
                    session.width = resize_params.width;
                    session.height = resize_params.height;
                }
                Ok(new_blob_url)
            }
            Err(e) => {
                gloo_console::error!("[ImageEditor::apply_resize] Failed:", &e);
                *self.error_message.write() = Some(e.clone());
                Err(e)
            }
        }
    }

    /// Apply rotate operation
    pub async fn apply_rotate(&self) -> Result<String, String> {
        gloo_console::log!("[ImageEditor::apply_rotate] Starting rotate operation");

        let session = (*self.current_session.write())
            .clone()
            .ok_or("No active editing session")?;
        let rotate_params = (*self.rotate_params.write()).clone();

        *self.is_processing.write() = true;

        let result = self
            .process_image(&session.current_blob_url, |mut img| {
                gloo_console::log!(
                    "[ImageEditor::apply_rotate] Rotating by:",
                    rotate_params.angle.to_string(),
                    "degrees"
                );

                // Use photon_rs rotate
                img = rotate(&img, rotate_params.angle);

                Ok(img)
            })
            .await;

        *self.is_processing.write() = false;

        match result {
            Ok(new_blob_url) => {
                gloo_console::log!("[ImageEditor::apply_rotate] Rotate applied successfully");
                let mut session_mut = self.current_session.write();
                if let Some(ref mut session) = *session_mut {
                    session.current_blob_url = new_blob_url.clone();
                    // Swap width/height if rotated by 90 or 270 degrees
                    if rotate_params.angle % 180 != 0 {
                        let temp = session.width;
                        session.width = session.height;
                        session.height = temp;
                    }
                }
                Ok(new_blob_url)
            }
            Err(e) => {
                gloo_console::error!("[ImageEditor::apply_rotate] Failed:", &e);
                *self.error_message.write() = Some(e.clone());
                Err(e)
            }
        }
    }

    /// Apply compression (re-encode with quality setting)
    pub async fn apply_compress(&self) -> Result<String, String> {
        gloo_console::log!("[ImageEditor::apply_compress] Starting compress operation");

        let session = (*self.current_session.write())
            .clone()
            .ok_or("No active editing session")?;
        let compress_params = (*self.compress_params.write()).clone();

        *self.is_processing.write() = true;

        let result = self
            .process_image_with_quality(&session.current_blob_url, compress_params.quality)
            .await;

        *self.is_processing.write() = false;

        match result {
            Ok(new_blob_url) => {
                gloo_console::log!("[ImageEditor::apply_compress] Compress applied successfully");
                let mut session_mut = self.current_session.write();
                if let Some(ref mut session) = *session_mut {
                    session.current_blob_url = new_blob_url.clone();
                }
                Ok(new_blob_url)
            }
            Err(e) => {
                gloo_console::error!("[ImageEditor::apply_compress] Failed:", &e);
                *self.error_message.write() = Some(e.clone());
                Err(e)
            }
        }
    }

    /// Reset to original image
    pub fn reset_to_original(&self) {
        gloo_console::log!("[ImageEditor::reset_to_original] Resetting to original");

        let mut session_mut = self.current_session.write();
        if let Some(ref mut session) = *session_mut {
            session.current_blob_url = session.original_blob_url.clone();
        }
    }

    /// Export the edited image as a File
    pub async fn export_as_file(&self, filename: String) -> Result<File, String> {
        gloo_console::log!("[ImageEditor::export_as_file] Exporting as file:", &filename);

        let session = (*self.current_session.write())
            .clone()
            .ok_or("No active editing session")?;

        // Fetch the blob from the URL
        let blob = self.fetch_blob(&session.current_blob_url).await?;

        // Create a new File from the blob
        let file = File::new_with_blob_sequence_and_options(
            &js_sys::Array::of1(&blob),
            &filename,
            web_sys::FilePropertyBag::new().type_("image/jpeg"),
        )
        .map_err(|e| format!("Failed to create file: {:?}", e))?;

        gloo_console::log!("[ImageEditor::export_as_file] File created successfully");
        Ok(file)
    }

    // Helper methods

    /// Get image dimensions from a blob URL
    async fn get_image_dimensions(&self, blob_url: &str) -> Result<(u32, u32), String> {
        let window = web_sys::window().ok_or("No window available")?;
        let document = window.document().ok_or("No document available")?;

        let img = document
            .create_element("img")
            .map_err(|e| format!("Failed to create img element: {:?}", e))?
            .dyn_into::<HtmlImageElement>()
            .map_err(|e| format!("Failed to cast to HtmlImageElement: {:?}", e))?;

        img.set_src(blob_url);

        // Wait for image to load
        let (sender, receiver) = futures_channel::oneshot::channel();
        let mut sender = Some(sender);

        let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            if let Some(sender) = sender.take() {
                let _ = sender.send(());
            }
        }) as Box<dyn FnMut()>);

        img.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();

        receiver
            .await
            .map_err(|_| "Failed to load image".to_string())?;

        Ok((img.width(), img.height()))
    }

    /// Process an image with a transformation function
    async fn process_image<F>(&self, blob_url: &str, transform: F) -> Result<String, String>
    where
        F: FnOnce(PhotonImage) -> Result<PhotonImage, String>,
    {
        // Fetch image bytes
        let bytes = self.fetch_bytes(blob_url).await?;

        // Convert to PhotonImage
        let mut img = open_image_from_bytes(&bytes)
            .map_err(|e| format!("Failed to open image: {:?}", e))?;

        // Apply transformation
        img = transform(img)?;

        // Convert back to blob URL
        let output_bytes = save_image(img, "jpeg")
            .map_err(|e| format!("Failed to save image: {:?}", e))?;

        self.bytes_to_blob_url(&output_bytes, "image/jpeg").await
    }

    /// Process image with quality compression
    async fn process_image_with_quality(
        &self,
        blob_url: &str,
        quality: u8,
    ) -> Result<String, String> {
        // For now, we'll use canvas to re-encode with quality
        // photon-rs doesn't directly support quality parameter
        let window = web_sys::window().ok_or("No window available")?;
        let document = window.document().ok_or("No document available")?;

        // Create image element
        let img = document
            .create_element("img")
            .map_err(|e| format!("Failed to create img: {:?}", e))?
            .dyn_into::<HtmlImageElement>()
            .map_err(|e| format!("Failed to cast to img: {:?}", e))?;

        img.set_src(blob_url);

        // Wait for load
        let (sender, receiver) = futures_channel::oneshot::channel();
        let mut sender = Some(sender);
        let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            if let Some(sender) = sender.take() {
                let _ = sender.send(());
            }
        }) as Box<dyn FnMut()>);
        img.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
        receiver
            .await
            .map_err(|_| "Failed to load image".to_string())?;

        // Create canvas
        let canvas = document
            .create_element("canvas")
            .map_err(|e| format!("Failed to create canvas: {:?}", e))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|e| format!("Failed to cast to canvas: {:?}", e))?;

        canvas.set_width(img.width());
        canvas.set_height(img.height());

        let context = canvas
            .get_context("2d")
            .map_err(|e| format!("Failed to get context: {:?}", e))?
            .ok_or("No context")?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .map_err(|e| format!("Failed to cast context: {:?}", e))?;

        context
            .draw_image_with_html_image_element(&img, 0.0, 0.0)
            .map_err(|e| format!("Failed to draw image: {:?}", e))?;

        // Convert to blob with quality
        let quality_f64 = (quality as f64) / 100.0;
        let blob_promise = canvas
            .to_blob_with_type_and_quality("image/jpeg", quality_f64)
            .map_err(|e| format!("Failed to create blob: {:?}", e))?;

        let blob_jsvalue = JsFuture::from(blob_promise)
            .await
            .map_err(|e| format!("Failed to convert to blob: {:?}", e))?;

        let blob: Blob = blob_jsvalue
            .dyn_into()
            .map_err(|e| format!("Failed to cast to Blob: {:?}", e))?;

        Url::create_object_url_with_blob(&blob)
            .map_err(|e| format!("Failed to create blob URL: {:?}", e))
    }

    /// Fetch bytes from a blob URL
    async fn fetch_bytes(&self, blob_url: &str) -> Result<Vec<u8>, String> {
        let window = web_sys::window().ok_or("No window available")?;

        let response = JsFuture::from(window.fetch_with_str(blob_url))
            .await
            .map_err(|e| format!("Failed to fetch: {:?}", e))?;

        let response: web_sys::Response = response
            .dyn_into()
            .map_err(|e| format!("Failed to cast response: {:?}", e))?;

        let blob = JsFuture::from(
            response
                .blob()
                .map_err(|e| format!("Failed to get blob: {:?}", e))?,
        )
        .await
        .map_err(|e| format!("Failed to convert to blob: {:?}", e))?;

        let blob: Blob = blob
            .dyn_into()
            .map_err(|e| format!("Failed to cast to Blob: {:?}", e))?;

        let array_buffer = JsFuture::from(blob.array_buffer())
            .await
            .map_err(|e| format!("Failed to get array buffer: {:?}", e))?;

        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
        Ok(uint8_array.to_vec())
    }

    /// Fetch blob from a blob URL
    async fn fetch_blob(&self, blob_url: &str) -> Result<Blob, String> {
        let window = web_sys::window().ok_or("No window available")?;

        let response = JsFuture::from(window.fetch_with_str(blob_url))
            .await
            .map_err(|e| format!("Failed to fetch: {:?}", e))?;

        let response: web_sys::Response = response
            .dyn_into()
            .map_err(|e| format!("Failed to cast response: {:?}", e))?;

        let blob = JsFuture::from(
            response
                .blob()
                .map_err(|e| format!("Failed to get blob: {:?}", e))?,
        )
        .await
        .map_err(|e| format!("Failed to convert to blob: {:?}", e))?;

        blob.dyn_into()
            .map_err(|e| format!("Failed to cast to Blob: {:?}", e))
    }

    /// Convert bytes to blob URL
    async fn bytes_to_blob_url(&self, bytes: &[u8], mime_type: &str) -> Result<String, String> {
        let uint8_array = js_sys::Uint8Array::from(bytes);
        let blob_parts = js_sys::Array::new();
        blob_parts.push(&uint8_array);

        let mut blob_options = web_sys::BlobPropertyBag::new();
        blob_options.type_(mime_type);

        let blob = Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_options)
            .map_err(|e| format!("Failed to create blob: {:?}", e))?;

        Url::create_object_url_with_blob(&blob)
            .map_err(|e| format!("Failed to create blob URL: {:?}", e))
    }
}
