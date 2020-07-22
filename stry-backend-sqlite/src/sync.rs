// A module with items that must be synced with other instances of itself
// This allows for trait implementations of database types

use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
    Result,
};

// NOTICE: must be kept in-sync with story-dl's site enum
pub enum Sites {
    ArchiveOfOurOwn,
    FanFictionNet,
}

impl Into<stry_common::models::sync::Sites> for Sites {
    fn into(self) -> stry_common::models::sync::Sites {
        match self {
            Sites::ArchiveOfOurOwn => stry_common::models::sync::Sites::ArchiveOfOurOwn,
            Sites::FanFictionNet => stry_common::models::sync::Sites::FanFictionNet,
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
