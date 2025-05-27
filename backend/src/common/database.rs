use sqlx::{Pool, Sqlite, SqlitePool};
use tokio::sync::OnceCell;

use super::config::CONFIG;

static POOL: OnceCell<SqlitePool> = OnceCell::const_new();

pub async fn init() {
    let database_url = CONFIG.database.url.clone();
    let pool = SqlitePool::connect_lazy(&database_url).expect("database init error");

    assert!(POOL.set(pool).is_ok(), "database init error")
}

pub fn database_connect() -> &'static Pool<Sqlite> {
    POOL.get().expect("database connect get error")
}
