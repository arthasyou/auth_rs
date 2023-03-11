use chrono::{NaiveDateTime};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use validator::Validate;
use mongodb::{bson::{oid::ObjectId}};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
    #[serde(rename = "_id")]        // use MongoDB _id as primary key
    pub id: Option<ObjectId>,
    pub username: String,
    pub psssword_hash: String,
    pub email: String,
    pub phone: String,
    pub token: String,
    pub refresh_token: String,
    pub user_id: Uuid,
    pub user_type: UserType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
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

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfile {
    pub full_name: Option<String>,
    pub bio: Option<String>,
    #[validate(url)]
    pub image: Option<String>
}