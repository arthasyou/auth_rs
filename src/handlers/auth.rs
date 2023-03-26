

use actix_web::{post, web::Data, Result, HttpRequest, HttpResponse };
// use log::debug;
use crate::{models::json_response, models::{user::*}, db::mongodb::MongoDB, service::jwt};
// use validator::Validate;
use mongodb::bson::doc;
use crate::service::header::{ parse_oid, parse_oid_string };


#[post("/test")]
pub async fn test(req: HttpRequest, db: Data<MongoDB>) -> Result<HttpResponse> {
    let oid = parse_oid(&req);
    let doc = doc! { "_id": &oid };
    println!("{:?}", doc);
    match db.find_one::<User>("user", doc).await {
        Ok(Some(user)) => {            
            json_response(&user)
        }            
        Err(err) => {
            println!("{:?}", err);
            Ok(HttpResponse::BadRequest().json(err.to_string()))
        }
        _ => Ok(HttpResponse::BadRequest().json("invalid account".to_string()))

    }
}

#[post("/refresh")]
pub async fn refresh_token(req: HttpRequest) -> Result<HttpResponse> {
    let oid = parse_oid_string(&req);
    let token = jwt::general_access_token(oid);
    json_response(&AccessToken{token})
}
