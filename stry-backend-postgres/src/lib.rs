// TODO: try to pipeline queries

mod author;
mod chapter;
mod character;
mod origin;
mod pairing;
mod story;
mod tag;
mod warning;
mod worker;

#[macro_use]
pub mod utils;

use {
    bb8::Pool,
    bb8_postgres::PostgresConnectionManager,
    std::sync::Arc,
    stry_common::LibraryDetails,
    stry_config::{BackendType, StorageType},
    tokio_postgres::NoTls,
};

pub const SCHEMA: &str = rewryte::schema!("postgresql", "../schema.dal");

#[derive(Clone, Debug)]
pub struct PostgresBackend(Pool<PostgresConnectionManager<NoTls>>);

#[stry_macros::box_async]
impl PostgresBackend {
    #[tracing::instrument(skip(_backend, _storage, _details), err)]
    pub async fn init(
        _backend: BackendType,
        _storage: StorageType,
        _details: Arc<Vec<LibraryDetails>>,
    ) -> anyhow::Result<Self> {
        todo!()
    }
}

pub fn library_details() -> Vec<LibraryDetails> {
    vec![]
}
