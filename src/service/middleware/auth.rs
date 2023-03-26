use std::future::{ready, Ready};

use actix_web::{
    HttpMessage, 
    dev::{ self, Service, ServiceRequest, ServiceResponse, Transform },
    Error, 
    
};
use futures_util::future::LocalBoxFuture;
use super::super::header::parse_token;
// use anyhow::Result;
// use crate::error::{Result, Error};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service })) 
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        
        match parse_token(&req) {
            Ok(sub) => {
                req.extensions_mut().insert(sub);                
            }
            Err(err) => {
                return Box::pin(async {
                    Err(err)
                });
            }
        }        

        let fut = self.service.call(req);      

        Box::pin(async move {
            let res = fut.await?;
            println!("Hi from response");
            Ok(res)
        })
    }

}

