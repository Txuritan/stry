pub mod author;
pub mod chapter;
pub mod origin;
pub mod story;
pub mod tag;

use {
    bb8::Pool,
    bb8_postgres::PostgresConnectionManager,
    stry_common::{Backend, BackendConnection},
    tokio_postgres::NoTls,
};

pub(crate) type PostgresPooledConnection<'l> =
    bb8::PooledConnection<'l, PostgresConnectionManager<NoTls>>;

#[derive(Clone, Debug)]
pub struct PostgresBackend(Pool<PostgresConnectionManager<NoTls>>);

#[async_trait::async_trait]
impl Backend for PostgresBackend {
    type Connection = PostgresPoolConnection;

    async fn conn(&self) -> anyhow::Result<Self::Connection> {
        Ok(PostgresPoolConnection(self.0.clone()))
    }
}

#[derive(Clone)]
pub struct PostgresPoolConnection(Pool<PostgresConnectionManager<NoTls>>);

#[async_trait::async_trait]
impl BackendConnection for PostgresPoolConnection {}
