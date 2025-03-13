#[cfg(feature = "ssr")]
pub mod ssr {
    use sqlx::{Connection, Error, SqliteConnection};

    pub async fn db() -> Result<SqliteConnection, Error> {
        Ok(SqliteConnection::connect("sqlite:career.db").await?)
    }
}