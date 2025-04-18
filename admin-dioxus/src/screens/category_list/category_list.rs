use dioxus::prelude::*;

use crate::ui::custom::*;

#[component]
pub fn CategoryListScreen() -> Element {
    rsx! {
        div { "Category List (placeholder)" }
        Command {
            CommandInput {}
            CommandList {
                CommandEmpty { "No results found." }
                CommandGroup { heading: "Suggestions",
                    CommandItem { value: "calendar", "Calendar" }
                    CommandItem { value: "search", "Search Emoji" }
                    CommandItem { value: "calculator", "Calculator" }
                }
                CommandSeparator {}
                CommandGroup { heading: "Settings",
                    CommandItem { value: "profile", "Profile" }
                    CommandItem { value: "billing", "Billing" }
                    CommandItem { value: "settings", "Settings" }
                }
            }
        }
    }
}
