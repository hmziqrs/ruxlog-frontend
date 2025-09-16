use dioxus::prelude::*;
use std::collections::HashMap;
use validator::{Validate, ValidationError};

use crate::hooks::{OxForm, OxFormModel};
use crate::store::{CategoryAddPayload, CategoryEditPayload};
use crate::utils::colors::get_contrast_yiq;

#[derive(Debug, Validate, Clone, PartialEq)]
pub struct CategoryForm {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(length(min = 1, message = "Slug is required"))]
    #[validate(custom(function = "validate_slug"))]
    pub slug: String,

    pub description: String,
    // Hex color like #3b82f6
    pub color: String,
    // Optional override for text color when custom_text_color is true
    pub text_color: String,
    // Whether to use custom text color instead of auto-contrast
    pub custom_text_color: bool,
    // Visibility: whether the category is publicly visible
    pub active: bool,

    pub cover_image: String,

    pub logo_image: String,

    // keep as string for input, will parse into i32 for payloads
    pub parent_id: String,
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

impl CategoryForm {
    pub fn new() -> Self {
        CategoryForm {
            name: String::new(),
            slug: String::new(),
            description: String::new(),
            color: "#3b82f6".to_string(),
            // default text color based on auto contrast of default bg
            text_color: get_contrast_yiq("#3b82f6").to_string(),
            custom_text_color: false,
            active: true,
            cover_image: String::new(),
            logo_image: String::new(),
            parent_id: String::new(),
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

    // Compute the final text color respecting user choice or auto-contrast
    pub fn effective_text_color(&self) -> String {
        if self.custom_text_color && !self.text_color.trim().is_empty() {
            self.text_color.clone()
        } else {
            get_contrast_yiq(&self.color).to_string()
        }
    }

    // Convert the form to the backend add payload contract
    pub fn to_add_payload(&self) -> CategoryAddPayload {
        let description = if self.description.trim().is_empty() {
            None
        } else {
            Some(self.description.clone())
        };
        let text_color = Some(self.effective_text_color());
        let is_active = Some(self.active);
        let cover_image = if self.cover_image.trim().is_empty() {
            None
        } else {
            Some(self.cover_image.clone())
        };
        let logo_image = if self.logo_image.trim().is_empty() {
            None
        } else {
            Some(self.logo_image.clone())
        };
        let parent_id = if self.parent_id.trim().is_empty() {
            None
        } else {
            self.parent_id.trim().parse::<i32>().ok()
        };

        CategoryAddPayload {
            name: self.name.clone(),
            slug: self.slug.clone(),
            color: self.color.clone(),
            text_color,
            is_active,
            cover_image,
            description,
            logo_image,
            parent_id,
        }
    }

    // Convert the form to the backend edit payload contract
    pub fn to_edit_payload(&self) -> CategoryEditPayload {
        let description = if self.description.trim().is_empty() {
            Some(None)
        } else {
            Some(Some(self.description.clone()))
        };
        let cover_image = if self.cover_image.trim().is_empty() {
            Some(None)
        } else {
            Some(Some(self.cover_image.clone()))
        };
        let logo_image = if self.logo_image.trim().is_empty() {
            Some(None)
        } else {
            Some(Some(self.logo_image.clone()))
        };
        let parent_id = if self.parent_id.trim().is_empty() {
            Some(None)
        } else {
            self.parent_id
                .trim()
                .parse::<i32>()
                .ok()
                .map(|v| Some(v))
                .or(Some(None))
        };

        CategoryEditPayload {
            name: Some(self.name.clone()),
            slug: Some(self.slug.clone()),
            parent_id,
            description,
            cover_image,
            logo_image,
            color: Some(self.color.clone()),
            text_color: Some(self.effective_text_color()),
            is_active: Some(self.active),
        }
    }
}

impl OxFormModel for CategoryForm {
    fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("name".to_string(), self.name.clone());
        map.insert("slug".to_string(), self.slug.clone());
        map.insert("description".to_string(), self.description.clone());
        map.insert("color".to_string(), self.color.clone());
        map.insert("text_color".to_string(), self.text_color.clone());
        map.insert(
            "custom_text_color".to_string(),
            if self.custom_text_color {
                "true".to_string()
            } else {
                "false".to_string()
            },
        );
        map.insert(
            "active".to_string(),
            if self.active {
                "true".to_string()
            } else {
                "false".to_string()
            },
        );
        map.insert("cover_image".to_string(), self.cover_image.clone());
        map.insert("logo_image".to_string(), self.logo_image.clone());
        map.insert("parent_id".to_string(), self.parent_id.to_string());

        map
    }

    fn update_field(&mut self, name: String, value: &str) {
        match name.as_str() {
            "name" => self.name = value.to_string(),
            "slug" => self.slug = value.to_string(),
            "description" => self.description = value.to_string(),
            "color" => self.color = value.to_string(),
            "text_color" => self.text_color = value.to_string(),
            "custom_text_color" => {
                let v = value.trim().to_lowercase();
                self.custom_text_color = matches!(v.as_str(), "true" | "1" | "yes" | "on");
            }
            "active" => {
                let v = value.trim().to_lowercase();
                self.active = matches!(v.as_str(), "true" | "1" | "yes" | "on");
            }
            "cover_image" => self.cover_image = value.to_string(),
            "logo_image" => self.logo_image = value.to_string(),
            "parent_id" => self.parent_id = value.to_string(),
            _ => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseCategoryForm {
    pub form: Signal<OxForm<CategoryForm>>,
    pub auto_slug: Signal<bool>,
}

pub fn use_category_form(initial_state: CategoryForm) -> UseCategoryForm {
    let mut form_signal: Signal<OxForm<CategoryForm>> = use_signal(|| OxForm::new(initial_state));
    let auto_slug: Signal<bool> = use_signal(|| true);
    let form_data = form_signal.read();

    let name = form_data.data.name.clone();

    use_effect(move || {
        if *auto_slug.read() && !name.is_empty() {
            let sanitized_slug = CategoryForm::sanitize_slug(&name);
            form_signal
                .write()
                .update_field("slug", sanitized_slug.to_string());
        }
    });

    UseCategoryForm {
        form: form_signal,
        auto_slug,
    }
}
