//! Sonner (Dioxus) State — Phase 0/1 scaffold
//! Provides handle types and context contracts. Provider implementation comes in Phase 2.

use dioxus::prelude::*;
use std::collections::VecDeque;
use std::future::Future;

use super::types::{
    ToastOptions, ToastT, ToastType, DEFAULT_TOAST_LIFETIME_MS, Position, HeightT, ToasterProps,
    PromiseConfig,
};

// Callback types used by the provider (wired in Phase 2)
type AddToastCallback = Callback<ToastT>;
type UpdateToastCallback = Callback<ToastT>;
type DismissToastCallback = Callback<u64>;
type DeleteToastCallback = Callback<u64>;

#[derive(Clone)]
pub struct SonnerCtx {
    // Signals (Phase 2: exposed for future stacking/controls)
    pub toasts: Signal<VecDeque<ToastT>>, // ordered list
    pub heights: Signal<Vec<HeightT>>,    // measurements (unused in Phase 2)
    pub interacting: Signal<bool>,        // pointer within region
    pub hidden: Signal<bool>,             // document.hidden flag (Phase 3)
    pub defaults: ToasterProps,           // effective defaults

    // Imperative controls
    pub add_toast: AddToastCallback,
    pub update_toast: UpdateToastCallback,
    pub dismiss_toast: DismissToastCallback,
    pub delete_toast: DeleteToastCallback,
}

/// Public handle returned by the hook to interact with Sonner provider
#[derive(Clone, Copy)]
pub struct SonnerToasts {
    add_toast: AddToastCallback,
    update_toast: UpdateToastCallback,
    dismiss_toast: DismissToastCallback,
    delete_toast: DeleteToastCallback,
}

impl SonnerToasts {
    /// Show a toast with given type and options.
    pub fn show(&self, title: String, toast_type: ToastType, options: ToastOptions) {
        let id = next_toast_id();
        let toast = ToastT {
            id,
            toaster_id: options.toaster_id.clone(),
            title: Some(title),
            toast_type,
            icon: options.icon.clone(),
            description: None,
            duration_ms: options
                .duration_ms
                .or(Some(DEFAULT_TOAST_LIFETIME_MS)),
            delete: false,
            close_button: options.close_button.unwrap_or(true),
            dismissible: true,
            action: None,
            cancel: None,
            class_name: options.class_name.clone(),
            class_names: options.class_names.clone(),
            position: Position::BottomRight,
            test_id: None,
            on_auto_close: options.on_auto_close.clone(),
        };
        self.add_toast.call(toast);
    }

    pub fn success(&self, title: String, options: ToastOptions) {
        self.show(title, ToastType::Success, options)
    }
    pub fn error(&self, title: String, options: ToastOptions) {
        self.show(title, ToastType::Error, options)
    }
    pub fn warning(&self, title: String, options: ToastOptions) {
        self.show(title, ToastType::Warning, options)
    }
    pub fn info(&self, title: String, options: ToastOptions) {
        self.show(title, ToastType::Info, options)
    }
    pub fn loading(&self, title: String, options: ToastOptions) {
        self.show(title, ToastType::Loading, options)
    }

    pub fn dismiss(&self, id: u64) {
        self.dismiss_toast.call(id)
    }
    pub fn delete(&self, id: u64) {
        self.delete_toast.call(id)
    }

    /// Promise-based toast flow (Phase 9): shows a loading toast, then updates to success or error.
    /// This simplified variant expects a future yielding Result<(), ()> and static messages.
    pub fn promise<F>(&self, fut: F, config: PromiseConfig, options: ToastOptions)
    where
        F: Future<Output = Result<(), ()>> + 'static,
    {
        let id = next_toast_id();

        // Insert loading toast (no duration to keep it until resolution)
        let mut loading = ToastT {
            id,
            toaster_id: options.toaster_id.clone(),
            title: Some(config.loading.clone()),
            toast_type: ToastType::Loading,
            icon: options.icon.clone(),
            description: None,
            duration_ms: None,
            delete: false,
            close_button: options.close_button.unwrap_or(true),
            dismissible: true,
            action: None,
            cancel: None,
            class_name: options.class_name.clone(),
            class_names: options.class_names.clone(),
            position: Position::BottomRight,
            test_id: None,
            on_auto_close: options.on_auto_close.clone(),
        };
        self.add_toast.call(loading.clone());

        let update = self.update_toast;
        spawn(async move {
            let res = fut.await;
            match res {
                Ok(()) => {
                    loading.toast_type = ToastType::Success;
                    loading.title = Some(config.success.clone());
                    loading.duration_ms = Some(DEFAULT_TOAST_LIFETIME_MS);
                }
                Err(()) => {
                    loading.toast_type = ToastType::Error;
                    loading.title = Some(config.error.clone());
                    loading.duration_ms = Some(DEFAULT_TOAST_LIFETIME_MS);
                }
            }
            update.call(loading);
        });
    }
}

/// Hook used by components to obtain a Sonner handle
pub fn use_sonner() -> SonnerToasts {
    use_hook(consume_sonner)
}

/// Consume the provider context and produce a handle
pub fn consume_sonner() -> SonnerToasts {
    let ctx = consume_context::<SonnerCtx>();
    SonnerToasts {
        add_toast: ctx.add_toast,
        update_toast: ctx.update_toast,
        dismiss_toast: ctx.dismiss_toast,
        delete_toast: ctx.delete_toast,
    }
}

/// Generate unique toast IDs
fn next_toast_id() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT_ID: AtomicU64 = AtomicU64::new(1);
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}
