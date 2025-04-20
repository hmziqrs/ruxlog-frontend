use dioxus::prelude::*;

use crate::ui::{custom::cmdk::*, shadcn::{TabItem, Tabs}};

#[component]
pub fn CategoryListScreen() -> Element {
        let tabs = vec![
        TabItem::new(
            "First Tab".to_string(), 
            "Content for first tab".to_string(), 
            false
        ),
        TabItem::new(
            "Second Tab".to_string(), 
            "Content for second tab".to_string(), 
            false
        ),
        TabItem::new(
            "Disabled Tab".to_string(), 
            "You won't see this content".to_string(), 
            true
        ),
    ];
    let groups = vec!["Settings".to_string(), "Suggestions".to_string(), "Other".to_string()];
    
    let data = vec![
        CommandListItem::new("settings".to_string(), "Profile".to_string(), false),
        CommandListItem::new("settings".to_string(), "Billing".to_string(), false),
        CommandListItem::new("settings".to_string(), "Settings".to_string(), false),
        CommandListItem::new("settings".to_string(), "Display".to_string(), false),
        
        //
        CommandListItem::new("suggestions".to_string(), "Calendar".to_string(), false),
        CommandListItem::new("suggestions".to_string(), "Emoji".to_string(), false),
        CommandListItem::new("suggestions".to_string(), "Calculator".to_string(), true),
        
        //
        CommandListItem::new("other".to_string(), "Camera".to_string(), true),
        CommandListItem::new("other".to_string(), "Video".to_string(), true),
        
    ];

    rsx! {
        div { "Category List (placeholder)" }
        Tabs { tabs, default_index: 0 }
        Cmdk { groups, data, reset_on_select: true }
    }
}
