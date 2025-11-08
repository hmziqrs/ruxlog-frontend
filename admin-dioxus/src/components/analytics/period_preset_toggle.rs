use dioxus::prelude::*;

/// Period preset toggle for quick date range selection.
///
/// Renders buttons for 7d, 30d, 90d presets with active state.
#[derive(Props, Clone, PartialEq)]
pub struct PeriodPresetToggleProps {
    /// Currently active preset ("7d", "30d", "90d", or None for custom)
    pub active_period: Option<String>,
    /// Callback when preset is clicked
    pub on_change: EventHandler<String>,
}

#[component]
pub fn PeriodPresetToggle(props: PeriodPresetToggleProps) -> Element {
    let presets = vec![
        ("7d", "7 days"),
        ("30d", "30 days"),
        ("90d", "90 days"),
    ];

    rsx! {
        div {
            class: "flex items-center gap-1.5",
            label {
                class: "text-xs font-medium text-zinc-600 dark:text-zinc-400 mr-1",
                "Period:"
            }
            div {
                class: "inline-flex rounded-lg border border-zinc-200 dark:border-zinc-800 \
                       bg-zinc-50/50 dark:bg-zinc-950/50 p-0.5",
                for (value, label) in presets {
                    {
                        let is_active = props.active_period.as_ref().map(|p| p == value).unwrap_or(false);
                        let value_clone = value.to_string();

                        rsx! {
                            button {
                                r#type: "button",
                                onclick: move |_| props.on_change.call(value_clone.clone()),
                                class: if is_active {
                                    "px-3 py-1.5 text-xs font-medium rounded-md \
                                     bg-white dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100 \
                                     shadow-sm border border-zinc-200 dark:border-zinc-700 \
                                     transition-all"
                                } else {
                                    "px-3 py-1.5 text-xs font-medium rounded-md \
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
