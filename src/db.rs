use std::{env, time::Duration};

use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::utils::errors::CustomErrors;

pub async fn get_db()->Result<PgPool,CustomErrors>{
    let url = env::var("DATABASE_URL").map_err(|_|CustomErrors::DbUrlNotFound)?;
    let pool = PgPoolOptions::new()
    .max_connections(10)
    .acquire_timeout(Duration::from_secs(10))
    .connect(&url)
    .await.map_err(|_|CustomErrors::DbConnectionError)?;

    Ok(pool)
}