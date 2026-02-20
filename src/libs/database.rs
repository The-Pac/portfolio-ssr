use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};
use std::env;

static DATABASE_POOL: std::sync::OnceLock<SqlitePool> = std::sync::OnceLock::new();
async fn establish_connection() -> SqlitePool {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data.db".to_string());

    if !database_url.contains(":memory:") {
        let db_path = "data.db";
        if !tokio::fs::try_exists(db_path).await.unwrap_or(false) {
            println!("Creating database file at: {}", db_path);
            Sqlite::create_database(&database_url)
                .await
                .expect("Failed to create database file");
        }
    }

    let pool = SqlitePool::connect_lazy(&database_url).expect("Failed to create database pool");

    sqlx::query("PRAGMA journal_mode = WAL").execute(&pool).await.expect("Failed to set journal_mode");
    sqlx::query("PRAGMA synchronous = NORMAL").execute(&pool).await.expect("Failed to set synchronous");
    sqlx::query("PRAGMA foreign_keys = ON").execute(&pool).await.expect("Failed to set foreign_keys");

    println!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migration failed");

    println!("Connected to database!");
    pool
}

pub async fn init_database() -> Result<(), sqlx::Pool<Sqlite>> {
    DATABASE_POOL.set(establish_connection().await)
}

pub fn get_database<'a>() -> &'a SqlitePool {
    DATABASE_POOL.get().expect("database unitialized")
}
