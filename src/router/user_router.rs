use crate::handlers::user::*;
use actix_web::web::ServiceConfig;

pub fn user(cfg: &mut ServiceConfig) {
    cfg.service(refresh_token).service(test).service(info);
}
