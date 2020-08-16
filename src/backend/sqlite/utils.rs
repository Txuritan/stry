use {
    crate::models::Rating,
    anyhow::Context,
    rewryte::sqlite::FromRow,
    rusqlite::{Row, ToSql},
    std::borrow::Cow,
};

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
