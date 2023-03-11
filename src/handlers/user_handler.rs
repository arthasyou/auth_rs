use actix_web::{post, web::{Json, Data}, Result, HttpResponse};
use log::debug;
use crate::{models::{json_response, json_response_empty}, models::{user_model::*}, db::mongodb::MongoDB};
use validator::Validate;
use mongodb::{bson::doc};

#[post("/signup")]
async fn sign_up(user: Json<NewUser>, db: Data<MongoDB>) -> Result<HttpResponse> {
    match user.validate() {
        Ok(_) => {           
            let user = user.into_inner();
            db.insert_one("user", user).await.unwrap();
            json_response_empty()
        },
        Err(err) => {
            debug!("{:?}", err);
            Ok(HttpResponse::BadRequest().json(err.to_string()))
        }
    }    
}

#[post("/get")]
async fn get_user(db: Data<MongoDB>) -> Result<HttpResponse> {
    match db.find_one::<NewUser>("user", doc! {"email": "aghe@163com"}).await {
        Ok(Some(user)) =>
            json_response(&user),
        _ =>
            json_response_empty()
     }
}

#[post("/get_all")]
async fn get_all(db: Data<MongoDB>) -> Result<HttpResponse> {
    let mut users: Vec<NewUser> = vec![];
    match db.find("user", None, None, &mut users) .await {
        Ok(()) =>
            json_response(&users),
        _ =>
            json_response_empty()
     }
}


async fn hash_password(db: Data<MongoDB>) -> Result<HttpResponse> {
    json_response_empty()
}

async fn verify_password(db: Data<MongoDB>) -> Result<HttpResponse> {
    json_response_empty()
}

async fn login(db: Data<MongoDB>) -> Result<HttpResponse> {
    json_response_empty()
}

