mod author;
mod chapter;
mod character;
mod origin;
mod pairing;
mod story;
mod tag;
mod warning;
mod worker;

mod sync;
mod utils;

use {
    r2d2::Pool,
    r2d2_sqlite::SqliteConnectionManager,
    std::sync::Arc,
    stry_common::{
        backend::{Backend, BackendType, StorageType},
        LibVersion,
    },
};

#[derive(Clone, Debug)]
pub struct SqliteBackend(Pool<SqliteConnectionManager>);

#[async_trait::async_trait]
impl Backend for SqliteBackend {
    async fn init(
        _backend: BackendType,
        storage: StorageType,
        _: Arc<Vec<LibVersion>>,
    ) -> anyhow::Result<Self> {
        if let StorageType::File { location } = storage {
            let manager = SqliteConnectionManager::file(&location)
                .with_init(|c| c.execute_batch("PRAGMA foreign_keys=1;"));

            let pool = Pool::new(manager)?;

            let conn = pool.get()?;

            conn.execute_batch(include_str!("../schema.sql"))?;

            Ok(Self(pool))
        } else {
            anyhow::bail!("The `SQLite` backend can only use the `File` storage type");
        }
    }
}

#[doc(hidden)]
pub mod test_utils {
    use {crate::SqliteBackend, r2d2::Pool, r2d2_sqlite::SqliteConnectionManager};

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
