use actix_web::{HttpResponse, Responder, get, post, web};
use sqlx::{PgPool};

use crate::{utils::errors::CustomErrors, models::chat_models::{CreateRoomReq, Message, Room}};

#[post("/rooms/create")]
async fn create_rooms(pool:web::Data<PgPool>,room_name:web::Json<CreateRoomReq>)->Result<impl Responder,CustomErrors>{
    if room_name.name.trim().is_empty(){
        return Err(CustomErrors::BadRequest("Room name cant be empty ".into()));
    }
    let query = r#"INSERT INTO rooms(name)
    VALUES ($1)
    RETURNING (id,name,created_at)
    "#;
    let room :Room = sqlx::query_as(query).bind(&room_name.name).fetch_one(pool.get_ref()).await.map_err(|e|CustomErrors::DatabaseError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(room))
}

#[get("/rooms/{id}/messages")]
async fn room_messages(pool:web::Data<PgPool>,id:web::Path<i32>)->Result<impl Responder,CustomErrors>{
    let room_id = id.into_inner();
    let query = r#"SELECT id,room_id,sender_id,content,timestamp
    FROM messages
    WHERE room_id = $1
    ORDER BY timestamp ASC"#;
    let messages :Vec<Message> = sqlx::query_as(query).bind(room_id).fetch_all(pool.get_ref()).await.map_err(|e|CustomErrors::DatabaseError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(messages))
}
