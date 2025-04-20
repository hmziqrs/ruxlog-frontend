use dioxus::prelude::*;

use crate::ui::custom::cmdk::*;

#[component]
pub fn CategoryListScreen() -> Element {
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
        Cmdk { groups, data, reset_on_select: true }
    }
}
