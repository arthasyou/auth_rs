// use chrono::{NaiveDateTime};
// use uuid::Uuid;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")] // use MongoDB _id as primary key
    pub id: Option<ObjectId>,
    pub username: String,
    // pub psssword_hash: String,
    pub password: String,
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
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRespon {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    pub refresh: String,
}

impl LoginRespon {
    pub fn into_json(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap() // 将 LoginRespon 转换为 serde_json::Value
    }
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
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub roles: Vec<String>,
    #[serde(rename = "realName")]
    pub real_name: String,
}

impl UserInfo {
    pub fn into_json(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap() // 将 LoginRespon 转换为 serde_json::Value
    }
}
