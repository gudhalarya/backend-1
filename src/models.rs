use serde::{Deserialize};
use sqlx::FromRow;

#[derive(Debug,Deserialize)]
pub struct UserAdd{
    pub username:String,
    pub password:String,
}

#[derive(Debug,FromRow)]
pub struct UserAddReq{
    pub username:String,
    pub id:i32,
    pub password:String,
}
