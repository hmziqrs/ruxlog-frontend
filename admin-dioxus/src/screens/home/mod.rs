use dioxus::prelude::*;

use crate::components::analytics::{
    dashboard_summary_cards::DashboardSummaryCards, page_views_chart::PageViewsChart,
    publishing_trends_chart::PublishingTrendsChart,
    registration_trend_chart::RegistrationTrendChart,
    verification_rates_chart::VerificationRatesChart,
};
use crate::components::PageHeader;
use crate::hooks::use_state_frame_toast::{use_state_frame_toast, StateFrameToastConfig};
use crate::store::analytics::{
    use_analytics, AnalyticsEnvelope, AnalyticsInterval, DashboardSummaryFilters,
    DashboardSummaryRequest, PageViewsFilters, PageViewsRequest, PublishingTrendsFilters,
    PublishingTrendsRequest, RegistrationTrendsFilters, RegistrationTrendsRequest,
    VerificationRatesFilters, VerificationRatesRequest,
};

#[component]
pub fn HomeScreen() -> Element {
    let analytics = use_analytics();

    // Wire toast helpers for key frames so dashboard surfaces API issues.
    let _summary_toast = use_state_frame_toast(
        &analytics.dashboard_summary,
        StateFrameToastConfig::default(),
    );
    let _views_toast =
        use_state_frame_toast(&analytics.page_views, StateFrameToastConfig::default());
    let _publishing_toast = use_state_frame_toast(
        &analytics.publishing_trends,
        StateFrameToastConfig::default(),
    );
    let _registration_toast = use_state_frame_toast(
        &analytics.registration_trends,
        StateFrameToastConfig::default(),
    );
    let _verification_toast = use_state_frame_toast(
        &analytics.verification_rates,
        StateFrameToastConfig::default(),
    );

    // Kick off initial dashboard analytics fetches on mount.
    use_future(move || {
        async move {
            // Dashboard summary: last 7 days
            let summary_req = DashboardSummaryRequest {
                envelope: Some(AnalyticsEnvelope {
                    date_from: None,
                    date_to: None,
                    page: None,
                    per_page: None,
                    sort_by: None,
                    sort_order: None,
                }),
                filters: DashboardSummaryFilters {
                    period: "7d".to_string(),
                },
            };
            analytics.fetch_dashboard_summary(summary_req).await;

            // Page views: default to last 7 days, grouped daily, all views
            let page_views_req = PageViewsRequest {
                envelope: AnalyticsEnvelope {
                    date_from: None,
                    date_to: None,
                    page: None,
                    per_page: None,
                    sort_by: None,
                    sort_order: None,
                },
                filters: PageViewsFilters {
                    group_by: AnalyticsInterval::Day,
                    post_id: None,
                    author_id: None,
                    only_unique: false,
                },
            };
            analytics.fetch_page_views(page_views_req).await;

            // Publishing trends: last 7 days, grouped daily
            let publishing_req = PublishingTrendsRequest {
                envelope: AnalyticsEnvelope {
                    date_from: None,
                    date_to: None,
                    page: None,
                    per_page: None,
                    sort_by: None,
                    sort_order: None,
                },
                filters: PublishingTrendsFilters {
                    group_by: AnalyticsInterval::Day,
                    status: None,
                },
            };
            analytics.fetch_publishing_trends(publishing_req).await;

            // Registration trends: last 7 days, grouped daily
            let registration_req = RegistrationTrendsRequest {
                envelope: AnalyticsEnvelope {
                    date_from: None,
                    date_to: None,
                    page: None,
                    per_page: None,
                    sort_by: None,
                    sort_order: None,
                },
                filters: RegistrationTrendsFilters {
                    group_by: AnalyticsInterval::Day,
                },
            };
            analytics.fetch_registration_trends(registration_req).await;

            // Verification rates: last 7 days, grouped daily
            let verification_req = VerificationRatesRequest {
                envelope: AnalyticsEnvelope {
                    date_from: None,
                    date_to: None,
                    page: None,
                    per_page: None,
                    sort_by: None,
                    sort_order: None,
                },
                filters: VerificationRatesFilters {
                    group_by: AnalyticsInterval::Day,
                },
            };
            analytics.fetch_verification_rates(verification_req).await;
        }
    });

    // Read frames once for rendering; the inner components handle states.
    let summary_frame = analytics.dashboard_summary.read();
    let page_views_frame = analytics.page_views.read();
    let publishing_frame = analytics.publishing_trends.read();
    let registration_frame = analytics.registration_trends.read();
    let verification_frame = analytics.verification_rates.read();

    rsx! {
        div { class: "min-h-screen bg-transparent text-foreground",
            // Page header
            PageHeader {
                title: "Dashboard".to_string(),
                description: "Overview of your blog performance, content, and activity.".to_string(),
            }

            div { class: "container mx-auto px-4 pb-10 space-y-6",

                // Summary KPI cards row
                DashboardSummaryCards {
                    frame: summary_frame.clone(),
                    title: "Analytics overview",
                    description: "Key metrics for users, content, engagement, and media in the last 7 days.",
                }

                // Primary charts row: traffic and publishing
                div { class: "grid grid-cols-1 lg:grid-cols-2 gap-4",
                    PageViewsChart {
                        frame: page_views_frame.clone(),
                        title: "Traffic & views (last 7 days)".to_string(),
                        height: "h-72".to_string(),
                        compact: false,
                    }

                    PublishingTrendsChart {
                        frame: publishing_frame.clone(),
                        title: "Publishing activity".to_string(),
                        height_class: "h-72".to_string(),
                        description: Some("Posts by status across recent days.".to_string()),
                    }
                }

                // Secondary charts row: registrations & verifications
                div { class: "grid grid-cols-1 lg:grid-cols-2 gap-4",
                    RegistrationTrendChart {
                        frame: registration_frame.clone(),
                        title: "New user registrations (last 7 days)".to_string(),
                        height: "h-64".to_string(),
                    }

                    VerificationRatesChart {
                        frame: verification_frame.clone(),
                        title: "Verification funnel (last 7 days)".to_string(),
                        height: "260px".to_string(),
                        show_success_rate: true,
                    }
                }
            }
        }
    }
}
