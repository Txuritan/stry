use {
    anyhow::Context,
    rewryte::sqlite::FromRow,
    rusqlite::{Connection, Row, ToSql},
    std::{
        borrow::Cow,
        fmt,
        path::{Path, PathBuf},
    },
    stry_common::models::Rating,
};

// A slightly modified version of ivanceras' `r2d2-sqlite`
#[derive(Debug)]
enum Source {
    File(PathBuf),
    #[cfg(test)]
    Memory,
}

type InitFn = dyn Fn(&mut Connection) -> Result<(), rusqlite::Error> + Send + Sync + 'static;

pub struct SqliteConnectionManager {
    source: Source,
    init: Option<Box<InitFn>>,
}

impl fmt::Debug for SqliteConnectionManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SqliteConnectionManager")
            .field("source", &self.source)
            .field("init", &self.init.as_ref().map(|_| "InitFn"))
            .finish()
    }
}

impl SqliteConnectionManager {
    pub fn file<P: AsRef<Path>>(path: P) -> Self {
        Self {
            source: Source::File(path.as_ref().to_path_buf()),
            init: None,
        }
    }

    #[cfg(test)]
    pub fn memory() -> Self {
        Self {
            source: Source::Memory,
            init: None,
        }
    }

    pub fn with_init<F>(self, init: F) -> Self
    where
        F: Fn(&mut Connection) -> Result<(), rusqlite::Error> + Send + Sync + 'static,
    {
        Self {
            init: Some(Box::new(init)),
            ..self
        }
    }
}

impl r2d2::ManageConnection for SqliteConnectionManager {
    type Connection = Connection;
    type Error = rusqlite::Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let conn = match self.source {
            Source::File(ref path) => Connection::open(path),
            #[cfg(test)]
            Source::Memory => Connection::open_in_memory(),
        };

        conn.map_err(Into::into).and_then(|mut c| match self.init {
            None => Ok(c),
            Some(ref init) => init(&mut c).map(|_| c),
        })
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.execute_batch("").map_err(Into::into)
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

#[derive(Debug)]
pub enum Wrapper<'p> {
    Cow(Cow<'p, str>),
    Rating(Rating),
    Num(i32),
}

impl<'p> ToSql for Wrapper<'p> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            Wrapper::Cow(cow) => cow.to_sql(),
            Wrapper::Rating(rating) => rating.to_sql(),
            Wrapper::Num(num) => num.to_sql(),
        }
    }
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
