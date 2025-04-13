use std::collections::HashMap;
use dioxus::prelude::*;
use validator::{Validate, ValidationError};

use crate::hooks::{OxForm, OxFormModel};

#[derive(Debug, Validate, Clone, PartialEq)]
pub struct TagForm {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(length(min = 1, message = "Slug is required"))]
    #[validate(custom(function = "validate_slug"))]
    pub slug: String,
    
    pub description: String,
}

fn validate_slug(slug: &str) -> Result<(), ValidationError> {
    let regex = regex::Regex::new(r"^[a-z0-9-_]+$").unwrap();
    if !regex.is_match(slug) {
        return Err(ValidationError::new(
            "Slug can only contain lowercase letters, numbers, hyphens and underscores",
        ));
    }
    Ok(())
}

impl TagForm {
    pub fn new() -> Self {
        TagForm {
            name: String::new(),
            slug: String::new(),
            description: String::new(),
        }
    }

    pub fn sanitize_slug(text: &str) -> String {
        let text = text.to_lowercase();
        let text = regex::Regex::new(r"[^\w\s-]")
            .unwrap()
            .replace_all(&text, "")
            .to_string();
        let text = regex::Regex::new(r"\s+")
            .unwrap()
            .replace_all(&text, "-")
            .to_string();
        let text = regex::Regex::new(r"-+")
            .unwrap()
            .replace_all(&text, "-")
            .to_string();
        let text = regex::Regex::new(r"^-+|-+$")
            .unwrap()
            .replace_all(&text, "")
            .to_string();
        text
    }
}

impl OxFormModel for TagForm {
    fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("name".to_string(), self.name.clone());
        map.insert("slug".to_string(), self.slug.clone());
        map.insert("description".to_string(), self.description.clone());
        map
    }

    fn update_field(&mut self, name: String, value: &str) {
        match name.as_str() {
            "name" => self.name = value.to_string(),
            "slug" => self.slug = value.to_string(),
            "description" => self.description = value.to_string(),
            _ => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseTagForm {
    pub form: Signal<OxForm<TagForm>>,
    pub auto_slug: Signal<bool>,
}

pub fn use_tag_form(initial_state: TagForm) -> UseTagForm {
    let mut form_signal: Signal<OxForm<TagForm>> = use_signal(|| OxForm::new(initial_state));
    let auto_slug: Signal<bool> = use_signal(|| true);
    let form_data = form_signal.read();
    
    let name = form_data.data.name.clone();
    
    use_effect(move || {
        if *auto_slug.read() && !name.is_empty() {
            let sanitized_slug = TagForm::sanitize_slug(&name);
            form_signal.write().update_field("slug", sanitized_slug.to_string());
        }
    });

    UseTagForm {
        form: form_signal,
        auto_slug,
    }
}