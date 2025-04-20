use dioxus::prelude::*;
use im::HashMap;

// So idea is we will always have consumable list from the props
// we can use that list to compute filtered groups at runtime
// and then we can use that to render the list
// also on every search reset active index to 0
// Also have a way for disabled items
// as soon as input is focused we should set the active index to 0

#[derive(Props, PartialEq, Clone)]
pub struct CommandListProps {
    pub data: Vec<CommandListItem>,
    pub initial_selected: CommandListItem,
    pub children: Element,
    pub on_select: fn(CommandListItem),
    pub reset_on_select: bool,
    pub groups: Vec<String>
}

#[derive( PartialEq, Clone)]
pub struct CommandListItem {
    pub label: String,
    pub value: String,
    pub group_id: String,
    pub disabled: bool,
}

#[derive(PartialEq, Clone)]
pub struct CommandContext {
    pub search: String,
    pub active_index: i32,
    pub selected_index: i32,
    pub groups: Vec<CommandListGroup>,

    // This shouldn't be exposed
    internal_groups: Vec<String>,
    list: Vec<CommandListItem>,
    reset_on_select: bool,
}

#[derive(PartialEq, Clone)]
pub struct CommandListGroup {
    pub label: String,
    pub id: String,
    pub items: Vec<CommandListItem>,
}

impl CommandListGroup {
    pub fn new(label: String, id: String, items: Vec<CommandListItem>) -> Self {
        // to create it. lowercase, trim, replace spaces with dashes
        // let id = label.to_lowercase().trim().replace(" ", "-");
        Self { label, items, id }
    }
}


impl CommandContext {
    pub fn new(groups: Vec<String>, list: Vec<CommandListItem>, reset: Option<bool>) -> Self {
        let mut g_map: HashMap<String, CommandListGroup> = HashMap::new();

        for item in &list {
            let group = g_map.entry(item.group_id.clone()).or_insert_with(|| {
                CommandListGroup::new(item.group_id.clone(), item.group_id.clone(), Vec::new())
            });
            group.items.push(item.clone());
        }

        let parsed_groups: Vec<CommandListGroup> = g_map.into_iter().map(|(_, v)| v).collect();

        Self {
            search: String::new(),
            active_index: 0,
            selected_index: 0,
            groups: parsed_groups,


            internal_groups: groups.clone(),
            list: list.clone(),
            reset_on_select: reset.unwrap_or(false),
        }
    }

    pub fn set_search(&mut self, search: String) {
        self.search = search;

        self.active_index = 0;

        self.compute_internal_groups();
    }

    fn compute_internal_groups(&mut self) {
        let mut g_map: HashMap<String, CommandListGroup> = HashMap::new();

        for item in &self.list {
            if self.search.is_empty() || item.label.to_lowercase().contains(&self.search.to_lowercase()) {
                let group = g_map.entry(item.group_id.clone()).or_insert_with(|| {
                    CommandListGroup::new(item.group_id.clone(), item.group_id.clone(), Vec::new())
                });
                group.items.push(item.clone());
            }
        }

        self.groups = g_map.into_iter().map(|(_, v)| v).collect();
    }
}