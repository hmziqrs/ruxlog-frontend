use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CommandRootProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    #[props(optional)]
    pub children: Element,

    // #[props]
    pub label: Option<String>,    
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandInputProps {
    #[props(default = "Type a command or search...".to_string())]
    pub placeholder: String,

    #[props(optional)]
    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandListProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    #[props(optional)]
    pub children: Element,
}
#[derive(Props, PartialEq, Clone)]
pub struct CommandItemProps {
    pub children: Element,
    pub value: Option<String>,

    #[props(optional)]
    pub on_select: Option<EventHandler<()>>,

    #[props(default = false)]
    pub disabled: bool,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandGroupProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub heading: Option<String>,
    pub children: Element,
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandSeparatorProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandLoadingProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    #[props(optional)]
    pub children: Element,
}

#[derive(Props, PartialEq, Clone)]
pub struct CommandEmptyProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    #[props(optional)]
    pub children: Element,
}
