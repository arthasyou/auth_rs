use actix_web::{post, HttpRequest, http::header};

#[post("/test")]
pub async fn test(req: HttpRequest) -> String {

    let token = req.headers().get(header::AUTHORIZATION).unwrap();
    format!("Hello test auth!{:?}", token)
}