pub mod author;
pub mod chapter;
pub mod origin;
pub mod story;
pub mod tag;

use {r2d2::Pool, r2d2_sqlite::SqliteConnectionManager, stry_common::Backend};

#[derive(Clone, Debug)]
pub struct SqliteBackend(Pool<SqliteConnectionManager>);

impl SqliteBackend {
    pub fn new() -> anyhow::Result<Self> {
        let manager = SqliteConnectionManager::file("stry.db");

        let pool = Pool::new(manager)?;

        Ok(Self(pool))
    }
}

#[async_trait::async_trait]
impl Backend for SqliteBackend {}
