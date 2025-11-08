use dioxus::prelude::*;

use crate::store::{AnalyticsEnvelopeResponse, MediaUploadPoint, StateFrame, StateFrameStatus};

/// Props for `MediaUploadTrendsChart`.
///
/// This component is intentionally thin:
/// - It consumes a `StateFrame` for media upload trends data.
/// - It renders a styled card shell with loading/error/empty states handled.
/// - When `dioxus-charts` is available, hook it up in the marked section below.
#[derive(Props, PartialEq, Clone)]
pub struct MediaUploadTrendsChartProps {
    /// Frame containing media upload trend points.
    pub frame: StateFrame<
        AnalyticsEnvelopeResponse<Vec<MediaUploadPoint>>,
        crate::store::MediaUploadRequest,
    >,
    /// Optional title for the chart card.
    #[props(default = "Media Upload Trends".to_string())]
    pub title: String,
    /// Tailwind height classes for the chart area.
    #[props(default = "h-64".to_string())]
    pub height_class: String,
}

/// Media upload trends visualization:
/// - Bars: upload_count
/// - Line: avg_size_mb (later: mapped via secondary axis when supported)
///
/// NOTE: This is scaffolded without `dioxus-charts` wiring. Once the dependency
/// is added, replace the placeholder graph area with real chart primitives,
/// using this component as a stable shell.
#[component]
pub fn MediaUploadTrendsChart(props: MediaUploadTrendsChartProps) -> Element {
    let MediaUploadTrendsChartProps {
        frame,
        title,
        height_class,
    } = props;

    let status = frame.status();
    let error = frame.error();
    let data = frame.data().map(|env| env.data.clone()).unwrap_or_default();

    let is_loading = matches!(status, StateFrameStatus::Init | StateFrameStatus::Loading);
    let has_error = error.is_some();
    let is_empty = !is_loading && !has_error && data.is_empty();

    // Derive basic stats for quick context badges.
    let (total_uploads, last_avg_size_mb) = if data.is_empty() {
        (0_i64, None)
    } else {
        let total: i64 = data.iter().map(|p| p.upload_count).sum();
        let last_avg = data.last().map(|p| p.avg_size_mb);
        (total, last_avg)
    };

    rsx! {
        div {
            class: "rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 \
                    bg-zinc-100/40 dark:bg-zinc-950/40 shadow-sm backdrop-blur-sm \
                    flex flex-col gap-3 p-4",

            // Header
            div {
                class: "flex items-center justify-between gap-3",
                div {
                    class: "flex flex-col",
                    h3 {
                        class: "text-sm font-semibold text-zinc-900 dark:text-zinc-50",
                        "{title}"
                    }
                    p {
                        class: "text-[11px] text-zinc-500",
                        "Track how many media files are uploaded over time and how their average size evolves."
                    }
                }

                // Quick stats
                div {
                    class: "flex flex-col items-end gap-0.5 text-[10px]",
                    span {
                        class: "px-2 py-0.5 rounded-full bg-sky-100/80 text-sky-700 \
                                dark:bg-sky-500/10 dark:text-sky-300 border border-sky-100/60 \
                                dark:border-sky-500/20",
                        "Total uploads: {total_uploads}"
                    }
                    if let Some(avg) = last_avg_size_mb {
                        span {
                            class: "px-2 py-0.5 rounded-full bg-emerald-100/80 text-emerald-700 \
                                    dark:bg-emerald-500/10 dark:text-emerald-300 border border-emerald-100/60 \
                                    dark:border-emerald-500/20",
                            "Last bucket avg size: {avg:.1} MB"
                        }
                    }
                }
            }

            // Content area
            if is_loading {
                // Loading skeleton
                div {
                    class: "animate-pulse flex-1 flex flex-col gap-3 mt-1",
                    div {
                        class: "h-4 w-32 bg-zinc-200/80 dark:bg-zinc-800/80 rounded-full"
                    }
                    div {
                        class: "h-3 w-24 bg-zinc-200/80 dark:bg-zinc-800/80 rounded-full"
                    }
                    div {
                        class: "{height_class} mt-1 rounded-xl bg-zinc-200/60 dark:bg-zinc-900/80"
                    }
                }
            } else if has_error {
                // Error state
                let err = error
                    .as_ref()
                    .map(|e| e.to_string())
                    .unwrap_or_else(|| "Unable to load media upload trends.".to_string());

                div {
                    class: "flex flex-col gap-2 mt-1",
                    div {
                        class: "text-[11px] px-2 py-1.5 rounded-lg bg-rose-100/80 text-rose-700 \
                                dark:bg-rose-500/10 dark:text-rose-300 border border-rose-200/80 \
                                dark:border-rose-500/30",
                        span { class: "font-semibold mr-1", "Error:" }
                        span { "{err}" }
                    }
                    div {
                        class: "{height_class} rounded-xl border border-dashed border-rose-200/70 \
                                dark:border-rose-500/20 flex items-center justify-center",
                        span {
                            class: "text-[10px] text-rose-500/90 dark:text-rose-300/90",
                            "Chart unavailable due to an error."
                        }
                    }
                }
            } else if is_empty {
                // Empty state
                div {
                    class: "flex flex-col gap-2 mt-1",
                    div {
                        class: "text-[11px] px-2 py-1.5 rounded-lg bg-zinc-100/80 text-zinc-600 \
                                dark:bg-zinc-900/80 dark:text-zinc-400 border border-dashed \
                                border-zinc-200/80 dark:border-zinc-800/80",
                        "No media upload data available for the selected period."
                    }
                    div {
                        class: "{height_class} rounded-xl flex items-center justify-center \
                                bg-gradient-to-br from-zinc-100/60 via-zinc-100/20 to-zinc-50/0 \
                                dark:from-zinc-900/70 dark:via-zinc-900/20 dark:to-zinc-950/0",
                        span {
                            class: "text-[10px] text-zinc-400",
                            "Uploads activity will appear here once there is data."
                        }
                    }
                }
            } else {
                // Data state: placeholder chart scaffold.
                //
                // TODO (charts): Integrate `dioxus-charts` here. Suggested mapping:
                // - X axis: point.bucket (chronological)
                // - Primary Y axis: upload_count (bars)
                // - Secondary Y axis: avg_size_mb (line)
                //
                // For now, render a minimal preview-like visualization so layout is stable.
                div {
                    class: "flex flex-col gap-2 mt-1",
                    // Legend
                    div {
                        class: "flex items-center gap-3 text-[9px] text-zinc-500",
                        div {
                            class: "flex items-center gap-1",
                            span { class: "w-3 h-1.5 rounded-full bg-sky-500" }
                            span { "Uploads" }
                        }
                        div {
                            class: "flex items-center gap-1",
                            span { class: "w-3 h-[2px] rounded-full bg-emerald-500" }
                            span { "Avg size (MB)" }
                        }
                    }

                    // Simple SVG-ish bars/line style scaffold using divs
                    div {
                        class: format!(
                            "relative {height_class} mt-1 rounded-xl border border-zinc-200/70 \
                             dark:border-zinc-800/80 bg-zinc-50/40 dark:bg-zinc-950/40 \
                             overflow-hidden px-3 pt-3 pb-4 flex items-end gap-1.5"
                        ),

                        // Compute max for normalization (avoid division by zero).
                        {
                            let max_uploads = data
                                .iter()
                                .map(|p| p.upload_count.max(0))
                                .max()
                                .unwrap_or(1) as f64;
                            let max_avg = data
                                .iter()
                                .map(|p| p.avg_size_mb.max(0.0))
                                .fold(0.0_f64, f64::max)
                                .max(1.0);

                            data.iter().enumerate().map(|(idx, point)| {
                                let uploads = point.upload_count.max(0) as f64;
                                let avg_mb = point.avg_size_mb.max(0.0);

                                // Bar height normalized to 70% of container.
                                let bar_ratio = if max_uploads > 0.0 {
                                    (uploads / max_uploads).clamp(0.05, 1.0)
                                } else {
                                    0.05
                                };

                                // Line marker height normalized to remaining 30% offset.
                                let line_ratio = if max_avg > 0.0 {
                                    (avg_mb / max_avg).clamp(0.0, 1.0)
                                } else {
                                    0.0
                                };

                                // A tiny label every few buckets to avoid clutter.
                                let show_label = idx % 3 == 0 || idx == data.len().saturating_sub(1);

                                rsx! {
                                    div {
                                        key: "{idx}",
                                        class: "flex-1 flex flex-col-reverse items-center gap-1",
                                        // X-axis bucket label
                                        if show_label {
                                            div {
                                                class: "text-[7px] text-zinc-400 truncate w-full text-center mt-1",
                                                "{point.bucket}"
                                            }
                                        } else {
                                            div { class: "mt-1" }
                                        }
                                        // Chart column (bar + optional line dot)
                                        div {
                                            class: "w-full flex flex-col justify-end items-center gap-0.5",
                                            // Uploads bar
                                            div {
                                                class: "w-2 rounded-t-full bg-sky-500/75 \
                                                        dark:bg-sky-400/80",
                                                style: format!(
                                                    "height: {}%;",
                                                    bar_ratio * 70.0
                                                ),
                                            }
                                            // Avg size marker as small dot line-aligned
                                            if avg_mb > 0.0 {
                                                div {
                                                    class: "w-[6px] h-[6px] rounded-full bg-emerald-500 \
                                                            dark:bg-emerald-400 shadow-sm",
                                                    style: format!(
                                                        "margin-bottom: {}%;",
                                                        line_ratio * 18.0
                                                    ),
                                                }
                                            }
                                        }
                                    }
                                }
                            })
                        }
                    }

                    // Footer hint
                    div {
                        class: "flex justify-between items-center mt-1 text-[8px] text-zinc-400",
                        span { "Visual scale is relative to current dataset." }
                        span { "Hook up full dioxus-charts here for production." }
                    }
                }
            }
        }
    }
}
