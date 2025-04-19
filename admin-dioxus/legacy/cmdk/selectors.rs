// Selector constants (Rust style)
pub const GROUP_SELECTOR: &str = r#"[cmdk-group=""]"#;
pub const GROUP_ITEMS_SELECTOR: &str = r#"[cmdk-group-items=""]"#;
pub const GROUP_HEADING_SELECTOR: &str = r#"[cmdk-group-heading=""]"#;
pub const ITEM_SELECTOR: &str = r#"[cmdk-item=""]"#;
pub const VALID_ITEM_SELECTOR: &str = r#"[cmdk-item=""]:not([aria-disabled="true"])"#;
pub const SELECT_EVENT: &str = "cmdk-item-select";
pub const VALUE_ATTR: &str = "data-value";

// CommandFilter type and default implementation
pub type CommandFilter = fn(value: &str, search: &str, keywords: Option<&str>) -> i32;
