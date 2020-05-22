pub mod author;
pub mod chapter;
pub mod origin;
pub mod story;
pub mod tag;

use {
    bb8::Pool, bb8_postgres::PostgresConnectionManager, stry_common::Backend, tokio_postgres::NoTls,
};

#[derive(Clone, Debug)]
pub struct PostgresBackend(Pool<PostgresConnectionManager<NoTls>>);

#[async_trait::async_trait]
impl Backend for PostgresBackend {}
