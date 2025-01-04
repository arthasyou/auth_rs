use crate::models::CommonResponse;
use actix_web::get;
use actix_web::{post, web::Data, HttpRequest, HttpResponse, Result};
// use log::debug;
use crate::{db::mongodb::MongoDB, models::json_response, models::user::*, service::jwt};
// use validator::Validate;
use crate::service::header::{parse_oid, parse_oid_string};
use mongodb::bson::doc;

#[post("/test")]
pub async fn test(req: HttpRequest, db: Data<MongoDB>) -> Result<HttpResponse> {
    let oid = parse_oid(&req);
    let doc = doc! { "_id": &oid };
    println!("{:?}", doc);
    match db.find_one::<User>("user", doc).await {
        Ok(Some(user)) => json_response(&user),
        Err(err) => {
            println!("{:?}", err);
            Ok(HttpResponse::BadRequest().json(err.to_string()))
        }
        _ => Ok(HttpResponse::BadRequest().json("invalid account".to_string())),
    }
}

#[get("/info")]
pub async fn info(_req: HttpRequest) -> Result<HttpResponse> {
    let user = UserInfo {
        roles: vec![],
        real_name: "admin".to_string(),
    };
    let res = CommonResponse {
        code: 0,
        data: user.into_json(),
        message: "success".to_string(),
    };
    json_response(&res)

    // let oid = parse_oid(&req);
    // let doc = doc! { "_id": &oid };
    // println!("{:?}", doc);
    // match db.find_one::<User>("user", doc).await {
    //     Ok(Some(user)) => json_response(&user),
    //     Err(err) => {
    //         println!("{:?}", err);
    //         Ok(HttpResponse::BadRequest().json(err.to_string()))
    //     }
    //     _ => Ok(HttpResponse::BadRequest().json("invalid account".to_string())),
    // }
}

#[post("/refresh")]
pub async fn refresh_token(req: HttpRequest) -> Result<HttpResponse> {
    let oid = parse_oid_string(&req);
    let token = jwt::general_access_token(oid);
    json_response(&AccessToken { token })
}
