#![allow(non_snake_case)]
use crate::components::command::{
    context::{use_command_context, GroupContext, GroupData},
    utils::use_unique_id,
};
use dioxus::prelude::*;
use dioxus_signals::*;

#[derive(Props, Clone, PartialEq)]
pub struct CommandGroupProps {
    children: Element<'_>,
    heading: Option<Element<'_>>,
    value: Option<String>, // Must be provided if heading is not, used as ID
    #[props(default)]
    force_mount: bool,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute<'_>>,
}

pub fn CommandGroup(props: CommandGroupProps) -> Element<'_> {
    let cx = use_hook_context();
    let cmdk_context = use_command_context();
    let generated_id = use_unique_id(cx); // Generate an ID anyway

    // Use provided value or the generated ID as the group's identifier
    let group_id = use_memo(cx, (props.value.clone(), generated_id), |(val, gen_id)| {
        val.unwrap_or_else(|| gen_id.read().clone())
    });

    let heading_id = use_unique_id(cx);

    // Determine if the group should be rendered based on filtering/force_mount
    let is_visible = use_memo(
        cx,
        (
            props.force_mount,
            cmdk_context.should_filter,
            cmdk_context.state.search,
            cmdk_context.state.filtered,
            group_id.clone(),
        ),
        |(forced, should_filter, search, filtered, id)| {
            if forced || !*should_filter.read() || search.read().is_empty() {
                // If not filtering or search is empty, visibility depends on having *any* items (handled by registration)
                // For simplicity here, assume visible if not actively filtered out. Refinement might be needed.
                true
            } else {
                filtered.read().groups.contains(&id)
            }
        },
    );

    // Register/Unregister Group
    use_effect(cx, &group_id, |id_val| {
        let id = id_val.clone();
        let data = GroupData {
            force_mount: props.force_mount,
            ..Default::default() // Items are managed by Item registration
        };
        cmdk_context.register_group.call((id.clone(), data));

        // Cleanup function
        let unregister_group = cmdk_context.unregister_group.clone();
        async move {
            unregister_group.call(id);
        }
    });

     // Update registration if force_mount changes
     use_effect(cx, (group_id.clone(), props.force_mount), |(id_val, forced)| {
         let id = id_val.clone();
         let data = GroupData {
             force_mount: forced,
             ..Default::default()
         };
         cmdk_context.register_group.call((id.clone(), data));
         async move {}
     });


    // Provide context for items within this group
    let group_context_value = GroupContext {
        id: signal(Some(group_id.clone())),
        force_mount: signal(props.force_mount),
    };

    if !*is_visible {
        return None; // Don't render if filtered out and not force_mount
    }

    rsx! {
        div {
            ..props.attributes,
            key: "{group_id}", // Use key for efficient updates
            "cmdk-group": "",
            "data-value": "{group_id}", // For potential selection logic later
            role: "presentation", // Group itself isn't selectable

            // Render heading if provided
            if let Some(heading) = &props.heading {
                div {
                    "cmdk-group-heading": "",
                    "aria-hidden": "true",
                    id: "{heading_id.read()}",
                    heading
                }
            }

            // Provide context and render children in their own div
            div {
                 "cmdk-group-items": "",
                 role: "group", // Items container role
                 "aria-labelledby": if props.heading.is_some() { Some(heading_id.read().clone()) } else { None },
                 ContextProvider {
                    context: group_context_value,
                    &props.children
                }
            }
        }
    }
}

