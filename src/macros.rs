#[macro_export]
macro_rules! execute {
    //#region[rgba(241,153,31,0.1)] PostgreSQL
    (p[$conn:expr] => ($query:expr, [$($param:expr),+ $(,)?])) => {
        $conn.execute($query, &[$(&$param as &(dyn postgres::types::ToSql + Sync)),+])?
    };

    (p[$conn:expr] => $query:expr) => {
        $conn.execute($query, crate::macros::POSTGRES_NO_PARAMS)?
    };
    //#endregion

    //#region[rgba(51,103,145,0.1)] SQLite
    (s[$conn:expr] => ($query:expr, [$($param:expr),+ $(,)?])) => {
        $conn.execute($query, &[$(&$param as &dyn rusqlite::types::ToSql),+])?
    };

    (s[$conn:expr] => $query:expr) => {
        $conn.execute($query, crate::macros::SQLITE_NO_PARAMS)?
    };
    //#endregion
}

#[macro_export]
macro_rules! rows {
    //#region[rgba(241,153,31,0.1)] PostgreSQL
    (p[$conn:expr] => ($query:expr, [$($param:expr),+ $(,)?], $body:expr)) => {{
        let rows = $conn.query($query, &[$(&$param as &(dyn postgres::types::ToSql + Sync)),+])?;

        if rows.is_empty() {
            return Err(Error::no_rows_returned());
        }

        let closure: &dyn Fn(&::postgres::row::Row) -> Result<_, crate::error::Error> = &$body;

        let mut vec = Vec::with_capacity(rows.len());

        for row in rows.iter() {
            vec.push(closure(row)?);
        }

        vec
    }};

    (p[$conn:expr] => $query:expr) => {{
        let rows = $conn.query($query, crate::macros::POSTGRES_NO_PARAMS)?;

        if rows.is_empty() {
            return Err(Error::no_rows_returned());
        }

        let closure: &dyn Fn(&::postgres::row::Row) -> Result<_, crate::error::Error> = &$body;

        let mut vec = Vec::with_capacity(rows.len());

        for row in rows.iter() {
            vec.push(closure(row)?);
        }

        vec
    }};
    //#endregion

    //#region[rgba(51,103,145,0.1)] SQLite
    (s[$conn:expr] => ($query:expr, [$($param:expr),+ $(,)?], $body:expr)) => {{
        let mut stmt = $conn.prepare($query)?;

        let mut rows = stmt.query(&[$(&$param as &dyn rusqlite::types::ToSql),+])?;

        // TODO: Add row count check

        let closure: &dyn Fn(&::rusqlite::Row<'_>) -> Result<_, crate::error::Error> = &$body;

        let mut vec = Vec::new();

        while let Some(row) = rows.next()? {
            vec.push(closure(row)?);
        }

        vec
    }};

    (s[$conn:expr] => $query:expr) => {{
        let mut stmt = $conn.prepare($query)?;

        let mut rows = stmt.query(crate::macros::SQLITE_NO_PARAMS)?;

        // TODO: Add row count check

        let closure: &dyn Fn(&::rusqlite::Row<'_>) -> Result<_, crate::error::Error> = &$body;

        let mut vec = Vec::new();

        while let Some(row) = rows.next()? {
            vec.push(closure(row)?);
        }

        vec
    }};
    //#endregion
}

#[macro_export]
macro_rules! row {
    //#region[rgba(241,153,31,0.1)] PostgreSQL
    (p[$conn:expr] => ($query:expr, [$($param:expr),+ $(,)?], $body:expr)) => {{
        let rows = $conn.query($query, &[$(&$param as &(dyn postgres::types::ToSql + Sync)),+])?;

        if rows.is_empty() {
            return Err(Error::no_rows_returned());
        }

        let closure: &dyn Fn(&::postgres::row::Row) -> Result<_, crate::error::Error> = &$body;

        let row = rows.get(0).unwrap();

        closure(row)?
    }};

    (p[$conn:expr] => ($query:expr, $body:expr)) => {{
        let rows = $conn.query($query, crate::macros::POSTGRES_NO_PARAMS)?;

        if rows.is_empty() {
            return Err(Error::no_rows_returned());
        }

        let closure: &dyn Fn(&::postgres::row::Row) -> Result<_, crate::error::Error> = &$body;

        let row = rows.get(0).unwrap();

        closure(row)?
    }};
    //#endregion

    //#region[rgba(51,103,145,0.1)] SQLite
    (s[$conn:expr] => ($query:expr, [$($param:expr),+ $(,)?], $body:expr)) => {{
        let mut stmt = $conn.prepare($query)?;

        let mut rows = stmt.query(&[$(&$param as &dyn rusqlite::types::ToSql),+])?;

        let closure: &dyn Fn(&::rusqlite::Row<'_>) -> Result<_, crate::error::Error> = &$body;

        let row = rows.next()?;

        if row.is_none() {
            return Err(Error::no_rows_returned());
        }

        closure(row.unwrap())?
    }};

    (s[$conn:expr] => ($query:expr, $body:expr)) => {{
        let mut stmt = $conn.prepare($query)?;

        let mut rows = stmt.query(crate::macros::SQLITE_NO_PARAMS)?;

        let closure: &dyn Fn(&::rusqlite::Row<'_>) -> Result<_, crate::error::Error> = &$body;

        let row = rows.next()?;

        if row.is_none() {
            return Err(Error::no_rows_returned());
        }

        closure(row.unwrap())?
    }};
    //#endregion
}

pub(crate) const POSTGRES_NO_PARAMS: &[&(dyn postgres::types::ToSql + Sync)] = &[];
pub(crate) const SQLITE_NO_PARAMS: &[&dyn rusqlite::types::ToSql] = &[];

#[macro_export]
macro_rules! params {
    //#region[rgba(241,153,31,0.1)] PostgreSQL
    (p => [$($param:expr),+ $(,)?]) => {
        &[$(&$param as &(dyn postgres::types::ToSql + Sync)),+]
    };

    (p => []) => {
        crate::macros::POSTGRES_NO_PARAMS
    };
    //#endregion

    //#region[rgba(51,103,145,0.1)] SQLite
    (s => [$($param:expr),+ $(,)?]) => {
        &[$(&$param as &dyn rusqlite::types::ToSql),+]
    };

    (s => []) => {
        crate::macros::SQLITE_NO_PARAMS
    };
    //#endregion
}
