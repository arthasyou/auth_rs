pub mod user;

use actix_web::{http::header::ContentType, HttpResponse, Result};
use serde::Serialize;

// JsonResponder
pub fn json_response<T: Serialize>(item: &T) -> Result<HttpResponse> {
    let body = serde_json::to_string(item)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body))
}

pub fn json_response_empty() -> Result<HttpResponse> {
    let item = Empty {};
    let body = serde_json::to_string(&item)?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body))
}

#[derive(Debug, Serialize)]
struct Empty {}

#[derive(Debug, Serialize)]
pub struct CommonResponse {
    pub code: u16,
    pub data: serde_json::Value,
    pub message: String,
}
