use actix_web::{post};


#[post("/test")]
pub async fn test() -> String {
    format!("Hello test auth!")
}