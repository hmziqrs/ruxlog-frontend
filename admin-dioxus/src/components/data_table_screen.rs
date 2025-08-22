use dioxus::prelude::*;

use crate::components::{
    ListErrorBanner, ListErrorBannerProps, LoadingOverlay, PageHeader, PageHeaderProps, Pagination,
};
use crate::store::{PaginatedList, StateFrame};
use crate::ui::shadcn::Card;

#[derive(Props, PartialEq, Clone)]
pub struct DataTableScreenProps<T: Clone + PartialEq + 'static> {
    /// State frame carrying paginated data for the table (clone of state)
    pub frame: StateFrame<PaginatedList<T>>,

    /// Page header configuration (spread as `PageHeader { ..props }`)
    #[props(optional)]
    pub header: Option<PageHeaderProps>,

    /// Error banner configuration (spread as `ListErrorBanner { ..props }`)
    #[props(optional)]
    pub error_banner: Option<ListErrorBannerProps>,

    /// Custom table markup (thead/tbody/etc.) to render inside the card
    pub children: Element,

    /// Pagination handlers
    pub on_prev: EventHandler<()>,
    pub on_next: EventHandler<()>,

    /// Toggle pagination / overlay if needed
    #[props(default = true)]
    pub show_pagination: bool,
    #[props(default = true)]
    pub show_loading_overlay: bool,
}

#[component]
pub fn DataTableScreen<T: Clone + PartialEq + 'static>(props: DataTableScreenProps<T>) -> Element {
    let list = props.frame.clone();
    let list_loading = list.is_loading();
    let list_failed = list.is_failed();

    let has_data = list
        .data
        .as_ref()
        .map(|p| !p.data.is_empty())
        .unwrap_or(false);

    rsx! {
        div { class: "min-h-screen bg-zinc-50 dark:bg-zinc-950 text-zinc-900 dark:text-zinc-50",
            // Page header (optional)
            if let Some(header_props) = props.header.clone() {
                PageHeader { ..header_props }
            }

            // Error banner (only when failed)
            if list_failed {
                div { class: "container mx-auto px-4 pt-4",
                    match props.error_banner.clone() {
                        Some(banner_props) => rsx!{ ListErrorBanner { ..banner_props } },
                        None => rsx!{ ListErrorBanner { message: "Failed to load data. Please try again.".to_string() } },
                    }
                }
            }

            // Main content
            div { class: "container mx-auto px-4 py-8 md:py-12",
                Card { class: "border-muted shadow-none mt-4",
                    div { class: "relative",
                        // Caller-provided table markup (thead/tbody/etc.)
                        { props.children }

                        // Pagination
                        if props.show_pagination {
                            Pagination::<T> {
                                page: list.data.clone(),
                                disabled: list_loading,
                                on_prev: move |_| { props.on_prev.call(()); },
                                on_next: move |_| { props.on_next.call(()); },
                            }
                        }

                        // Loading overlay when we have data
                        if props.show_loading_overlay {
                            LoadingOverlay { visible: list_loading && has_data }
                        }
                    }
                }
            }
        }
    }
}
