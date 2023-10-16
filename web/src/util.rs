use crate::r#const::REST_API_URL;
use serde::de::DeserializeOwned;

use gloo::net::http::Request;
use serde_json::Value;

pub async fn request_post<T: DeserializeOwned>(uri: &str, body: Value) -> T {
    Request::post(&format!("{REST_API_URL}/{uri}"))
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .unwrap()
        .send()
        .await
        .unwrap()
        .json::<T>()
        .await
        .unwrap()
}

pub async fn request_get<T: DeserializeOwned>(uri: &str) -> T {
    Request::get(&format!("{REST_API_URL}/{uri}"))
        .send()
        .await
        .unwrap()
        .json::<T>()
        .await
        .unwrap()
}
