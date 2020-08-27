use {
    futures::{pin_mut, TryStreamExt},
    tokio_postgres::{types::ToSql, Client, Row, ToStatement},
};

#[macro_export]
macro_rules! opt_try {
    ($opt:ident) => {
        match $opt {
            Some(item) => item,
            None => return Ok(None),
        }
    };
}

#[macro_export]
macro_rules! params {
    () => {
        &[] as &[&(dyn tokio_postgres::types::ToSql + Sync)]
    };
    ($( $param:expr ),+ $(,)?) => {
        &[$(&$param as &(dyn tokio_postgres::types::ToSql + Sync)),+] as &[&(dyn tokio_postgres::types::ToSql + Sync)]
    };
}

pub trait FromRow {
    fn from_row(row: &Row) -> anyhow::Result<Self>
    where
        Self: Sized;
}

impl FromRow for i32 {
    fn from_row(row: &Row) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let num = row.try_get(0)?;

        Ok(num)
    }
}

impl FromRow for stry_common::models::Entity {
    fn from_row(row: &Row) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let entity = stry_common::models::Entity {
            id: row.try_get(0)?,
        };

        Ok(entity)
    }
}

fn slice_iter<'a>(
    s: &'a [&'a (dyn ToSql + Sync)],
) -> impl ExactSizeIterator<Item = &'a dyn ToSql> + 'a {
    s.iter().map(|s| *s as _)
}

#[async_trait::async_trait]
pub trait ClientExt {
    async fn type_query_row_anyhow<'a, T, S>(
        &self,
        statement: &S,
        params: &[&(dyn ToSql + Sync)],
    ) -> anyhow::Result<Option<T>>
    where
        S: ?Sized + ToStatement + Send + Sync,
        T: FromRow;

    async fn type_query_map_anyhow<'a, T, S>(
        &self,
        statement: &S,
        params: &[&(dyn ToSql + Sync)],
    ) -> anyhow::Result<Option<Vec<T>>>
    where
        S: ?Sized + ToStatement + Send + Sync,
        T: FromRow;
}

#[async_trait::async_trait]
impl ClientExt for Client {
    async fn type_query_row_anyhow<'a, T, S>(
        &self,
        statement: &S,
        params: &[&(dyn ToSql + Sync)],
    ) -> anyhow::Result<Option<T>>
    where
        S: ?Sized + ToStatement + Send + Sync,
        T: FromRow,
    {
        let stream = self.query_raw(statement, slice_iter(params)).await?;
        pin_mut!(stream);

        let row = match stream.try_next().await? {
            Some(row) => row,
            None => return Ok(None),
        };

        if stream.try_next().await?.is_some() {
            anyhow::bail!("query returned an unexpected number of rows");
        }

        let mapped = T::from_row(&row)?;

        Ok(Some(mapped))
    }

    // TODO: figure out how to check for no returned rows
    async fn type_query_map_anyhow<'a, T, S>(
        &self,
        statement: &S,
        params: &[&(dyn ToSql + Sync)],
    ) -> anyhow::Result<Option<Vec<T>>>
    where
        S: ?Sized + ToStatement + Send + Sync,
        T: FromRow,
    {
        let rows = self.query(statement, params).await?;

        let mut items = Vec::with_capacity(rows.len());

        for row in rows.iter() {
            let item = T::from_row(row)?;

            items.push(item);
        }

        Ok(Some(items))
    }
}
