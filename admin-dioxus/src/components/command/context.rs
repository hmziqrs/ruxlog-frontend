use crate::components::command::state::{CommandFilter, CommandState};
use dioxus::prelude::*;
use dioxus_signals::Signal;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

// Represents the data associated with a registered item
#[derive(Clone, Debug, PartialEq)]
pub struct ItemData {
    pub value: String,
    pub keywords: Vec<String>,
    pub group_id: Option<String>,
    pub node_ref: Option<UnmountedElement>, // Store unmounted element for potential reordering/access
}

// Represents the data associated with a registered group
#[derive(Clone, Debug, PartialEq, Default)]
pub struct GroupData {
    pub items: HashSet<String>, // Set of item IDs in this group
    pub force_mount: bool,
    pub node_ref: Option<UnmountedElement>, // Store unmounted element for potential reordering/access
}

// Shared state and callbacks accessible via context
#[derive(Clone)]
pub struct CommandContext {
    // State signals (already wrapped in Signal)
    pub state: CommandState,

    // Configuration
    pub label: Signal<Option<String>>,
    pub should_filter: Signal<bool>,
    pub filter_fn: Signal<CommandFilter>,
    pub loop_selection: Signal<bool>,
    pub disable_pointer_selection: Signal<bool>,
    pub vim_bindings: Signal<bool>,
    pub on_value_change: Signal<Option<Callback<String>>>,

    // Internal registry and refs
    // Use Rc<RefCell<>> for interior mutability needed for registration/unregistration in effects
    pub items: Rc<RefCell<HashMap<String, ItemData>>>, // item_id -> ItemData
    pub groups: Rc<RefCell<HashMap<String, GroupData>>>, // group_id -> GroupData
    pub item_ids_order: Rc<RefCell<Vec<String>>>, // Maintain insertion order for default sort
    pub group_ids_order: Rc<RefCell<Vec<String>>>, // Maintain insertion order for default sort

    // IDs generated in the root Command component
    pub list_id: Signal<String>,
    pub label_id: Signal<String>,
    pub input_id: Signal<String>,

    // Ref to the inner list element for scrolling and querying items
    pub list_inner_ref: Signal<Option<MountedElement>>,

    // Callbacks for children to register/unregister
    pub register_item: Callback<(String, ItemData)>,
    pub unregister_item: Callback<String>,
    pub register_group: Callback<(String, GroupData)>,
    pub unregister_group: Callback<String>,
    pub set_search: Callback<String>,
    pub set_value: Callback<(String, bool)>, // value, should_scroll
}

// Provide the context
pub fn use_command_context() -> CommandContext {
    use_context::<CommandContext>()
}

// Provide group-specific context
#[derive(Clone, Copy)]
pub struct GroupContext {
    pub id: Signal<Option<String>>,
    pub force_mount: Signal<bool>,
}

pub fn use_group_context() -> GroupContext {
    use_context::<GroupContext>()
}
