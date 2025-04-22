use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SelectProps {
    pub groups: Vec<SelectGroup>,
    pub selected: Option<String>,
    pub on_select: Option<fn(String)>,
    pub placeholder: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectContext {
    pub groups: Vec<SelectGroup>,
    pub selected: Option<String>,
    
    pub is_open: bool,
    pub internal_group: Vec<InternalSelectGroup>,
    pub max_index: usize,
    pub active_index: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectGroup {
    pub label: String,
    pub items: Vec<String>,
}

impl SelectGroup {
    pub fn new(label: String, items: Vec<String>) -> Self {
        Self { label, items }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InternalSelectGroup {
    pub label: String,
    pub items: Vec<InternalSelectItem>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct InternalSelectItem {
    pub value: String,
    pub label: String,
    pub index: usize,
}

impl InternalSelectItem {
    pub fn new(label: String, index: usize) -> Self {
        let value = label.clone().to_lowercase().replace(" ", "-");
        Self {
            value,
            label,
            index,
        }
    }
}

impl SelectContext {
    pub fn new(groups: Vec<SelectGroup>, selected: Option<String>) -> Self {
        let mut internal_groups = Vec::new();
        let mut index = 0;
        for group in groups.iter() {
            let parsed_items = {
                let mut internal_items = Vec::<InternalSelectItem>::new();
                for item in group.items.iter() {
                    let internal_item = InternalSelectItem::new(item.clone(), index);
                    internal_items.push(internal_item);
                    index += 1;
                }
                internal_items
            };
            internal_groups.push(InternalSelectGroup {
                label: group.label.clone(),
                items: parsed_items,
            });
        }
        Self {
            selected,
            is_open: false,
            groups,
            active_index: 0,
            internal_group: internal_groups,
            max_index: index,
        }
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn next_index(&mut self) {
        if self.active_index < self.max_index  {
            self.active_index += 1;
        } else {
            self.active_index = 0;
        }
    }

    pub fn prev_index(&mut self) {
        if self.active_index > 0 {
            self.active_index -= 1;
        } else {
            self.active_index = self.max_index;
        }
    }
}