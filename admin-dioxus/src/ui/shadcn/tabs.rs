use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TabsProps {
    /// Additional CSS classes to apply to the tabs
    #[props(default)]
    pub class: Option<String>,
    /// The tabs data
    pub tabs: Vec<TabItem>,
    /// Default active tab index
    #[props(default)]
    pub default_index: usize,
}

#[derive(PartialEq, Clone)]
pub struct TabItem {
    pub label: String,
    pub content: String,
    pub disabled: bool,
}

impl TabItem {
    pub fn new(label: String, content: String, disabled: bool) -> Self {
        Self {
            label,
            content,
            disabled,
        }
    }
}

#[derive(PartialEq, Clone)]
struct TabsState {
    active_index: usize,
    items: Vec<TabItem>,
}

impl TabsState {
    fn new(tabs: Vec<TabItem>, default_index: usize) -> Self {
        Self {
            active_index: default_index,
            items: tabs,
        }
    }
    
    fn set_active_tab(&mut self, index: usize) {
        if index < self.items.len() && !self.items[index].disabled {
            self.active_index = index;
        }
    }
}

/// A simple tabs component
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let mut state = use_signal(|| TabsState::new(props.tabs.clone(), props.default_index));
    let active_index = state.read().active_index;
    let active_content = state.read().items[active_index].content.clone();
    
    let mut class = vec!["flex flex-col gap-2".to_string()];
    if let Some(custom_class) = props.class.clone() {
        class.push(custom_class);
    }

    rsx! {
        div { class: class.join(" "),
            // Tab list
            div { class: "bg-muted text-muted-foreground inline-flex h-9 w-fit items-center justify-center rounded-lg p-[3px]",
                // Tab buttons
                for (i , tab) in state.read().items.clone().into_iter().enumerate() {
                    button {
                        class: format!(
                            "inline-flex h-[calc(100%-1px)] items-center justify-center rounded-md px-3 py-1 text-sm font-medium transition-all {} {}",
                            if i == active_index { "bg-background shadow-sm text-foreground" } else { "" },
                            if tab.disabled { "opacity-50 cursor-not-allowed" } else { "" },
                        ),
                        disabled: tab.disabled,
                        onclick: move |_| {
                            state.write().set_active_tab(i);
                        },
                        {tab.label}
                    }
                }
            }
            div { class: "mt-2",
                div { class: "p-4 rounded-md", {active_content} }
            }
        }
    }
}