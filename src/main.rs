
mod router;
mod handlers;
mod models;
mod db;
mod settings;
mod error;
mod service;

use log::debug;
use router::root;
use settings::Settings;
use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use actix_cors::Cors;
use dotenv::dotenv;
use crate::service::jwt;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().ok();              // load .env   
    env_logger::init();         // init logger

    let settings = Settings::new().unwrap();
    // debug!("{:?}", settings);

    jwt::init(settings.jwt);    

    let db = crate::db::mongodb::MongoDB::init(settings.mongodb).await.unwrap();

    HttpServer::new(move || {
        let logger = Logger::default();
        let cors = Cors::permissive();

        // let cors = Cors::default()            
        //     .allow_any_origin()
        //     .allowed_methods(vec!["GET", "POST"])
        //     .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        //     .allowed_header(header::CONTENT_TYPE)
        //     .max_age(3600);  

        App::new()
            .wrap(logger)
            .wrap(cors)
            .app_data(Data::new(db.clone()))
            .configure(root)    //create router pls check router directory
    })
    .bind((settings.server.host, settings.server.port))?
    .run()
    .await
}