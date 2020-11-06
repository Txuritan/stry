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
    stry_common::LibraryDetails,
    stry_config::{BackendType, StorageType},
};

pub const SCHEMA: &str = rewryte::schema!("sqlite", "../schema.dal");

#[cfg(test)]
pub const TEST_DATA: &str = include_str!("test-data.sql");

#[derive(Clone, Debug)]
pub struct SqliteBackend(Pool<SqliteConnectionManager>);

impl SqliteBackend {
    #[tracing::instrument(skip(_backend, storage, _details), err)]
    pub async fn init(
        _backend: BackendType,
        storage: StorageType,
        _details: Arc<Vec<LibraryDetails>>,
    ) -> anyhow::Result<Self> {
        if let StorageType::File { location } = storage {
            let pool = tokio::task::spawn_blocking(
                move || -> anyhow::Result<Pool<SqliteConnectionManager>> {
                    let manager = SqliteConnectionManager::file(&location).with_init(|conn| {
                        conn.execute_batch("PRAGMA foreign_keys=1;")?;

                        utils::add_compression_functions(conn)?;

                        Ok(())
                    });

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
        crate::{
            utils::{self, SqliteConnectionManager},
            SqliteBackend, SCHEMA, TEST_DATA,
        },
        r2d2::Pool,
    };

    pub fn setup() -> anyhow::Result<SqliteBackend> {
        let manager = SqliteConnectionManager::memory().with_init(|conn| {
            conn.execute_batch("PRAGMA foreign_keys=1;")?;

            conn.execute_batch(SCHEMA)?;
            conn.execute_batch(TEST_DATA)?;

            utils::add_compression_functions(conn)?;

            Ok(())
        });

        let pool = Pool::new(manager)?;

        Ok(SqliteBackend(pool))
    }
}

pub fn library_details() -> Vec<LibraryDetails> {
    vec![LibraryDetails::SQLite {
        version: rusqlite::version(),
    }]
}
