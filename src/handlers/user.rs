use actix_web::{post, web::{Json, Data}, Result, HttpResponse};
use log::debug;
use crate::{models::{json_response, json_response_empty}, models::{user::*}, db::mongodb::MongoDB};
use validator::Validate;
use mongodb::{ bson::doc };
use crate::service::jwt;


#[post("/signup")]
async fn sign_up(new_user: Json<NewUser>, db: Data<MongoDB>) -> Result<HttpResponse> {
    match new_user.validate() {
        Ok(_) => {           
            let new_user = new_user.into_inner();
            db.insert_one("user", new_user).await.unwrap();
            json_response_empty()
        },
        Err(err) => {
            debug!("{:?}", err);
            Ok(HttpResponse::BadRequest().json(err.to_string()))
        }
    }    
}

#[post("/login")]
async fn login(login: Json<Login>, db: Data<MongoDB>) -> Result<HttpResponse> {
    let doc = doc! { "email": &login.account };
    println!("{:?}", doc);
    match db.find_one::<User>("user", doc).await {
        Ok(Some(user)) => {
            let (a, r) = jwt::general_token_pair(user.id.unwrap().to_string());
            let res = LoginRespon{ access:a, refresh:r };
            json_response(&res)
        }            
        Err(err) => {
            println!("{:?}", err);
            Ok(HttpResponse::BadRequest().json(err.to_string()))
        }
        _ => Ok(HttpResponse::BadRequest().json("invalid account".to_string()))

    }
}

#[post("/get")]
async fn get_user(db: Data<MongoDB>) -> Result<HttpResponse> {
    let doc = doc! {"email": "abc@163com"};
    println!("{:?}", doc);
    match db.find_one::<NewUser>("user", doc).await {
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



