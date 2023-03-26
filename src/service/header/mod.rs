use std::str::FromStr;

use crate::service::jwt::{ validate_access_token };
use actix_web::{
    HttpRequest, HttpMessage,
    dev::ServiceRequest,
    http::header,
    Result,
    error,
};
use mongodb::bson::oid::ObjectId;


pub fn parse_token(req: &ServiceRequest) -> Result<String> {
    let header = req.headers().get(header::AUTHORIZATION)
        .ok_or(error::ErrorUnauthorized("Authorization header required"))?;

    let mut parts = header.to_str().unwrap().splitn(2, ' ');
    match parts.next() {
        Some(scheme) if scheme == "Bearer" => {}
        _ => return Err(error::ErrorUnauthorized("Invalid Authorization Token"))
    }

    let token = parts.next()
        .ok_or(error::ErrorUnauthorized("Invalid Authorization Token"))?;

    match validate_access_token(token) {
        Ok(data) => Ok(data.claims.sub),
        _ => return Err(error::ErrorUnauthorized("Invalid Authorization Token"))
    }
}

pub fn parse_oid(req: &HttpRequest) -> ObjectId {
    let sub = req.extensions().get::<String>().unwrap().clone();
    ObjectId::from_str(&sub).unwrap()
}

