pub mod user_router;
pub mod auth_router;

use user_router::*;
use auth_router::*;

use actix_web::{web::{ServiceConfig, scope}};

pub fn root(cfg: &mut ServiceConfig) {
    cfg
        .service(scope("/user").configure(user))
        .service(scope("/auth").configure(auth))
    ;
}
