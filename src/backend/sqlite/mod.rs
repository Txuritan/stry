mod author;
mod chapter;
mod character;
mod origin;
mod pairing;
mod story;
mod tag;
mod warning;
mod worker;

mod utils;

use {
    crate::{
        backend::{Backend, BackendType, StorageType},
        version::LibVersion,
    },
    r2d2::Pool,
    r2d2_sqlite::SqliteConnectionManager,
    std::sync::Arc,
};

#[derive(Clone, Debug)]
pub struct SqliteBackend(Pool<SqliteConnectionManager>);

#[async_trait::async_trait]
impl Backend for SqliteBackend {
    #[tracing::instrument(skip(_backend, storage, _version))]
    async fn init(
        _backend: BackendType,
        storage: StorageType,
        _version: Arc<Vec<LibVersion>>,
    ) -> anyhow::Result<Self> {
        if let StorageType::File { location } = storage {
            let manager = SqliteConnectionManager::file(&location)
                .with_init(|c| c.execute_batch("PRAGMA foreign_keys=1;"));

            let pool = Pool::new(manager)?;

            let conn = pool.get()?;

            conn.execute_batch(include_str!("schema.sql"))?;

            Ok(Self(pool))
        } else {
            anyhow::bail!("The `SQLite` backend can only use the `File` storage type");
        }
    }
}

#[doc(hidden)]
pub mod test_utils {
    use {super::SqliteBackend, r2d2::Pool, r2d2_sqlite::SqliteConnectionManager};

    pub fn setup(schema: &str, data: &str) -> anyhow::Result<SqliteBackend> {
        let manager = SqliteConnectionManager::memory()
            .with_init(|c| c.execute_batch("PRAGMA foreign_keys=1;"));

        let pool = Pool::new(manager)?;

        let conn = pool.get()?;

        conn.execute_batch(schema)?;

        conn.execute_batch(data)?;

        Ok(SqliteBackend(pool))
    }
}

pub fn version() -> Vec<LibVersion> {
    vec![LibVersion::SQLite {
        version: rusqlite::version(),
    }]
}
