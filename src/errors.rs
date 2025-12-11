use thiserror::Error;

#[derive(Debug,Error)]
pub enum CustomErrors {
    #[error("When the url is not found")]
    DbUrlNotFound,
    #[error("When connection is not set")]
    DbConnectionError,
}