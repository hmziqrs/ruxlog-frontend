mod use_form;
mod use_previous;
mod use_tag_view;
mod use_category_view;
mod use_stateframe_toast;


pub use use_form::*;
pub use use_previous::use_previous;
pub use use_tag_view::{use_tag_view, TagViewState};
pub use use_category_view::{use_category_view, CategoryViewState as CategoryDetailViewState};
pub use use_stateframe_toast::{use_stateframe_toast, StateframeToastConfig};

use std::sync::atomic::{AtomicUsize, Ordering};
use dioxus::prelude::*;

/// Run some cleanup code when the component is unmounted if the effect was run.
pub fn use_effect_cleanup<F: FnOnce() + 'static>(#[allow(unused)] cleanup: F) {
    client!(dioxus::core::use_drop(cleanup))
}

pub fn use_unique_id() -> Signal<String> {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    #[allow(unused_mut)]
    let mut initial_value = use_hook(|| {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let id_str = format!("dxc-{id}");
        id_str
    });

    fullstack! {
        let server_id = dioxus::prelude::use_server_cached(move || {
            initial_value.clone()
        });
        initial_value = server_id;
    }
    use_signal(|| initial_value)
}
