use dioxus::prelude::*;
use crate::ui::shadcn::{
    Alert, AlertDescription, AlertTitle,
    Badge, BadgeVariant,
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
    Button, ButtonVariant,
    Card, CardContent, CardDescription, CardHeader, CardTitle,
    Checkbox,
};

#[component]
pub fn TagsAddScreen() -> Element {
    // Static placeholder values (no interactivity/state)
    let default_color = "#3b82f6";
    let url_preview = "/tags/your-tag-slug";

    rsx! {
        // Page wrapper
        div { class: "min-h-screen bg-zinc-50 dark:bg-zinc-950 text-zinc-900 dark:text-zinc-50",
            // Top region with breadcrumb and header
            div { class: "border-b border-zinc-200 dark:border-zinc-800 bg-gradient-to-b from-zinc-50/60 to-transparent dark:from-zinc-950/40",
                div { class: "container mx-auto px-4 py-6 md:py-8",
                    // Breadcrumb
                    Breadcrumb {
                        BreadcrumbList {
                            BreadcrumbItem {
                                BreadcrumbLink { href: "/dashboard".to_string(), "Dashboard" }
                            }
                            BreadcrumbSeparator {}
                            BreadcrumbItem {
                                BreadcrumbLink { href: "/dashboard/tags".to_string(), "Tags" }
                            }
                            BreadcrumbSeparator {}
                            BreadcrumbItem { BreadcrumbPage { "New" } }
                        }
                    }

                    // Header row
                    div { class: "mt-4 flex flex-col items-start justify-between gap-4 md:flex-row md:items-center",
                        div { class: "space-y-1",
                            h1 { class: "text-3xl md:text-4xl font-bold tracking-tight", "Create Tag" }
                            p { class: "text-sm md:text-base text-zinc-600 dark:text-zinc-400",
                                "Define how your tag looks and behaves. Keep names concise and meaningful."
                            }
                        }
                        div { class: "flex items-center gap-2",
                            // Save button (static)
                            Button { r#type: "submit".to_string(), class: "gap-2".to_string(), "Save Tag" }
                        }
                    }
                }
            }

            // Content
            div { class: "container mx-auto px-4 py-8",
                form { id: "new-tag-form", class: "grid grid-cols-1 gap-6 lg:grid-cols-3",
                    // Main column
                    div { class: "lg:col-span-2 space-y-6",
                        Card { class: Some("dark:bg-zinc-900/60 border-zinc-200 dark:border-zinc-800".to_string()),
                            CardHeader { class: Some("pb-4".to_string()),
                                CardTitle { "Tag details" }
                                CardDescription { "Basic information and metadata for your tag." }
                            }
                            CardContent { class: Some("space-y-6".to_string()),
                                // Name
                                div { class: "space-y-2",
                                    label { r#for: "name", "Name" }
                                    input {
                                        id: "name",
                                        name: "name",
                                        r#type: "text",
                                        placeholder: "e.g. Product Updates",
                                        class: "bg-white dark:bg-zinc-900 border-zinc-200 dark:border-zinc-800 w-full h-10 rounded-md border px-3 text-sm outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
                                    }
                                    p { class: "text-xs text-zinc-500 dark:text-zinc-400",
                                        "Keep it short and recognizable. You can change this later."
                                    }
                                }

                                // Separator
                                div { class: "bg-zinc-200 dark:bg-zinc-800 h-px w-full" }

                                // Slug
                                div { class: "space-y-2",
                                    div { class: "flex items-center justify-between",
                                        label { r#for: "slug", "Slug" }
                                        Button {
                                            r#type: "button".to_string(),
                                            variant: ButtonVariant::Outline,
                                            class: "h-8 px-3 text-xs border-zinc-200 dark:border-zinc-800".to_string(),
                                            "Generate from name"
                                        }
                                    }
                                    input {
                                        id: "slug",
                                        name: "slug",
                                        r#type: "text",
                                        placeholder: "product-updates",
                                        class: "bg-white dark:bg-zinc-900 border-zinc-200 dark:border-zinc-800 w-full h-10 rounded-md border px-3 text-sm outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
                                    }
                                    div { class: "flex items-center gap-2",
                                        span { class: "text-xs text-zinc-500 dark:text-zinc-400", "URL preview:" }
                                        code { class: "rounded bg-zinc-100 px-1.5 py-0.5 text-xs dark:bg-zinc-800", {url_preview} }
                                    }
                                }

                                // Separator
                                div { class: "bg-zinc-200 dark:bg-zinc-800 h-px w-full" }

                                // Description
                                div { class: "space-y-2",
                                    label { r#for: "description", "Description" }
                                    textarea {
                                        id: "description",
                                        name: "description",
                                        placeholder: "Briefly describe what posts belong in this tag.",
                                        class: "h-28 resize-none bg-white dark:bg-zinc-900 border-zinc-200 dark:border-zinc-800 w-full rounded-md border px-3 py-2 text-sm outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
                                    }
                                    p { class: "text-xs text-zinc-500 dark:text-zinc-400",
                                        "Optional. Shown in certain listings and SEO contexts."
                                    }
                                }
                            }
                        }

                        Alert { class: Some("border-zinc-200 dark:border-zinc-800 bg-white/60 dark:bg-zinc-900/60".to_string()),
                            AlertTitle { "Design tip" }
                            AlertDescription {
                                "Use contrasting colors to ensure the tag remains legible in both light and dark themes."
                            }
                        }
                    }

                    // Sidebar column
                    div { class: "space-y-6 lg:sticky lg:top-24 h-fit",
                        // Appearance
                        Card { class: Some("dark:bg-zinc-900/60 border-zinc-200 dark:border-zinc-800".to_string()),
                            CardHeader { class: Some("pb-4".to_string()),
                                CardTitle { "Appearance" }
                                CardDescription { "Choose a color and preview the tag." }
                            }
                            CardContent { class: Some("space-y-4".to_string()),
                                div { class: "space-y-2",
                                    label { "Tag color" }
                                    input { r#type: "color", value: default_color, class: "h-9 w-14 rounded-md border border-zinc-200 dark:border-zinc-800 bg-transparent p-1" }
                                }
                                div { class: "space-y-2",
                                    label { "Preview" }
                                    div { class: "flex items-center gap-3",
                                        span { class: "inline-flex items-center rounded-md px-2.5 py-1.5 text-sm font-medium shadow-sm ring-1 ring-inset",
                                            style: format!("background-color: {default_color}; color: {}; border-color: rgba(0,0,0,0.06);", "#ffffff"),
                                            span { class: "mr-2 inline-block size-2.5 rounded-full bg-white/40" }
                                            "Tag preview"
                                        }
                                        Badge { variant: BadgeVariant::Outline, class: "text-xs border-zinc-200 dark:border-zinc-800".to_string(), "#3b82f6" }
                                    }
                                    p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Text color auto-adjusts for readability." }
                                }
                            }
                        }

                        // Visibility
                        Card { class: Some("dark:bg-zinc-900/60 border-zinc-200 dark:border-zinc-800".to_string()),
                            CardHeader { class: Some("pb-4".to_string()),
                                CardTitle { "Visibility" }
                                CardDescription { "Control whether this tag is available publicly." }
                            }
                            CardContent { class: Some("space-y-4".to_string()),
                                div { class: "flex items-center justify-between",
                                    div { class: "space-y-0.5",
                                        label { r#for: "active", "Active" }
                                        p { class: "text-xs text-zinc-500 dark:text-zinc-400",
                                            "This tag will be visible across your site."
                                        }
                                    }
                                    // Static checkbox (acts as switch stand-in)
                                    Checkbox { class: Some("size-6 rounded-full".to_string()), checked: true }
                                }
                            }
                        }

                        // Footer actions (static)
                        div { class: "flex gap-2 pt-2",
                            Button { r#type: "button".to_string(), variant: ButtonVariant::Outline, class: "w-full border-zinc-200 dark:border-zinc-800".to_string(), "Cancel" }
                            Button { r#type: "submit".to_string(), class: "w-full gap-2".to_string(), "Save Tag" }
                        }
                    }
                }
            }
        }
    }
}
