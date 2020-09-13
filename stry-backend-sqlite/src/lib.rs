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
    crate::utils::SqliteConnectionManager,
    r2d2::Pool,
    std::sync::Arc,
    stry_common::{
        backend::{BackendType, StorageType},
        version::LibVersion,
    },
};

pub const SCHEMA: &str = rewryte::schema!("sqlite", "../schema.dal");

#[cfg(test)]
pub const TEST_DATA: &str = include_str!("test-data.sql");

#[derive(Clone, Debug)]
pub struct SqliteBackend(Pool<SqliteConnectionManager>);

impl SqliteBackend {
    #[tracing::instrument(skip(_backend, storage, _version), err)]
    pub async fn init(
        _backend: BackendType,
        storage: StorageType,
        _version: Arc<Vec<LibVersion>>,
    ) -> anyhow::Result<Self> {
        if let StorageType::File { location } = storage {
            let pool = tokio::task::spawn_blocking(
                move || -> anyhow::Result<Pool<SqliteConnectionManager>> {
                    let manager = SqliteConnectionManager::file(&location)
                        .with_init(|c| c.execute_batch("PRAGMA foreign_keys=1;"));

                    let pool = Pool::new(manager)?;

                    let conn = pool.get()?;

                    conn.execute_batch(SCHEMA)?;

                    Ok(pool)
                },
            )
            .await??;

            Ok(Self(pool))
        } else {
            anyhow::bail!("The `SQLite` backend can only use the `File` storage type");
        }
    }
}

#[cfg(test)]
pub mod test_utils {
    use {
        crate::{utils::SqliteConnectionManager, SqliteBackend, SCHEMA, TEST_DATA},
        r2d2::Pool,
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
