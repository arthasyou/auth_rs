// use chrono::{NaiveDateTime};
// use uuid::Uuid;
use serde::{Serialize, Deserialize};
use validator::Validate;
use mongodb::{bson::{oid::ObjectId}};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]        // use MongoDB _id as primary key
    pub id: Option<ObjectId>,
    pub username: String,
    // pub psssword_hash: String,    
    pub email: String,
    pub password: String,
    pub phone: String,
    // pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserType {    
    User = 1,
    Admin = 2,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 1, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 20))]
    pub password: String,
    #[validate(length(min = 6, max = 20))]
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub account: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRespon {
    pub access: String,
    pub refresh: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    pub token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfile {
    pub full_name: Option<String>,
    pub bio: Option<String>,
    #[validate(url)]
    pub image: Option<String>
}