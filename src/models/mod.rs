pub mod user_model;

use serde::{Serialize};
use actix_web::{HttpResponse, http::header::ContentType, Result};

// JsonResponder
pub fn json_response<T: Serialize>(item: &T) -> Result<HttpResponse> {

    let body = serde_json::to_string(item)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
    )    
}

pub fn json_response_empty() -> Result<HttpResponse> {
    let item = Empty{};
    let body = serde_json::to_string(&item)?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
    )    
}

#[derive(Debug, Serialize)]    
struct Empty {}