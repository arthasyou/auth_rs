use actix_web::web::ServiceConfig;
use crate::handlers::user_handler::*;

pub fn user(cfg: &mut ServiceConfig) {
    cfg.service(sign_up);
}

