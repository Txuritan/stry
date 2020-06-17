pub mod mysql;
pub mod postgresql;
pub mod sqlite;

use std::{io, str::FromStr};

#[derive(Clone, Copy, Debug)]
pub enum DatabaseType {
    MySQL,
    PostgreSQL,
    SQLite,
}

impl FromStr for DatabaseType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mysql" => Ok(DatabaseType::MySQL),
            "postgresql" => Ok(DatabaseType::PostgreSQL),
            "sqlite" => Ok(DatabaseType::SQLite),
            t => anyhow::bail!("`{}` is not a database type", t),
        }
    }
}

pub trait ToSql<W: io::Write>: ToMySQL<W> + ToPostgreSQL<W> + ToSQLite<W> {
    fn to_sql(&self, writer: &mut W, typ: DatabaseType) -> anyhow::Result<()> {
        match typ {
            DatabaseType::MySQL => self.to_mysql(writer)?,
            DatabaseType::PostgreSQL => self.to_postgresql(writer)?,
            DatabaseType::SQLite => self.to_sqlite(writer)?,
        }

        Ok(())
    }
}

pub trait ToMySQL<W: io::Write> {
    fn to_mysql(&self, writer: &mut W) -> anyhow::Result<()>;
}

pub trait ToPostgreSQL<W: io::Write> {
    fn to_postgresql(&self, writer: &mut W) -> anyhow::Result<()>;
}

pub trait ToSQLite<W: io::Write> {
    fn to_sqlite(&self, writer: &mut W) -> anyhow::Result<()>;
}

impl<W: io::Write, T: ToMySQL<W> + ToPostgreSQL<W> + ToSQLite<W>> ToSql<W> for T {}
