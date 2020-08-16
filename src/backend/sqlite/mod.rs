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

#[doc(hidden)]
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

pub const SCHEMA: &str = rewryte::schema!("sqlite", "src/backend/schema.dal");

#[cfg(test)]
pub const TEST_DATA: &str = include_str!("test-data.sql");

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

#[cfg(test)]
pub mod test_utils {
    use {
        super::{SqliteBackend, SCHEMA, TEST_DATA},
        r2d2::Pool,
        r2d2_sqlite::SqliteConnectionManager,
    };

    pub fn setup() -> anyhow::Result<SqliteBackend> {
        let manager = SqliteConnectionManager::memory().with_init(|c| {
            c.execute_batch("PRAGMA foreign_keys=1;")?;
            c.execute_batch(SCHEMA)?;
            c.execute_batch(TEST_DATA)?;
            Ok(())
        });

        let pool = Pool::new(manager)?;

        Ok(SqliteBackend(pool))
    }
}

pub fn version() -> Vec<LibVersion> {
    vec![LibVersion::SQLite {
        version: rusqlite::version(),
    }]
}
