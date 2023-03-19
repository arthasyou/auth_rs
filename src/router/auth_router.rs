use actix_web::web::ServiceConfig;
use crate::handlers::auth_handler::*;


pub fn auth(cfg: &mut ServiceConfig) {
    
    cfg.service(test)        
    ;
}

