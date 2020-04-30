pub(crate) use std::io;

pub mod column;
pub mod database;
pub mod foreign_key;
pub mod table;

pub use self::{
    column::{Column, Schema},
    database::Database,
    foreign_key::ForeignKey,
    table::Table,
};

pub trait ToSql<W: io::Write> {
    fn to_mysql(&self, writer: &mut W, is_last: bool) -> io::Result<()>;
    fn to_postgresql(&self, writer: &mut W, is_last: bool) -> io::Result<()>;
    fn to_sqlite(&self, writer: &mut W, is_last: bool) -> io::Result<()>;
}
