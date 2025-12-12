use serde::{Deserialize,Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug,Deserialize)]
pub struct CreateRoomReq{
    pub name:String,
}

#[derive(Serialize,FromRow)]
pub struct Message{
   pub id:i32,
   pub room_id:i32,
   pub sender_id:i32,
   pub content:String,
   pub timestamp:chrono::NaiveDateTime,
}

#[derive(Debug,Serialize,FromRow)]
pub struct Room{
   pub id:i32,
   pub name:String,
   pub created_at:chrono::NaiveDateTime,
}