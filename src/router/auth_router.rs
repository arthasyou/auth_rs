use actix_web::web::ServiceConfig;
use crate::handlers::auth::*;


pub fn auth(cfg: &mut ServiceConfig) {
    
    cfg.service(test)  
    ;
}

