use {
    crate::schema::Schema,
    chrono::{DateTime, Utc},
    std::fmt,
};

const TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Series (
        Id          TEXT    PRIMARY KEY                         NOT NULL    UNIQUE,
        Name        TEXT                                        NOT NULL,
        Summary     TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StorySeries (
        StoryId     TEXT        REFERENCES Story(Id)                ON UPDATE CASCADE   NOT NULL,
        SeriesId    TEXT        REFERENCES Series(Id)               ON UPDATE CASCADE   NOT NULL,
        Place       INTEGER     REFERENCES Series(Id)                                   NOT NULL,
        Created     TEXT        DEFAULT (DATETIME('now', 'utc'))                        NOT NULL,
        Updated     TEXT        DEFAULT (DATETIME('now', 'utc'))                        NOT NULL
    );";

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Series {
    pub id: String,

    pub name: String,

    pub summary: String,

    pub place: Option<i32>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Schema for Series {
    fn schema(m: &mut impl fmt::Write) -> fmt::Result {
        writeln!(m, "{}", TABLE)?;
        writeln!(m, "{}", TABLE_BRIDGE)?;

        Ok(())
    }
}
