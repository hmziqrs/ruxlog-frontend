use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ChildProps {
    pub children: Element,
    #[props(default = String::new())]
    pub class: String,
}

struct DropdownContext(bool);

#[component]
pub fn DropdownMenu(props: ChildProps) -> Element {
    let mut open = use_context_provider(|| Signal::new(DropdownContext(false)));

    rsx! {
        div { class: "relative inline-block text-left", {props.children} }
    }
}

#[component]
pub fn DropdownMenuTrigger(props: ChildProps) -> Element {
    let mut open = use_context::<Signal<DropdownContext>>();
    rsx! {
        button {
            class: "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 w-10",
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
                "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 absolute right-0 mt-2 w-56 origin-top-right {} ",
                if open.0 { "block" } else { "hidden" },
            ),
            {props.children}
        }
    }
}

#[component]
pub fn DropdownMenuItem(props: ChildProps) -> Element {
    rsx! {
        div { class: "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground hover:bg-accent hover:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
            {props.children}
        }
    }
}

#[component]
pub fn Badge(props: ChildProps) -> Element {
    rsx! {
        span {
            class: format!(
                "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 border-transparent bg-primary text-primary-foreground hover:bg-primary/80 {}",
                props.class,
            ),
            {props.children}
        }
    }
}

#[component]
pub fn Avatar(props: ChildProps) -> Element {
    rsx! {
        span { class: "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full",
            {props.children}
        }
    }
}

#[component]
pub fn AvatarImage(src: String, alt: String) -> Element {
    rsx! {
        img {
            src,
            alt,
            class: "aspect-square h-full w-full object-cover",
        }
    }
}

#[component]
pub fn AvatarFallback(props: ChildProps) -> Element {
    rsx! {
        span { class: "flex h-full w-full items-center justify-center rounded-full bg-muted text-muted-foreground",
            {props.children}
        }
    }
}