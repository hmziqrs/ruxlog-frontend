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

    let invoices = vec![
    ("INV001", "Pay App, Inc.", "Credit Card", "$250.00"),
    ("INV002", "Cloud Services Co", "Visa", "$150.00"),
    ("INV003", "Design System Inc", "Mastercard", "$350.00"),
    ("INV004", "Web Solutions LLC", "PayPal", "$450.00"),
    ("INV005", "Tech Innovations", "Visa", "$550.00"),
];

    rsx! {
        div { "Category List (placeholder)" }
        div { class: "space-y-3",
            Skeleton { class: Some("h-[125px] w-full rounded-xl".to_string()) }
            div { class: "space-y-2",
                Skeleton { class: Some("h-4 w-[250px]".to_string()) }
                Skeleton { class: Some("h-4 w-[200px]".to_string()) }
            }
        }
        Table {
            TableCaption { "A list of your recent invoices." }
            TableHeader {
                TableRow {
                    TableHead { "Invoice" }
                    TableHead { "Status" }
                    TableHead { "Method" }
                    TableHead { class: Some("text-right".to_string()), "Amount" }
                }
            }
            TableBody {
                {
                    invoices
                        .iter()
                        .enumerate()
                        .map(|(index, invoice)| {
                            let (id, status, method, amount) = invoice;
                            rsx! {
                                TableRow { selected: index % 3 == 0,
                                    TableCell { class: Some("font-medium".to_string()), "{id}" }
                                    TableCell { "{status}" }
                                    TableCell { "{method}" }
                                    TableCell { class: Some("text-right".to_string()), "{amount}" }
                                }
                            }
                        })
                }
            }
        }
        Select {
            onchange: move |value| {
                println!("Selected: {}", value);
            },
            SelectTrigger { class: Some("w-[200px]".to_string()),
                SelectValue { placeholder: String::from("Select a framework") }
            }
            SelectContent {
                SelectGroup {
                    SelectLabel { "Frameworks" }
                    SelectItem { value: String::from("next.js"), "Next.js" }
                    SelectItem { value: String::from("sveltekit"), "SvelteKit" }
                    SelectItem { value: String::from("nuxt"), "Nuxt.js" }
                    SelectItem { value: String::from("remix"), "Remix" }
                    SelectItem { value: String::from("astro"), "Astro" }
                }
            }
        }
        Progress { value: 50 }
        Popover {
            PopoverTrigger { "Click me" }
            PopoverContent {
                Cmdk { groups, data, reset_on_select: true }
            }
        }
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
    }
}
