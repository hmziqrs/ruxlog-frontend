use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct BadgeProps {
    /// Additional classes to apply to the badge
    #[props(default)]
    pub class: Option<String>,
    /// Badge content
    pub children: Element,
    /// Badge variant
    #[props(default = BadgeVariant::Default)]
    pub variant: BadgeVariant,
}

#[derive(PartialEq, Clone)]
pub enum BadgeVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
}

impl BadgeVariant {
    fn to_class_string(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "border-transparent bg-primary text-primary-foreground [a&]:hover:bg-primary/90",
            BadgeVariant::Secondary => "border-transparent bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90",
            BadgeVariant::Destructive => "border-transparent bg-destructive text-white [a&]:hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60",
            BadgeVariant::Outline => "text-foreground [a&]:hover:bg-accent [a&]:hover:text-accent-foreground",
        }
    }
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let base_classes = "inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0 [&>svg]:size-3 gap-1 [&>svg]:pointer-events-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive transition-[color,box-shadow] overflow-hidden";
    
    let mut classes = vec![base_classes.to_string(), props.variant.to_class_string().to_string()];
    if let Some(custom_class) = props.class {
        classes.push(custom_class);
    }

    rsx! {
        div { "data-slot": "badge", class: classes.join(" "), {props.children} }
    }
}