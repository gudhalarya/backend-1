use std::env;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use sqlx::{PgPool, postgres::PgPoolOptions};

//fn to connect to the database ->Error is not hadled properly right now.
async fn get_db()->PgPool{
    let db_url  = env::var("DATABASE_URL").expect("Could not find the database url in the env file ");
    let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect(&db_url)
    .await.expect("Could not connect to the database");

    return pool;
}

#[get("/index")]
async fn index()->impl Responder{
    HttpResponse::Ok().body("This is the main index route of the backend ")
}

//This is the where the main fn lies 
#[actix_web::main]
async fn main ()->std::io::Result<()>{
    let pool = get_db().await;
    HttpServer::new(move||{
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(index)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}