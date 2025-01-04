pub mod auth_router;
pub mod user_router;

use auth_router::*;
use user_router::*;

use crate::service::middleware::auth::Auth;
use actix_web::web::{scope, ServiceConfig};

pub fn root(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").configure(auth))
        .service(scope("/user").wrap(Auth).configure(user));
}
