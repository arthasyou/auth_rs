
use crate::error::{Result, Error};
use actix_web::{dev::ServiceRequest, http::header};
// use actix_web::{HttpRequest, http::header};

pub fn parse_token(req: &ServiceRequest) -> Result<String>{
    let header = req.headers().get(header::AUTHORIZATION)
        .ok_or(Error::AuthError("Authorization header required".to_string()))?;
        let mut parts = header.to_str()?.splitn(2, ' ');
        match parts.next() {
            Some(scheme) if scheme == "Bearer" => {}
            _ => return Err(Error::AuthError("Authorization header required".to_string()))
        }
        let token = parts.next()
            .ok_or(Error::AuthError("Authorization header required".to_string()))?;

        Ok(token.to_string())
}