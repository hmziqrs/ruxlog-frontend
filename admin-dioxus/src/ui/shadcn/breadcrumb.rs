use dioxus::prelude::*;
use hmziq_dioxus_free_icons::{Icon, icons::ld_icons::{LdChevronRight, LdEllipsis}};

#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbProps {
    #[props(default)]
    class: Option<String>,
    children: Element,
}

#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbListProps {
    #[props(default)]
    class: Option<String>,
    children: Element,
}

#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbItemProps {
    #[props(default)]
    class: Option<String>,
    children: Element,
}

#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbLinkProps {
    #[props(default)]
    class: Option<String>,
    href: String,
    children: Element,
}

#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbPageProps {
    #[props(default)]
    class: Option<String>,
    children: Element,
}

#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbSeparatorProps {
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    children: Option<Element>,
}

#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbEllipsisProps {
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    rsx! {
        nav {
            "aria-label": "breadcrumb",
            "data-slot": "breadcrumb",
            class: props.class,
            {props.children}
        }
    }
}

#[component]
pub fn BreadcrumbList(props: BreadcrumbListProps) -> Element {
    let mut class = vec!["text-muted-foreground flex flex-wrap items-center gap-1.5 text-sm break-words sm:gap-2.5".to_string()];
    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        ol {
            "data-slot": "breadcrumb-list",
            class: class.join(" "),
            {props.children}
        }
    }
}

#[component]
pub fn BreadcrumbItem(props: BreadcrumbItemProps) -> Element {
    let mut class = vec!["inline-flex items-center gap-1.5".to_string()];
    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        li {
            "data-slot": "breadcrumb-item",
            class: class.join(" "),
            {props.children}
        }
    }
}

#[component]
pub fn BreadcrumbLink(props: BreadcrumbLinkProps) -> Element {
    let mut class = vec!["hover:text-foreground transition-colors".to_string()];
    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        a {
            "data-slot": "breadcrumb-link",
            class: class.join(" "),
            href: props.href,
            {props.children}
        }
    }
}

#[component]
pub fn BreadcrumbPage(props: BreadcrumbPageProps) -> Element {
    let mut class = vec!["text-foreground font-normal".to_string()];
    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        span {
            "data-slot": "breadcrumb-page",
            role: "link",
            "aria-disabled": "true",
            "aria-current": "page",
            class: class.join(" "),
            {props.children}
        }
    }
}

#[component]
pub fn BreadcrumbSeparator(props: BreadcrumbSeparatorProps) -> Element {
    let mut class = vec!["[&>svg]:size-3.5".to_string()];
    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        li {
            "data-slot": "breadcrumb-separator",
            role: "presentation",
            "aria-hidden": "true",
            class: class.join(" "),
            {props.children.unwrap_or_else(|| rsx! {
                div { class: "h-3.5 w-3.5",
                    Icon { icon: LdChevronRight {} }
                }
            })}
        }
    }
}

#[component]
pub fn BreadcrumbEllipsis(props: BreadcrumbEllipsisProps) -> Element {
    let mut class = vec!["flex size-9 items-center justify-center".to_string()];
    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        span {
            "data-slot": "breadcrumb-ellipsis",
            role: "presentation",
            "aria-hidden": "true",
            class: class.join(" "),
            div { class: "size-4",
                Icon { icon: LdEllipsis {} }
            }
            span { class: "sr-only", "More" }
        }
    }
}