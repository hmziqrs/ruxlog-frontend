use dioxus::prelude::*;

use crate::store::analytics::AnalyticsInterval;

/// Interval selector for analytics charts.
///
/// Renders a compact dropdown/toggle for selecting Hour/Day/Week/Month intervals.
#[derive(Props, Clone, PartialEq)]
pub struct IntervalSelectorProps {
    /// Currently selected interval
    pub current: AnalyticsInterval,
    /// Callback when interval changes
    pub on_change: EventHandler<AnalyticsInterval>,
    /// Optional label (defaults to "Interval:")
    #[props(default = "Interval:".to_string())]
    pub label: String,
}

#[component]
pub fn IntervalSelector(props: IntervalSelectorProps) -> Element {
    let intervals = vec![
        (AnalyticsInterval::Hour, "Hour"),
        (AnalyticsInterval::Day, "Day"),
        (AnalyticsInterval::Week, "Week"),
        (AnalyticsInterval::Month, "Month"),
    ];

    rsx! {
        div {
            class: "flex items-center gap-1.5",
            if !props.label.is_empty() {
                label {
                    class: "text-[9px] font-medium text-zinc-500 dark:text-zinc-400",
                    "{props.label}"
                }
            }
            div {
                class: "inline-flex rounded-md border border-zinc-200 dark:border-zinc-800 \
                       bg-zinc-50/50 dark:bg-zinc-950/50 p-0.5",
                for (interval, label) in intervals {
                    {
                        let is_active = props.current == interval;
                        let interval_clone = interval.clone();

                        rsx! {
                            button {
                                r#type: "button",
                                onclick: move |_| props.on_change.call(interval_clone),
                                class: if is_active {
                                    "px-2 py-0.5 text-[9px] font-medium rounded \
                                     bg-white dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100 \
                                     shadow-sm border border-zinc-200 dark:border-zinc-700 \
                                     transition-all"
                                } else {
                                    "px-2 py-0.5 text-[9px] font-medium rounded \
                                     text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 \
                                     dark:hover:text-zinc-100 hover:bg-white/50 \
                                     dark:hover:bg-zinc-900/50 transition-colors"
                                },
                                "{label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
