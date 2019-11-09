use {
    crate::{
        row,
        schema::{Backend, Schema},
        Error,
    },
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Chapter {
    pub id: String,

    pub name: String,

    pub pre: String,
    pub main: String,
    pub post: String,

    pub words: u32,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Chapter {
    pub fn of_story(backend: Backend, story: &str, place: u32) -> Result<Self, Error> {
        match backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let chapter = row!(p[conn] => (
                    "SELECT C.Id, C.Name, C.Pre, C.Main, C.Post, C.Words, C.Created, C.Updated FROM StoryChapter SC LEFT JOIN Chapter C ON SC.ChapterId = C.Id WHERE SC.StoryId = $1 AND SC.Place = $2;",
                    [story, place],
                    |row| Ok(Self {
                        id: row.get("Id"),
                        name: row.get("Name"),
                        pre: row.get("Pre"),
                        main: row.get("Main"),
                        post: row.get("Post"),
                        words: row.get("Words"),
                        created: row.get("Created"),
                        updated: row.get("Updated"),
                    })
                ));

                Ok(chapter)
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let chapter = row!(s[conn] => (
                    "SELECT C.Id, C.Name, C.Pre, C.Main, C.Post, C.Words, C.Created, C.Updated FROM StoryChapter SC LEFT JOIN Chapter C ON SC.ChapterId = C.Id WHERE SC.StoryId = ? AND SC.Place = ?;",
                    [story, place],
                    |row| Ok(Self {
                        id: row.get("Id")?,
                        name: row.get("Name")?,
                        pre: row.get("Pre")?,
                        main: row.get("Main")?,
                        post: row.get("Post")?,
                        words: row.get("Words")?,
                        created: row.get("Created")?,
                        updated: row.get("Updated")?,
                    })
                ));

                Ok(chapter)
            } //#endregion
        }
    }
}

impl Schema for Chapter {
    fn schema(b: Backend, m: &mut impl fmt::Write) -> fmt::Result {
        match b {
            Backend::PostgreSQL { .. } => {}
            Backend::SQLite { .. } => {
                writeln!(m, "{}", SQLITE_TABLE)?;
                writeln!(m, "{}", SQLITE_TABLE_BRIDGE)?;
            }
        }

        Ok(())
    }
}
