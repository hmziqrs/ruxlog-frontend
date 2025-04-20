use dioxus::{html::button::disabled, prelude::*};

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
    pub disable: bool,
}

impl TabItem {
    pub fn new(label: String, disable: bool) -> Self {
        Self {
            label,
            disable,
        }
    }
}

#[derive(PartialEq, Clone)]
struct TabsState {
    active_index: usize,
    items: Vec<TabItem>,
}

impl TabsState {
    pub fn new(tabs: Vec<TabItem>, default_index: usize) -> Self {
        Self {
            active_index: default_index,
            items: tabs,
        }
    }
    
    pub fn set_active_tab(&mut self, index: usize) {
        if index < self.items.len() && !self.items[index].disable {
            self.active_index = index;
        }
    }

    pub fn next_tab(&mut self) {
        let initial_tab = self.active_index.clone();
        loop {
            let next_tab = {
                if self.active_index + 1 >= self.items.len() {
                    0
                } else {
                    self.active_index + 1
                }
            };
            if self.items[next_tab].disable {
                continue;
            }
            if next_tab == initial_tab {
                break;
            }
            self.active_index = next_tab;
            break;
        }
    }

    pub fn prev_tab(&mut self) {
        let initial_tab = self.active_index.clone();
        loop {
            let prev_tab = {
                if self.active_index == 0 {
                    self.items.len() - 1
                } else {
                    self.active_index - 1
                }
            };
            if self.items[prev_tab].disable {
                continue;
            }
            if prev_tab == initial_tab {
                break;
            }
            self.active_index = prev_tab;
        }
    }
}

/// A simple tabs component
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let mut state = use_signal(|| TabsState::new(props.tabs.clone(), props.default_index));
    let active_index = state.read().active_index;
    
    let mut class = vec!["flex flex-col gap-2".to_string()];
    if let Some(custom_class) = props.class.clone() {
        class.push(custom_class);
    }

    rsx! {
        div {
            onkeydown: move |e| {
                let key = e.key();
                match key {
                    Key::ArrowLeft => {
                        state.write().prev_tab();
                    }
                    Key::ArrowRight => {
                        state.write().next_tab();
                    }
                    _ => {}
                }
            },
            class: class.join(" "),
            // Tab list
            div { class: "bg-muted text-muted-foreground inline-flex h-9 w-fit items-center justify-center rounded-lg p-[3px]",
                // Tab buttons
                for (i , tab) in state.read().items.clone().into_iter().enumerate() {
                    button {
                        class: format!(
                            "inline-flex h-[calc(100%-1px)] items-center justify-center rounded-md px-3 py-1 text-sm font-medium transition-all {} {}",
                            if i == active_index { "bg-background shadow-sm text-foreground" } else { "" },
                            if tab.disable { "opacity-50 cursor-not-allowed" } else { "" },
                        ),
                        disabled: tab.disable,
                        onclick: move |_| {
                            state.write().set_active_tab(i);
                        },
                        {tab.label}
                    }
                }
            }
        }
    }
}