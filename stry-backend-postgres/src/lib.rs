// TODO: try to pipeline queries

#[doc(hidden)]
mod author;
#[doc(hidden)]
mod chapter;
#[doc(hidden)]
mod character;
#[doc(hidden)]
mod origin;
#[doc(hidden)]
mod pairing;
#[doc(hidden)]
mod story;
#[doc(hidden)]
mod tag;
#[doc(hidden)]
mod warning;
#[doc(hidden)]
mod worker;

#[macro_use]
pub mod utils;

use {
    bb8::Pool,
    bb8_postgres::PostgresConnectionManager,
    std::sync::Arc,
    stry_common::{
        backend::{Backend, BackendType, StorageType},
        version::LibVersion,
    },
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
