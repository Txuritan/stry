pub mod author;
pub mod chapter;
pub mod origin;
pub mod story;
pub mod tag;

use {
    sqlx::{pool::PoolConnection, SqliteConnection, SqlitePool},
    stry_common::{Backend, BackendConnection},
};

#[derive(Clone, Debug)]
pub struct SqliteBackend(SqlitePool);

#[async_trait::async_trait]
impl Backend for SqliteBackend {
    type Connection = SqlitePoolConnection;

    async fn conn(&self) -> anyhow::Result<Self::Connection> {
        let conn = self.0.acquire().await?;

        Ok(SqlitePoolConnection(conn))
    }
}

pub struct SqlitePoolConnection(PoolConnection<SqliteConnection>);

#[async_trait::async_trait]
impl BackendConnection for SqlitePoolConnection {}
