use gloo_net::http::{Request, RequestBuilder};
use gloo_storage::{LocalStorage, Storage};
use serde::Serialize;
use web_sys::RequestCredentials;
use crate::env::{APP_API_URL, APP_CSRF_TOKEN};

const AUTH_TOKEN_KEY: &str = "auth_token";

pub fn get_base_url() -> String {
    format!("http://{}", APP_API_URL)
}

pub fn get_auth_token() -> Option<String> {
    LocalStorage::get(AUTH_TOKEN_KEY).ok()
}

fn create_headers(mut req: RequestBuilder) -> RequestBuilder {
    req = req.header("Content-Type", "application/json");
    if let Some(token) = get_auth_token() {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }
    req = req.header("csrf-token", APP_CSRF_TOKEN);
    req.credentials(RequestCredentials::Include)
}

pub fn get(endpoint: &str) -> RequestBuilder {
    let url = format!("{}{}", get_base_url(), endpoint);
    let req = Request::get(&url);
    create_headers(req)
}

pub fn post<T: Serialize>(endpoint: &str, body: &T) -> Request {
    let url = format!("{}{}", get_base_url(), endpoint);
    let req_pre = Request::post(&url);
    let req = create_headers(req_pre).json(body).unwrap();
    req
}

pub fn put<T: Serialize>(endpoint: &str, body: &T) -> Request {
    let url = format!("{}{}", get_base_url(), endpoint);
    let req_pre = Request::put(&url);
    let req = create_headers(req_pre).json(body).unwrap();
    req
}

pub fn delete(endpoint: &str) -> RequestBuilder {
    let url = format!("{}{}", get_base_url(), endpoint);
    let req = Request::delete(&url);
    create_headers(req)
}
