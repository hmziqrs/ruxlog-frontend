use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ChildProps {
    pub children: Element,
}



#[component]
pub fn Card(props: ChildProps) -> Element {
    rsx! {
        div { class: "rounded-xl border bg-white dark:bg-zinc-900 border-zinc-200 dark:border-zinc-800 shadow",
            {props.children}
        }
    }
}

#[component]
pub fn CardHeader(props: ChildProps) -> Element {
    rsx! {
        div { class: "p-4 pb-0", {props.children} }
    }
}

#[component]
pub fn CardContent(props: ChildProps) -> Element {
    rsx! {
        div { class: "p-4 pt-2", {props.children} }
    }
}

#[component]
pub fn CardFooter(props: ChildProps) -> Element {
    rsx! {
        div { class: "p-4 pt-0 flex items-center justify-between", {props.children} }
    }
}

struct DropdownContext(bool);

#[component]
pub fn DropdownMenu(props: ChildProps) -> Element {
    let mut open = use_context_provider(|| Signal::new(DropdownContext(false)));

    rsx! {
        div { class: "relative", {props.children} }
    }
}


#[component]
pub fn DropdownMenuTrigger(props: ChildProps) -> Element {
    let mut open = use_context::<Signal<DropdownContext>>();
    rsx! {
        button {
            class: "h-8 w-8",
            onclick: move |_| {
                let is_open = open.peek().0;
                open.set(DropdownContext(!is_open));
            },
            {props.children}
        }
    }
}

#[component]
pub fn DropdownMenuContent(props: ChildProps) -> Element {
    let signal_open = use_context::<Signal<DropdownContext>>();
    let open = signal_open.read();
    rsx! {
        div {
            class: format!(
                "absolute right-0 mt-2 w-40 rounded-md bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 shadow-lg z-50 {}",
                if open.0 { "block" } else { "hidden visible" },
            ),
            {props.children}
        }
    }
}

#[component]
pub fn DropdownMenuItem(props: ChildProps) -> Element {
    rsx! {
        div { class: "px-4 py-2 hover:bg-zinc-100 dark:hover:bg-zinc-800 cursor-pointer",
            {props.children}
        }
    }
}



// Badge
#[component]
pub fn Badge(props: ChildProps) -> Element {
    rsx! {
        span { class: "inline-block rounded px-2 py-0.5 text-xs font-medium bg-zinc-100 dark:bg-zinc-800 text-zinc-800 dark:text-zinc-200",
            {props.children}
        }
    }
}

// Avatar
#[component]
pub fn Avatar(props: ChildProps) -> Element {
    rsx! {
        span { class: "inline-flex items-center justify-center rounded-full bg-zinc-200 dark:bg-zinc-800 h-6 w-6 overflow-hidden",
            {props.children}
        }
    }
}

#[component]
pub fn AvatarImage(src: String, alt: String) -> Element {
    rsx! {
        img { src, alt, class: "h-full w-full object-cover" }
    }
}

#[component]
pub fn AvatarFallback(props: ChildProps) -> Element {
    rsx! {
        span { class: "text-xs bg-zinc-200 dark:bg-zinc-800 text-zinc-800 dark:text-zinc-200 flex items-center justify-center h-full w-full",
            {props.children}
        }
    }
}
