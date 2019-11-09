use {
    crate::{
        models::{Paging, Resource, Story},
        row, rows,
        schema::{Backend, Schema},
        Error,
    },
    chrono::{DateTime, Utc},
    std::fmt,
};

const SQLITE_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Author (
        Id          TEXT    PRIMARY KEY                         NOT NULL    UNIQUE,
        Name        TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const SQLITE_TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StoryAuthor (
        StoryId     TEXT    REFERENCES Story(Id)                ON UPDATE CASCADE   NOT NULL,
        AuthorId    TEXT    REFERENCES Author(Id)               ON UPDATE CASCADE   NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL
    );";

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Author {
    pub id: String,

    pub name: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Author {
    pub fn all(backend: Backend, paging: Paging) -> Result<(u32, Vec<Self>), Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let authors = rows!(p[conn] => (
                    "SELECT Id, Name, Created, Updated FROM Author ORDER BY Name DESC LIMIT $1 OFFSET $2;",
                    [paging.page_size, (paging.page_size * paging.page)],
                    |row| Ok(Self {
                        id: row.get("Id"),
                        name: row.get("Name"),
                        created: row.get("Created"),
                        updated: row.get("Updated"),
                    })
                ));

                let count = row!(p[conn] => (
                    "SELECT COUNT(Id) as Count FROM Author;",
                    |row| Ok(row.get("Count"))
                ));

                Ok((count, authors))
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let authors = rows!(s[conn] => (
                    "SELECT Id, Name, Created, Updated FROM Author ORDER BY Name DESC LIMIT ? OFFSET ?;",
                    [paging.page_size, (paging.page_size * paging.page)],
                    |row| Ok(Self {
                        id: row.get("Id")?,
                        name: row.get("Name")?,
                        created: row.get("Created")?,
                        updated: row.get("Updated")?,
                    })
                ));

                let count = row!(s[conn] => (
                    "SELECT COUNT(Id) as Count FROM Author;",
                    |row| Ok(row.get("Count")?)
                ));

                Ok((count, authors))
            } //#endregion
        }
    }

    pub fn get(backend: Backend, id: &str) -> Result<Self, Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let author = row!(p[conn] => (
                    "SELECT Id, Name, Created, Updated FROM Author WHERE Id = $1;",
                    [id],
                    |row| Ok(Self {
                        id: row.get("Id"),
                        name: row.get("Name"),
                        created: row.get("Created"),
                        updated: row.get("Updated"),
                    })
                ));

                Ok(author)
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let author = row!(s[conn] => (
                    "SELECT Id, Name, Created, Updated FROM Author WHERE Id = ?;",
                    [id],
                    |row| Ok(Self {
                        id: row.get("Id")?,
                        name: row.get("Name")?,
                        created: row.get("Created")?,
                        updated: row.get("Updated")?,
                    })
                ));

                Ok(author)
            } //#endregion
        }
    }

    pub fn for_stories(
        backend: Backend,
        id: &str,
        paging: Paging,
    ) -> Result<(u32, Vec<Story>), Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let stories = rows!(p[conn] => (
                    "SELECT SA.StoryId FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = $1 ORDER BY S.Updated DESC LIMIT $2 OFFSET $3;",
                    [id, paging.page_size, (paging.page_size * paging.page)],
                    |row| Ok(Story::get(backend.clone(), &row.get::<_, String>("StoryId"))?)
                ));

                let count = row!(p[conn] => (
                    "SELECT COUNT(SA.StoryId) as Count FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = $1;",
                    [id],
                    |row| Ok(row.get("Count"))
                ));

                Ok((count, stories))
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let stories = rows!(s[conn] => (
                    "SELECT SA.StoryId FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = ? ORDER BY S.Updated DESC LIMIT ? OFFSET ?;",
                    [id, paging.page_size, (paging.page_size * paging.page)],
                    |row| Ok(Story::get(backend.clone(), &row.get::<_, String>("StoryId")?)?)
                ));

                let count = row!(s[conn] => (
                    "SELECT COUNT(SA.StoryId) as Count FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = ?;",
                    [id],
                    |row| Ok(row.get("Count")?)
                ));

                Ok((count, stories))
            } //#endregion
        }
    }

    pub fn of_story(backend: Backend, story: &str) -> Result<Vec<Self>, Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let authors = rows!(p[conn] => (
                    "SELECT A.Id, A.Name, A.Created, A.Updated FROM StoryAuthor SA LEFT JOIN Author A ON SA.AuthorId = A.Id WHERE SA.StoryId = $1 ORDER BY A.Name;",
                    [story],
                    |row| Ok(Self {
                        id: row.get("Id"),
                        name: row.get("Name"),
                        created: row.get("Created"),
                        updated: row.get("Updated"),
                    })
                ));

                Ok(authors)
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let authors = rows!(s[conn] => (
                    "SELECT A.Id, A.Name, A.Created, A.Updated FROM StoryAuthor SA LEFT JOIN Author A ON SA.AuthorId = A.Id WHERE SA.StoryId = ? ORDER BY A.Name;",
                    [story],
                    |row| Ok(Self {
                        id: row.get("Id")?,
                        name: row.get("Name")?,
                        created: row.get("Created")?,
                        updated: row.get("Updated")?,
                    })
                ));

                Ok(authors)
            } //#endregion
        }
    }
}

impl Resource for Author {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    fn updated(&self) -> &DateTime<Utc> {
        &self.updated
    }

    fn color(&self) -> (&str, &str) {
        ("blue-700", "blue-500")
    }
}

impl Schema for Author {
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

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<a href=\"/author/{}\">{}</a>", self.id, self.name)
    }
}
