mod db;
mod errors;
mod routes;
mod models;
mod utils;

use actix_web::{App, HttpServer, web};
use crate::routes::{login, register};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = match db::get_db().await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to connect to database: {:?}", e); 
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(register)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
