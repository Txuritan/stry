// TODO: try to pipeline queries

pub mod author;
pub mod chapter;
pub mod character;
pub mod origin;
pub mod pairing;
pub mod story;
pub mod tag;
pub mod warning;

#[macro_use]
pub mod utils;

use {
    bb8::Pool,
    bb8_postgres::PostgresConnectionManager,
    stry_common::backend::{Backend, BackendType, StorageType},
    tokio_postgres::NoTls,
};

#[derive(Clone, Debug)]
pub struct PostgresBackend(Pool<PostgresConnectionManager<NoTls>>);

#[async_trait::async_trait]
impl Backend for PostgresBackend {
    async fn init(backend: BackendType, storage: StorageType) -> anyhow::Result<Self> {
        todo!()
    }
}
