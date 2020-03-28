use {
    crate::schema::Schema,
    chrono::{DateTime, Utc},
    std::fmt,
};

const POSTGRES_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Notification (
        Id          TEXT    PRIMARY KEY                         NOT NULL    UNIQUE,
        Level       TEXT                                        NOT NULL,
        Head        TEXT                                        NOT NULL,
        Body        TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const SQLITE_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Notification (
        Id          TEXT    PRIMARY KEY                         NOT NULL    UNIQUE,
        Level       TEXT                                        NOT NULL,
        Head        TEXT                                        NOT NULL,
        Body        TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

#[rustfmt::skip]
#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(db_derive::Table)]
#[table(schema)]
pub struct Notification {
    #[table(rename = "Id")]
    pub id: String,

    #[table(rename = "Level")]
    #[serde(rename = "level")]
    pub level: Level,

    #[table(rename = "Head")]
    pub head: String,
    #[table(rename = "Body")]
    pub body: String,

    #[table(rename = "Created")]
    pub created: DateTime<Utc>,

    #[table(rename = "Updated")]
    pub updated: DateTime<Utc>,
}

impl Schema for Notification {
    fn postgres_schema(buff: &mut impl fmt::Write) -> fmt::Result {
        writeln!(buff, "{}", POSTGRES_TABLE)?;

        Ok(())
    }

    fn sqlite_schema(buff: &mut impl fmt::Write) -> fmt::Result {
        writeln!(buff, "{}", SQLITE_TABLE)?;

        Ok(())
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(db_derive::Kind)]
pub enum Level {
    #[kind(rename = "error")]
    #[serde(rename = "error")]
    Error,

    #[kind(rename = "success")]
    #[serde(rename = "success")]
    Success,

    #[kind(rename = "info")]
    #[serde(rename = "info")]
    Info,

    #[kind(rename = "general")]
    #[serde(rename = "general")]
    General,
}
