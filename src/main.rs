mod db;
mod errors;
mod routes;
mod models;
mod utils;

use actix_web::{App, HttpServer, web};
use crate::{db::get_db, routes::index};

#[actix_web::main]
async fn main()->std::io::Result<()>{
    let pool= get_db().await.expect("Could not find the url");
    HttpServer::new(move||{
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(index)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}