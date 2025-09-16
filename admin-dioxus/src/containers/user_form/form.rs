use dioxus::prelude::*;
use std::collections::HashMap;
use validator::{Validate, ValidationError};

use crate::hooks::{OxForm, OxFormModel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    SuperAdmin,
    Admin,
    Moderator,
    Author,
    User,
}

impl UserRole {
    pub fn to_string(&self) -> String {
        match self {
            UserRole::SuperAdmin => "super-admin".to_string(),
            UserRole::Admin => "admin".to_string(),
            UserRole::Moderator => "moderator".to_string(),
            UserRole::Author => "author".to_string(),
            UserRole::User => "user".to_string(),
        }
    }

    pub fn from_string(value: &str) -> Result<Self, ValidationError> {
        match value {
            "super-admin" => Ok(UserRole::SuperAdmin),
            "admin" => Ok(UserRole::Admin),
            "moderator" => Ok(UserRole::Moderator),
            "author" => Ok(UserRole::Author),
            "user" => Ok(UserRole::User),
            _ => Err(ValidationError::new("Invalid role")),
        }
    }
}

#[derive(Debug, Validate, Clone, PartialEq)]
pub struct UserForm {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    pub role: String,

    pub is_verified: bool,

    #[validate(custom(function = "validate_password"))]
    pub password: Option<String>,

    pub is_update: bool,
}

fn validate_password(password: &&String) -> Result<(), ValidationError> {
    if password.is_empty() {
        Err(ValidationError::new("Password cannot be empty"))
    } else {
        Ok(())
    }
}

impl UserForm {
    pub fn new() -> Self {
        UserForm {
            name: String::new(),
            email: String::new(),
            role: UserRole::User.to_string(),
            is_verified: false,
            password: Some(String::new()),
            is_update: false,
        }
    }

    pub fn update(user: Self) -> Self {
        let mut updated_user = user;
        updated_user.is_update = true;
        updated_user.password = None;
        updated_user
    }
}

impl OxFormModel for UserForm {
    fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("name".to_string(), self.name.clone());
        map.insert("email".to_string(), self.email.clone());
        map.insert("role".to_string(), self.role.to_string());
        map.insert("is_verified".to_string(), self.is_verified.to_string());

        if let Some(password) = &self.password {
            map.insert("password".to_string(), password.clone());
        } else {
            map.insert("password".to_string(), String::new());
        }

        map
    }

    fn update_field(&mut self, name: String, value: &str) {
        match name.as_str() {
            "name" => self.name = value.to_string(),
            "email" => self.email = value.to_string(),
            "role" => self.role = value.to_string(),
            "is_verified" => self.is_verified = value.parse().unwrap_or(false),
            "password" => {
                if self.is_update && value.is_empty() {
                    self.password = None;
                } else {
                    self.password = Some(value.to_string());
                }
            }
            _ => {}
        }
    }
}

pub fn use_user_form(initial_state: UserForm) -> Signal<OxForm<UserForm>> {
    let form_signal: Signal<OxForm<UserForm>> = use_signal(|| OxForm::new(initial_state));
    form_signal
}
