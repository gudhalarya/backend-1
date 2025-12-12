use actix_web::{HttpResponse, Responder, get, post, web};
use sqlx::PgPool;

use crate::{utils::errors::CustomErrors, models::{UserAdd, UserAddReq}, utils::{hashing_password, verify_password}};

#[get("/index")]
async fn index()->impl Responder{
    HttpResponse::Ok().body("This is the main body page")
}

#[post("/register")]
async fn register(pool:web::Data<PgPool>,user:web::Json<UserAdd>)->Result<impl Responder,CustomErrors>{
    let hashed_password = hashing_password(user.password.clone());
    if user.username.trim().is_empty()||user.password.trim().is_empty(){
        return Err(CustomErrors::BadRequest("Username or password could not be empty".to_string(),));
    }
    let rows :Option<UserAddReq>  = sqlx::query_as("SELECT * FROM users WHERE username = $1").bind(&user.username).fetch_optional(pool.get_ref()).await.map_err(|e|CustomErrors::DatabaseError(e.to_string()))?;
    if rows.is_some(){
      return  Err(CustomErrors::BadRequest("User already exist".to_string(),));
    }
    sqlx::query("INSERT INTO users(username,password) VALUES ($1,$2)").bind(&user.username).bind(&hashed_password).execute(pool.get_ref()).await.map_err(|e|CustomErrors::DatabaseError(e.to_string()))?;

    Ok(HttpResponse::Ok().body("user added successfully"))
}

#[post("/login")]
async fn login(
    pool: web::Data<PgPool>,
    user: web::Json<UserAdd>
) -> Result<impl Responder, CustomErrors> {

    if user.username.trim().is_empty() || user.password.trim().is_empty() {
        return Err(CustomErrors::BadRequest(
            "Username or password cannot be empty".to_string()
        ));
    }

    let db_user: Option<UserAddReq> = sqlx::query_as::<_, UserAddReq>(
        "SELECT * FROM users WHERE username = $1"
    )
    .bind(&user.username)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| CustomErrors::DatabaseError(e.to_string()))?;

    if db_user.is_none() {
        return Err(CustomErrors::BadRequest(
            "User does not exist, please register first".to_string()
        ));
    }

    let db_user = db_user.unwrap();

    if !verify_password(user.password.clone(), db_user.password.clone()) {
        return Err(CustomErrors::BadRequest(
            "Incorrect password".to_string()
        ));
    }

    Ok(HttpResponse::Ok().body("Login successful"))
}
