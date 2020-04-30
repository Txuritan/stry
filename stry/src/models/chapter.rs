use {
    crate::schema::Schema,
    chrono::{DateTime, Utc},
    std::fmt,
};

const SQLITE_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Chapter (
        Id          TEXT        PRIMARY KEY                         NOT NULL    UNIQUE,
        Name        TEXT                                            NOT NULL,
        Pre         TEXT        DEFAULT ('')                        NOT NULL,
        Main        TEXT        DEFAULT ('')                        NOT NULL,
        Post        TEXT        DEFAULT ('')                        NOT NULL,
        Words       INTEGER                                         NOT NULL,
        Created     TEXT        DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT        DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const SQLITE_TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StoryChapter (
        StoryId     TEXT        NOT NULL    REFERENCES Story(Id)                ON UPDATE CASCADE,
        ChapterId   TEXT        NOT NULL    REFERENCES Chapter(Id)              ON UPDATE CASCADE,
        Place       INTEGER     NOT NULL                                    ,
        Created     TEXT        NOT NULL    DEFAULT (DATETIME('now', 'utc')),
        Updated     TEXT        NOT NULL    DEFAULT (DATETIME('now', 'utc'))
    );";

#[rustfmt::skip]
#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(db_derive::Table)]
#[table(schema)]
pub struct Chapter {
    #[table(rename = "Id")]
    pub id: String,

    #[table(rename = "Name")]
    pub name: String,

    #[table(rename = "Pre")]
    pub pre: String,

    #[table(rename = "Main")]
    pub main: String,

    #[table(rename = "Post")]
    pub post: String,

    #[table(rename = "Words")]
    pub words: i64,

    #[table(rename = "Created")]
    pub created: DateTime<Utc>,

    #[table(rename = "Updated")]
    pub updated: DateTime<Utc>,
}

impl Schema for Chapter {
    fn postgres_schema(_buff: &mut impl fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn sqlite_schema(buff: &mut impl fmt::Write) -> fmt::Result {
        writeln!(buff, "{}", SQLITE_TABLE)?;
        writeln!(buff, "{}", SQLITE_TABLE_BRIDGE)?;

        Ok(())
    }
}
