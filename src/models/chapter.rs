use {
    crate::{
        params,
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
        StoryId     TEXT    REFERENCES Story(Id)                ON UPDATE CASCADE   NOT NULL,
        ChapterId   TEXT    REFERENCES Chapter(Id)              ON UPDATE CASCADE   NOT NULL,
        Place       INTEGER                                                         NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL
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

                let rows = conn.query(
                    "SELECT C.Id, C.Name, C.Pre, C.Main, C.Post, C.Words, C.Created, C.Updated FROM StoryChapter SC LEFT JOIN Chapter C ON SC.ChapterId = C.Id WHERE SC.StoryId = $1 AND SC.Place = $2;",
                    params!(p => [story, place])
                )?;

                if rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let row = rows.get(0).unwrap();

                Ok(Self {
                    id: row.get("Id"),
                    name: row.get("Name"),
                    pre: row.get("Pre"),
                    main: row.get("Main"),
                    post: row.get("Post"),
                    words: row.get("Words"),
                    created: row.get("Created"),
                    updated: row.get("Updated"),
                })
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let chapter = conn.query_row(
                    "SELECT C.Id, C.Name, C.Pre, C.Main, C.Post, C.Words, C.Created, C.Updated FROM StoryChapter SC LEFT JOIN Chapter C ON SC.ChapterId = C.Id WHERE SC.StoryId = ? AND SC.Place = ?;",
                    params!(s => [story, place]),
                    |row| {
                        Ok(Self {
                            id: row.get("Id")?,
                            name: row.get("Name")?,
                            pre: row.get("Pre")?,
                            main: row.get("Main")?,
                            post: row.get("Post")?,
                            words: row.get("Words")?,
                            created: row.get("Created")?,
                            updated: row.get("Updated")?,
                        })
                    }
                )?;

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
