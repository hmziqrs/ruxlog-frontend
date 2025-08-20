use gloo_net::http::{Request, RequestBuilder};
use serde::Serialize;
use web_sys::RequestCredentials;
use crate::env::{APP_API_URL, APP_CSRF_TOKEN};

pub fn get_base_url() -> String {
    format!("http://{}", APP_API_URL)
}

fn create_headers(mut req: RequestBuilder) -> RequestBuilder {
    req = req.header("Content-Type", "application/json").header("csrf-token", APP_CSRF_TOKEN).credentials(RequestCredentials::Include);
    req
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
