use std::sync::Arc;
pub use sqlx::SqlitePool;

pub type AppDb = Arc<SqlitePool>;
