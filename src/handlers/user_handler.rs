use actix_web::{post};


#[post("/signup")]
async fn sign_up() -> String {
    format!("Hello user!")
}