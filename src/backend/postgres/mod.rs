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
    crate::{
        backend::{Backend, BackendType, StorageType},
        version::LibVersion,
    },
    bb8::Pool,
    bb8_postgres::PostgresConnectionManager,
    std::sync::Arc,
    tokio_postgres::NoTls,
};

#[derive(Clone, Debug)]
pub struct PostgresBackend(Pool<PostgresConnectionManager<NoTls>>);

#[async_trait::async_trait]
impl Backend for PostgresBackend {
    #[tracing::instrument(skip(_backend, _storage, _version))]
    async fn init(
        _backend: BackendType,
        _storage: StorageType,
        _version: Arc<Vec<LibVersion>>,
    ) -> anyhow::Result<Self> {
        todo!()
    }
}

pub fn version() -> Vec<LibVersion> {
    vec![]
}
