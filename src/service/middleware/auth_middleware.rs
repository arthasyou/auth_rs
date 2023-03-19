use std::future::{ready, Ready};

use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, http::header, http::header::HeaderValue
};
use futures_util::future::LocalBoxFuture;
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
        println!("Hi from start. You requested: {}", req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }

    // fn call(&self, req: ServiceRequest) -> Self::Future {
    //     match super::super::header::parse_token(&req) {
    //         Ok(token) => {
    //             println!("Hi from start. You requested: {:?}", token);
    //             let mut new_headers = req.headers().clone(); // Clone the current headers
    //             new_headers.insert(
    //                 header::AUTHORIZATION, 
    //                 HeaderValue::from_str(&token).unwrap() // Add the token as a string value
    //             );
                
    //             req.headers_mut().clone_from(&new_headers);
    //             let fut = self.service.call(req);
    //             Box::pin(async move {
    //                 let res = fut.await?;
    //                 Ok(res)
    //             })
    //         }
    //         Err(_err) => 
    //         {
    //             Box::pin(async move {
                    
    //                 Err(Error::from(actix_web::error::ErrorUnauthorized("Missing Authorization Header")))
    //             })

    //         }
            
    //     }        
    // }
}

