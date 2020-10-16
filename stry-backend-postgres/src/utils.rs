use rewryte::postgres::FromRow;

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
