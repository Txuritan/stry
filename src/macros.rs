pub(crate) const POSTGRES_NO_PARAMS: &[&(dyn postgres::types::ToSql + Sync)] = &[];
pub(crate) const SQLITE_NO_PARAMS: &[&dyn rusqlite::types::ToSql] = &[];

#[macro_export]
macro_rules! params {
    (p => []) => {
        crate::macros::POSTGRES_NO_PARAMS
    };

    (p => [$($param:expr),+ $(,)?]) => {
        &[$(&$param as &(dyn postgres::types::ToSql + Sync)),+]
    };

    (p => [$($param_name:literal: $param_val:expr),+ $(,)?]) => {
        &[$(($param_name, &$param_val as &(dyn postgres::types::ToSql + Sync))),+]
    };

    (s => []) => {
        crate::macros::SQLITE_NO_PARAMS
    };

    (s => [$($param:expr),+ $(,)?]) => {
        &[$(&$param as &dyn rusqlite::types::ToSql),+]
    };

    (s => [$($param_name:literal: $param_val:expr),+ $(,)?]) => {
        &[$(($param_name, &$param_val as &dyn rusqlite::types::ToSql)),+]
    };
}