
mod router;
mod handlers;

use router::root;
use actix_web::{App, HttpServer, middleware::Logger};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().ok();              // load .env   
    env_logger::init();         // init logger

    HttpServer::new(|| {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .configure(root)    //create router pls check router directory
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}