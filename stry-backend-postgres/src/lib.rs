// TODO: try to pipeline queries

pub mod author;
pub mod chapter;
pub mod character;
pub mod origin;
pub mod pairing;
pub mod story;
pub mod tag;
pub mod warning;
pub mod worker;

#[macro_use]
pub mod utils;

use {
    bb8::Pool,
    bb8_postgres::PostgresConnectionManager,
    std::sync::Arc,
    stry_common::{
        backend::{Backend, BackendType, StorageType},
        LibVersion,
    },
    tokio_postgres::NoTls,
};

#[derive(Clone, Debug)]
pub struct PostgresBackend(Pool<PostgresConnectionManager<NoTls>>);

#[async_trait::async_trait]
impl Backend for PostgresBackend {
    async fn init(
        backend: BackendType,
        storage: StorageType,
        _: Arc<Vec<LibVersion>>,
    ) -> anyhow::Result<Self> {
        todo!()
    }
}

pub fn version() -> Vec<stry_common::LibVersion> {
    vec![]
}
