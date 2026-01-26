use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum StError {
    #[error("Database Error: {0}")]
    Sql(#[from] rusqlite::Error),

    #[error("Thread Pool Error: {0}")]
    Task(#[from] tokio::task::JoinError),

    #[error("Connection Pool Error: {0}")]
    Pool(#[from] r2d2::Error),

    #[error("Authentication Error: {0}")]
    Auth(#[from] argon2::password_hash::Error),

    #[error("Db Error: Could not find {0} with id {1}")]
    DbNotFound(String, Uuid),

    #[error("Db Error: Found {0} {1} with id {2}")]
    DbNotUnique(usize, String, Uuid),
}
