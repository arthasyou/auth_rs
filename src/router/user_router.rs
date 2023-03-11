use actix_web::web::ServiceConfig;
use crate::handlers::user_handler::*;

pub fn user(cfg: &mut ServiceConfig) {
    cfg
        .service(sign_up)
        .service(get_user)
        .service(get_all)
    ;
}

