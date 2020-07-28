use {
    anyhow::Context,
    rusqlite::{Row, Rows, ToSql},
    std::marker::PhantomData,
};

pub trait FromRow {
    fn from_row(row: &Row<'_>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub struct Total {
    pub total: i32,
}

impl FromRow for Total {
    fn from_row(row: &Row<'_>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            total: row
                .get(0)
                .context("Attempting to get row index 0 for row count")?,
        })
    }
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
    pub(crate) fn new(rows: Rows<'stmt>, f: F) -> Self {
        Self { rows, map: f }
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
pub struct TypeMappedRows<'stmt, T> {
    rows: Rows<'stmt>,
    typ: PhantomData<T>,
}

impl<'stmt, T> TypeMappedRows<'stmt, T>
where
    T: FromRow,
{
    pub(crate) fn new(rows: Rows<'stmt>) -> Self {
        Self {
            rows,
            typ: PhantomData::default(),
        }
    }
}

impl<T> Iterator for TypeMappedRows<'_, T>
where
    T: FromRow,
{
    type Item = anyhow::Result<T>;

    fn next(&mut self) -> Option<anyhow::Result<T>> {
        self.rows
            .next()
            .map_err(anyhow::Error::from)
            .transpose()
            .map(|row_result| {
                row_result
                    .and_then(|row| T::from_row(&row))
                    .map_err(anyhow::Error::from)
            })
    }
}

pub trait SqliteExt {
    fn query_row_anyhow<T, P, F>(&self, sql: &str, params: P, f: F) -> anyhow::Result<Option<T>>
    where
        P: IntoIterator,
        P::Item: ToSql,
        F: FnOnce(&Row<'_>) -> anyhow::Result<T>;

    fn type_query_row_anyhow<T, P>(&self, sql: &str, params: P) -> anyhow::Result<Option<T>>
    where
        P: IntoIterator,
        P::Item: ToSql,
        T: FromRow;
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

    fn type_query_row_anyhow<T, P>(&self, sql: &str, params: P) -> anyhow::Result<Option<T>>
    where
        P: IntoIterator,
        P::Item: ToSql,
        T: FromRow,
    {
        let mut stmt = self.prepare(sql)?;

        match stmt.type_query_row_anyhow(params) {
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

    fn type_query_row_anyhow<T, P>(&mut self, params: P) -> anyhow::Result<Option<T>>
    where
        P: IntoIterator,
        P::Item: ToSql,
        T: FromRow;

    fn query_map_anyhow<T, P, F>(
        &mut self,
        params: P,
        f: F,
    ) -> anyhow::Result<Option<MappedRows<'_, F>>>
    where
        P: IntoIterator,
        P::Item: ToSql,
        F: FnMut(&Row<'_>) -> anyhow::Result<T>;

    fn type_query_map_anyhow<T, P>(
        &mut self,
        params: P,
    ) -> anyhow::Result<Option<TypeMappedRows<'_, T>>>
    where
        P: IntoIterator,
        P::Item: ToSql,
        T: FromRow;
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

    fn type_query_row_anyhow<T, P>(&mut self, params: P) -> anyhow::Result<Option<T>>
    where
        P: IntoIterator,
        P::Item: ToSql,
        T: FromRow,
    {
        let mut rows = match self.query(params).map_err(anyhow::Error::from) {
            Ok(rows) => rows,
            Err(err) => match err.downcast_ref::<rusqlite::Error>() {
                Some(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
                _ => return Err(err),
            },
        };

        let res: Option<T> = match rows.next()? {
            Some(row) => Some(T::from_row(&row)?),
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

    fn type_query_map_anyhow<T, P>(
        &mut self,
        params: P,
    ) -> anyhow::Result<Option<TypeMappedRows<'_, T>>>
    where
        P: IntoIterator,
        P::Item: ToSql,
        T: FromRow,
    {
        let rows = match self.query(params).map_err(anyhow::Error::from) {
            Ok(rows) => rows,
            Err(err) => match err.downcast_ref::<rusqlite::Error>() {
                Some(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
                _ => return Err(err),
            },
        };

        Ok(Some(TypeMappedRows::new(rows)))
    }
}
