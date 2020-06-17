pub mod author;
pub mod chapter;
pub mod character;
pub mod origin;
pub mod pairing;
pub mod story;
pub mod tag;
pub mod warning;

use {
    r2d2::Pool,
    r2d2_sqlite::SqliteConnectionManager,
    stry_common::backend::{Backend, BackendType, StorageType},
};

#[derive(Clone, Debug)]
pub struct SqliteBackend(Pool<SqliteConnectionManager>);

#[async_trait::async_trait]
impl Backend for SqliteBackend {
    async fn init(_backend: BackendType, storage: StorageType) -> anyhow::Result<Self> {
        if let StorageType::File { location } = storage {
            let manager = SqliteConnectionManager::file(&location)
                .with_init(|c| c.execute_batch("PRAGMA foreign_keys=1;"));

            let pool = Pool::new(manager)?;

            Ok(Self(pool))
        } else {
            anyhow::bail!("The `SQLite` backend can only use the `File` storage type");
        }
    }
}

pub mod utils {
    use rusqlite::{Row, Rows, ToSql};

    pub trait FromRow {
        fn from_row(row: &Row<'_>) -> anyhow::Result<Self>
        where
            Self: Sized;
    }

    // This is a horrible way to add anyhow to rusqlite but hey
    pub struct MappedRows<'stmt, F> {
        rows: Rows<'stmt>,
        map: F,
    }

    impl<'stmt, T, F> MappedRows<'stmt, F>
    where
        F: FnMut(&Row<'_>) -> anyhow::Result<T>,
    {
        pub(crate) fn new(rows: Rows<'stmt>, f: F) -> MappedRows<'stmt, F> {
            MappedRows { rows, map: f }
        }
    }

    impl<T, F> Iterator for MappedRows<'_, F>
    where
        F: FnMut(&Row<'_>) -> anyhow::Result<T>,
    {
        type Item = anyhow::Result<T>;

        fn next(&mut self) -> Option<anyhow::Result<T>> {
            let map = &mut self.map;

            self.rows
                .next()
                .map_err(anyhow::Error::from)
                .transpose()
                .map(|row_result| {
                    row_result
                        .and_then(|row| (map)(&row))
                        .map_err(anyhow::Error::from)
                })
        }
    }

    pub trait SqliteExt {
        fn query_row_anyhow<T, P, F>(
            &self,
            sql: &str,
            params: P,
            f: F,
        ) -> anyhow::Result<Option<T>>
        where
            P: IntoIterator,
            P::Item: ToSql,
            F: FnOnce(&Row<'_>) -> anyhow::Result<T>;
    }

    impl SqliteExt for rusqlite::Connection {
        fn query_row_anyhow<T, P, F>(&self, sql: &str, params: P, f: F) -> anyhow::Result<Option<T>>
        where
            P: IntoIterator,
            P::Item: ToSql,
            F: FnOnce(&Row<'_>) -> anyhow::Result<T>,
        {
            let mut stmt = self.prepare(sql)?;

            match stmt.query_row_anyhow(params, f) {
                Ok(res) => Ok(res),
                Err(err) => match err.downcast_ref::<rusqlite::Error>() {
                    Some(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                    _ => Err(err),
                },
            }
        }
    }

    pub trait SqliteStmtExt {
        fn query_row_anyhow<T, P, F>(&mut self, params: P, f: F) -> anyhow::Result<Option<T>>
        where
            P: IntoIterator,
            P::Item: ToSql,
            F: FnOnce(&Row<'_>) -> anyhow::Result<T>;

        fn query_map_anyhow<T, P, F>(
            &mut self,
            params: P,
            f: F,
        ) -> anyhow::Result<Option<MappedRows<'_, F>>>
        where
            P: IntoIterator,
            P::Item: ToSql,
            F: FnMut(&Row<'_>) -> anyhow::Result<T>;
    }

    impl SqliteStmtExt for rusqlite::Statement<'_> {
        fn query_row_anyhow<T, P, F>(&mut self, params: P, f: F) -> anyhow::Result<Option<T>>
        where
            P: IntoIterator,
            P::Item: ToSql,
            F: FnOnce(&Row<'_>) -> anyhow::Result<T>,
        {
            let mut rows = match self.query(params).map_err(anyhow::Error::from) {
                Ok(rows) => rows,
                Err(err) => match err.downcast_ref::<rusqlite::Error>() {
                    Some(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
                    _ => return Err(err),
                },
            };

            let res: Option<T> = match rows.next()? {
                Some(row) => Some(f(&row)?),
                None => None,
            };

            Ok(res)
        }

        fn query_map_anyhow<T, P, F>(
            &mut self,
            params: P,
            f: F,
        ) -> anyhow::Result<Option<MappedRows<'_, F>>>
        where
            P: IntoIterator,
            P::Item: ToSql,
            F: FnMut(&Row<'_>) -> anyhow::Result<T>,
        {
            let rows = match self.query(params).map_err(anyhow::Error::from) {
                Ok(rows) => rows,
                Err(err) => match err.downcast_ref::<rusqlite::Error>() {
                    Some(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
                    _ => return Err(err),
                },
            };

            Ok(Some(MappedRows::new(rows, f)))
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
