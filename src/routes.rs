use actix_web::{HttpResponse, Responder, get, web};
use sqlx::PgPool;

use crate::{errors::CustomErrors, models::UserAddReq};

#[get("/index")]
async fn index()->impl Responder{
    HttpResponse::Ok().body("This is the main body page")
}

#[get("/get_users")]
async fn get_users(pool:web::Data<PgPool>)->Result<impl Responder,CustomErrors>{
    let rows :Vec<UserAddReq> = sqlx::query_as("SELECT * FROM users ").fetch_all(pool.get_ref()).await.map_err(|e|CustomErrors::DatabaseError(e.to_string()))?;
    if rows.is_empty(){
        return Err(CustomErrors::UserNotExist);
    }
    Ok(HttpResponse::Ok().json(rows))
}

