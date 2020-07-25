// A module with items that must be synced with other instances of itself
// This allows for trait implementations of database types
use {
    rusqlite::{
        types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
        Result,
    },
    std::fmt,
};

// NOTICE: must be kept in-sync with story-dl's site enum
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Sites {
    ArchiveOfOurOwn,
    FanFictionNet,
}

impl Sites {
    pub fn url(&self) -> &'static str {
        match self {
            Sites::ArchiveOfOurOwn => "https://archiveofourown.org/",
            Sites::FanFictionNet => "https://fanfiction.net/",
        }
    }
}

impl fmt::Display for Sites {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sites::ArchiveOfOurOwn => write!(f, "Archive of Our Own"),
            Sites::FanFictionNet => write!(f, "FanFiction.net"),
        }
    }
}

impl FromSql for Sites {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        value.as_str().and_then(|value| match value {
            "archive-of-our-own" => Ok(Sites::ArchiveOfOurOwn),
            "fanfiction-net" => Ok(Sites::FanFictionNet),
            _ => Err(FromSqlError::InvalidType),
        })
    }
}

impl ToSql for Sites {
    fn to_sql(&self) -> Result<ToSqlOutput> {
        Ok(ToSqlOutput::from(match self {
            Sites::ArchiveOfOurOwn => "archive-of-our-own",
            Sites::FanFictionNet => "fanfiction-net",
        }))
    }
}
