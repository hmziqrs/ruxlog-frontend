use std::collections::HashMap;
use dioxus::prelude::*;
use validator::{Validate, ValidationError};

use crate::hooks::{OxForm, OxFormModel};

#[derive(Debug, Validate, Clone, PartialEq)]
pub struct BlogForm {
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,

    #[validate(length(min = 1, message = "Content is required"))]
    pub content: String,

    #[validate(length(min = 1, message = "Slug is required"))]
    #[validate(custom(function = "validate_slug"))]
    pub slug: String,
    
    pub excerpt: String,
    
    pub featured_image_url: String,
    
    pub is_published: bool,
    
    pub category_id: Option<i32>,
    
    pub tag_ids: Vec<i32>,
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

impl BlogForm {
    pub fn new() -> Self {
        BlogForm {
            title: String::new(),
            content: String::new(),
            slug: String::new(),
            excerpt: String::new(),
            featured_image_url: String::new(),
            is_published: false,
            category_id: None,
            tag_ids: vec![],
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

impl OxFormModel for BlogForm {
    fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("title".to_string(), self.title.clone());
        map.insert("content".to_string(), self.content.clone());
        map.insert("slug".to_string(), self.slug.clone());
        map.insert("excerpt".to_string(), self.excerpt.clone());
        map.insert("featured_image_url".to_string(), self.featured_image_url.clone());
        map.insert("is_published".to_string(), self.is_published.to_string());
        if let Some(category_id) = self.category_id {
            map.insert("category_id".to_string(), category_id.to_string());
        }
        // Tag IDs are handled separately in the form UI
        map
    }

    fn update_field(&mut self, name: String, value: &str) {
        match name.as_str() {
            "title" => self.title = value.to_string(),
            "content" => self.content = value.to_string(),
            "slug" => self.slug = value.to_string(),
            "excerpt" => self.excerpt = value.to_string(),
            "featured_image_url" => self.featured_image_url = value.to_string(),
            "is_published" => self.is_published = value.parse().unwrap_or(false),
            "category_id" => self.category_id = value.parse().ok(),
            _ => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseBlogForm {
    pub form: Signal<OxForm<BlogForm>>,
    pub auto_slug: Signal<bool>,
}

pub fn use_blog_form(initial_state: BlogForm) -> UseBlogForm {
    let mut form_signal: Signal<OxForm<BlogForm>> = use_signal(|| OxForm::new(initial_state));
    let auto_slug: Signal<bool> = use_signal(|| true);
    let form_data = form_signal.read();
    
    let title = form_data.data.title.clone();
    
    use_effect(move || {
        if *auto_slug.read() && !title.is_empty() {
            let sanitized_slug = BlogForm::sanitize_slug(&title);
            form_signal.write().update_field("slug", sanitized_slug.to_string());
        }
    });

    UseBlogForm {
        form: form_signal,
        auto_slug,
    }
}