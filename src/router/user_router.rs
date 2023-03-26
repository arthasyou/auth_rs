use actix_web::web::ServiceConfig;
use crate::handlers::user::*;


pub fn user(cfg: &mut ServiceConfig) {
    cfg
        .service(sign_up)
        .service(login)
        .service(get_user)
        .service(get_all)
    ;
}

