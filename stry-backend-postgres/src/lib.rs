// TODO: try to pipeline queries

mod utils;

mod author;
mod chapter;
mod character;
mod origin;
mod pairing;
mod story;
mod tag;
mod warning;
mod worker;

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

#[cfg_attr(feature = "boxed-futures", stry_macros::box_async)]
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
