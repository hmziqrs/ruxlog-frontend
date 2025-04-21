use dioxus::prelude::*;

use crate::ui::{custom::cmdk::*, shadcn::*};

#[component]
pub fn CategoryListScreen() -> Element {
    let tabs = vec![
        TabItem::new("First Tab".to_string(), false),
        TabItem::new("Second Tab".to_string(), false),
        TabItem::new("Disabled Tab".to_string(), true),
    ];

    let groups = vec![
        "Settings".to_string(),
        "Suggestions".to_string(),
        "Other".to_string(),
    ];

    let data = vec![
        CommandListItem::new("settings".to_string(), "Profile".to_string(), false),
        CommandListItem::new("settings".to_string(), "Billing".to_string(), false),
        CommandListItem::new("settings".to_string(), "Settings".to_string(), false),
        CommandListItem::new("settings".to_string(), "Display".to_string(), false),
        //
        CommandListItem::new("suggestions".to_string(), "Calendar".to_string(), false),
        CommandListItem::new("suggestions".to_string(), "Emoji".to_string(), false),
        CommandListItem::new("suggestions".to_string(), "Calculator".to_string(), true),
        //
        CommandListItem::new("other".to_string(), "Camera".to_string(), true),
        CommandListItem::new("other".to_string(), "Video".to_string(), true),
    ];

    let tab_state: Signal<TabsState> = use_signal(|| TabsState::new(tabs, 0));

    rsx! {
        div { "Category List (placeholder)" }
        Popover {
            PopoverTrigger { "Click me" }
            PopoverContent {
                h3 { class: "font-medium mb-2", "Popover Title" }
                p { "This is some popover content with useful information." }
                div { class: "mt-4 flex justify-end",
                    PopoverClose { class: "px-3 py-1.5 bg-zinc-100 rounded text-sm hover:bg-zinc-200",
                        "Close"
                    }
                }
            }
        }
        Breadcrumb {
            BreadcrumbList {
                BreadcrumbItem {
                    BreadcrumbLink { href: "/", "Home" }
                }
                BreadcrumbSeparator {}
                BreadcrumbItem { BreadcrumbEllipsis {} }
                BreadcrumbSeparator {}
                BreadcrumbItem {
                    BreadcrumbLink { href: "/blogs/tech", "Tech" }
                }
                BreadcrumbSeparator {}
                BreadcrumbItem {
                    BreadcrumbPage { "Current Article" }
                }
            }
        }
        Accordion {
            AccordionItem { value: 0,
                AccordionTrigger { "Accordion Item 1" }
                AccordionContent { "Content for accordion item 1" }
            }
            AccordionItem { value: 1,
                AccordionTrigger { "Accordion Item 2" }
                AccordionContent { "Content for accordion item 2" }
            }
        }
        Tabs { state: tab_state }
        Cmdk { groups, data, reset_on_select: true }
    }
}
