use {
    crate::{
        models::{Paging, Resource, Story},
        params,
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

                let rows = conn.query(
                    "SELECT Id, Name, Created, Updated FROM Author ORDER BY Name DESC LIMIT $1 OFFSET $2;",
                    params!(p => [paging.page_size, (paging.page_size * paging.page)])
                )?;

                if rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let mut authors = Vec::with_capacity(rows.len());

                for row in rows.iter() {
                    authors.push(Self {
                        id: row.get("Id"),
                        name: row.get("Name"),
                        created: row.get("Created"),
                        updated: row.get("Updated"),
                    });
                }

                let count_rows =
                    conn.query("SELECT COUNT(Id) as Count FROM Author;", params!(p => []))?;

                if count_rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let count = count_rows.get(0).unwrap().get("Count");

                Ok((count, authors))
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let mut stmt = conn.prepare(
                    "SELECT Id, Name, Created, Updated FROM Author ORDER BY Name DESC LIMIT ? OFFSET ?;",
                )?;

                let author_rows = stmt.query_map(
                    params!(s => [paging.page_size, paging.page_size * paging.page]),
                    |row| {
                        Ok(Self {
                            id: row.get("Id")?,
                            name: row.get("Name")?,
                            created: row.get("Created")?,
                            updated: row.get("Updated")?,
                        })
                    },
                )?;

                let mut authors = Vec::new();

                for author in author_rows {
                    authors.push(author?);
                }

                let count = conn.query_row(
                    "SELECT COUNT(Id) as Count FROM Author;",
                    params!(s => []),
                    |row| row.get("Count"),
                )?;

                Ok((count, authors))
            } //#endregion
        }
    }

    pub fn get(backend: Backend, id: &str) -> Result<Self, Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let rows = conn.query(
                    "SELECT Id, Name, Created, Updated FROM Author WHERE Id = $1;",
                    params!(p => [id]),
                )?;

                if rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let row = rows.get(0).unwrap();

                Ok(Self {
                    id: row.get("Id"),
                    name: row.get("Name"),
                    created: row.get("Created"),
                    updated: row.get("Updated"),
                })
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let author = conn.query_row(
                    "SELECT Id, Name, Created, Updated FROM Author WHERE Id = ?;",
                    params!(s => [id]),
                    |row| {
                        Ok(Self {
                            id: row.get("Id")?,
                            name: row.get("Name")?,
                            created: row.get("Created")?,
                            updated: row.get("Updated")?,
                        })
                    },
                )?;

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

                let rows = conn.query(
                    "SELECT SA.StoryId FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = $1 ORDER BY S.Updated DESC LIMIT $2 OFFSET $3;",
                    params!(p => [id, paging.page_size, (paging.page_size * paging.page)]),
                )?;

                if rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let mut stories = Vec::with_capacity(rows.len());

                for row in rows.iter() {
                    stories.push(Story::get(
                        backend.clone(),
                        &row.get::<_, String>("StoryId"),
                    )?);
                }

                let count_rows = conn.query(
                    "SELECT COUNT(SA.StoryId) as Count FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = $1;",
                    params!(p => [id]),
                )?;

                if count_rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let count = count_rows.get(0).unwrap().get("Count");

                Ok((count, stories))
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let mut stmt = conn.prepare(
                    "SELECT SA.StoryId FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = ? ORDER BY S.Updated DESC LIMIT ? OFFSET ?;",
                )?;

                let story_rows = stmt.query_map(
                    params!(s => [id, paging.page_size, paging.page_size * paging.page]),
                    |row| row.get::<_, String>("StoryId"),
                )?;

                let mut stories = Vec::new();

                for story in story_rows {
                    stories.push(Story::get(backend.clone(), &story?)?);
                }

                let count = conn.query_row(
                    "SELECT COUNT(SA.StoryId) as Count FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = ?;",
                    params!(s => [id]),
                    |row| row.get("Count"),
                )?;

                Ok((count, stories))
            } //#endregion
        }
    }

    pub fn of_story(backend: Backend, story: &str) -> Result<Vec<Self>, Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let rows = conn.query(
                    "SELECT A.Id, A.Name, A.Created, A.Updated FROM StoryAuthor SA LEFT JOIN Author A ON SA.AuthorId = A.Id WHERE SA.StoryId = $1 ORDER BY A.Name;",
                    params!(p => [story])
                )?;

                if rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let mut authors = Vec::with_capacity(rows.len());

                for row in rows.iter() {
                    authors.push(Self {
                        id: row.get("Id"),
                        name: row.get("Name"),
                        created: row.get("Created"),
                        updated: row.get("Updated"),
                    });
                }

                Ok(authors)
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let mut stmt = conn.prepare(
                    "SELECT A.Id, A.Name, A.Created, A.Updated FROM StoryAuthor SA LEFT JOIN Author A ON SA.AuthorId = A.Id WHERE SA.StoryId = ? ORDER BY A.Name;"
                )?;

                let authors = stmt.query_map(params!(s => [story]), |row| {
                    Ok(Self {
                        id: row.get("Id")?,
                        name: row.get("Name")?,
                        created: row.get("Created")?,
                        updated: row.get("Updated")?,
                    })
                })?;

                authors.map(|a| a.map_err(Error::from)).collect()
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
