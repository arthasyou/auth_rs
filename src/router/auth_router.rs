use crate::handlers::auth::*;
use actix_web::web::ServiceConfig;

pub fn auth(cfg: &mut ServiceConfig) {
    cfg.service(sign_up)
        .service(login)
        .service(get_user)
        .service(get_all);
}
