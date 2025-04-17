use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct AvatarProps {
    /// Additional classes to apply to the avatar
    #[props(default)]
    class: Option<String>,
    children: Element,
}

#[derive(Props, PartialEq, Clone)]
pub struct AvatarImageProps {
    /// Additional classes to apply to the image
    #[props(default)]
    class: Option<String>,
    /// Image source URL
    src: String,
    /// Alt text for the image
    alt: String,
}

#[derive(Props, PartialEq, Clone)]
pub struct AvatarFallbackProps {
    /// Additional classes to apply to the fallback
    #[props(default)]
    class: Option<String>,
    children: Element,
}



#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let mut class = vec!["relative flex size-8 shrink-0 overflow-hidden rounded-full".to_string()];
    
    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        div { "data-slot": "avatar", class: class.join(" "), {props.children} }
    }
}

#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    let mut class = vec!["aspect-square size-full".to_string()];
    
    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        img {
            "data-slot": "avatar-image",
            class: class.join(" "),
            src: props.src,
            alt: props.alt,
        }
    }
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    let mut class = vec!["flex size-full items-center justify-center rounded-full bg-muted".to_string()];
    
    if let Some(custom_class) = props.class {
        class.push(custom_class);
    }

    rsx! {
        div { "data-slot": "avatar-fallback", class: class.join(" "), {props.children} }
    }
}