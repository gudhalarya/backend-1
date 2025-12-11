use actix_web::{HttpResponse, Responder, get, web};
use sqlx::PgPool;

#[get("/index")]
async fn index()->impl Responder{
    HttpResponse::Ok().body("This is the main body ")
}
