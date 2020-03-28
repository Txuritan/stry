use {db_derive::PoolKind, std::fmt};

pub trait Schema {
    fn schema(conn: impl Into<PoolKind>, buff: &mut impl fmt::Write) -> fmt::Result {
        match conn.into() {
            PoolKind::PostgreSQL => Self::postgres_schema(buff),
            PoolKind::SQLite => Self::sqlite_schema(buff),
        }
    }

    fn postgres_schema(buff: &mut impl fmt::Write) -> fmt::Result;

    fn sqlite_schema(buff: &mut impl fmt::Write) -> fmt::Result;
}
