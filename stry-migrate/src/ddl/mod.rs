pub(crate) use std::io;

pub mod column;
pub mod database;
pub mod table;

pub use self::{
    column::Column,
    database::Database,
    table::{ForeignKey, Table},
};

#[derive(Clone, Copy, Debug)]
pub enum DatabaseType {
    MySQL,
    PostgreSQL,
    SQLite,
}

pub trait ToSql<W: io::Write> {
    type Params;

    fn to_sql(&self, writer: &mut W, typ: DatabaseType, params: Self::Params)
        -> anyhow::Result<()>;
}
