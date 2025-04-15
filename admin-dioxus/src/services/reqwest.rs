use gloo_storage::{LocalStorage, Storage};
use once_cell::sync::Lazy;
use reqwest::{header, Client, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Mutex;


use crate::env::{APP_API_URL, APP_CSRF_TOKEN};

// Token key in local storage
const AUTH_TOKEN_KEY: &str = "auth_token";

// Global client instance using once_cell
static CLIENT: Lazy<Mutex<Client>> = Lazy::new(|| Mutex::new(create_client()));

// Create a new client with default configuration
fn create_client() -> Client {
    Client::builder()
        .build()
        .expect("Failed to create HTTP client")
}

// Get the base URL with proper http/https prefix
pub fn get_base_url() -> String {
    format!("http://{}", APP_API_URL)
}

// Get the stored authentication token if available
pub fn get_auth_token() -> Option<String> {
    LocalStorage::get(AUTH_TOKEN_KEY).ok()
}

// Helper to create headers with auth token if available
fn create_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();

    // Set default headers
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    // Add auth token if available
    if let Some(token) = get_auth_token() {
        if let Ok(auth_value) = header::HeaderValue::from_str(&format!("Bearer {}", token)) {
            headers.insert(header::AUTHORIZATION, auth_value);
        }
    }

    // Add CSRF token
    if let Ok(csrf_value) = header::HeaderValue::from_str(APP_CSRF_TOKEN) {
        headers.insert("csrf-token", csrf_value);
    }

    // headers.insert(
    //     "Access-Control-Allow-Credentials",
    //     header::HeaderValue::from_static("true"),
    // );

    // Add X-Requested-With header for CORS
    // headers.insert(
    //     "x-requested-with",
    //     header::HeaderValue::from_static("XMLHttpRequest"),
    // );

    // // SameSite=None is crucial for cross-origin cookies
    // headers.insert(
    //     "cookie-same-site",
    //     header::HeaderValue::from_static("None"),
    // );

    headers
}

// Get the global client instance
pub fn get_client() -> Client {
    CLIENT.lock().unwrap().clone()
}

// Helper for GET requests
pub fn get<T>(endpoint: &str) -> RequestBuilder
where
    T: DeserializeOwned,
{
    let url = format!("{}{}", get_base_url(), endpoint);
    let headers = create_headers();

    get_client().get(&url).headers(headers)
}

// Helper for POST requests
pub fn post<T>(endpoint: &str, body: &T) -> RequestBuilder
where
    T: Serialize,
{
    let url = format!("{}{}", get_base_url(), endpoint);
    let headers = create_headers();

    get_client().post(&url).headers(headers).json(body)
}

// Helper for PUT requests
pub fn put<T>(endpoint: &str, body: &T) -> RequestBuilder
where
    T: Serialize,
{
    let url = format!("{}{}", get_base_url(), endpoint);
    let headers = create_headers();

    get_client().put(&url).headers(headers).json(body)
}

// Helper for DELETE requests
pub fn delete(endpoint: &str) -> RequestBuilder {
    let url = format!("{}{}", get_base_url(), endpoint);
    let headers = create_headers();

    get_client().delete(&url).headers(headers)
}
