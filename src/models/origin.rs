use {
    crate::{
        models::{Paging, Resource, Story},
        params,
        schema::{Backend, Schema},
        Error,
    },
    chrono::{DateTime, Utc},
    rusqlite::OptionalExtension,
    std::fmt,
};

const SQLITE_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Origin (
        Id          TEXT    PRIMARY KEY                         NOT NULL    UNIQUE,
        Name        TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const SQLITE_TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StoryOrigin (
        StoryId     TEXT    REFERENCES Story(Id)                ON UPDATE CASCADE   NOT NULL,
        OriginId    TEXT    REFERENCES Origin(Id)               ON UPDATE CASCADE   NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL
    );";

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Origin {
    pub id: String,

    pub name: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Origin {
    pub fn all(backend: Backend, paging: Paging) -> Result<(u32, Vec<Self>), Error> {
        match backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let rows = conn.query(
                    "SELECT Id, Name, Created, Updated FROM Origin ORDER BY Name DESC LIMIT $1 OFFSET $2;",
                    params!(p => [paging.page_size, (paging.page_size * paging.page)]),
                )?;

                if rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let mut origins = Vec::with_capacity(rows.len());

                for row in rows.iter() {
                    origins.push(Self {
                        id: row.get("Id"),
                        name: row.get("Name"),
                        created: row.get("Created"),
                        updated: row.get("Updated"),
                    });
                }

                let count_rows =
                    conn.query("SELECT COUNT(Id) as Count FROM Origin;", params!(p => []))?;

                if count_rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let count = count_rows.get(0).unwrap().get("Count");

                Ok((count, origins))
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let mut stmt = conn.prepare(
                    "SELECT Id, Name, Created, Updated FROM Origin ORDER BY Name DESC LIMIT ? OFFSET ?;",
                )?;

                let origin_rows = stmt.query_map(
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

                let mut origins = Vec::new();

                for origin in origin_rows {
                    origins.push(origin?);
                }

                let count = conn.query_row(
                    "SELECT COUNT(Id) as Count FROM Origin;",
                    params!(s => []),
                    |row| row.get("Count"),
                )?;

                Ok((count, origins))
            } //#endregion
        }
    }

    pub fn get(backend: Backend, id: &str) -> Result<Self, Error> {
        match backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let rows = conn.query(
                    "SELECT Id, Name, Created, Updated FROM Origin WHERE Id = $1;",
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

                let origin = conn.query_row(
                    "SELECT Id, Name, Created, Updated FROM Origin WHERE Id = ?;",
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

                Ok(origin)
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
                    "SELECT SO.StoryId FROM StoryOrigin SO LEFT JOIN Story S ON S.Id = SO.StoryId WHERE SO.OriginId = $1 ORDER BY S.Updated DESC LIMIT $2 OFFSET $3;",
                    params!(p => [id, paging.page_size, (paging.page_size * paging.page)]),
                )?;

                let mut stories = Vec::<Story>::with_capacity(rows.len());

                for row in rows.iter() {
                    stories.push(Story::get(
                        backend.clone(),
                        &row.get::<_, String>("StoryId"),
                    )?);
                }

                let count_rows = conn.query(
                    "SELECT COUNT(SO.StoryId) as Count FROM StoryOrigin SO LEFT JOIN Story S ON S.Id = StoryId WHERE SO.OriginId = $1;",
                    params!(p => [id])
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
                    "SELECT SO.StoryId FROM StoryOrigin SO LEFT JOIN Story S ON S.Id = SO.StoryId WHERE SO.OriginId = ? ORDER BY S.Updated DESC LIMIT 10 OFFSET ?;"
                )?;

                let story_rows = stmt
                    .query_map(params!(s => [id, paging.page_size * paging.page]), |row| {
                        row.get::<_, String>("StoryId")
                    })?;

                let mut stories = Vec::new();

                for story in story_rows {
                    stories.push(Story::get(backend.clone(), &story?)?);
                }

                let count = conn.query_row(
                    "SELECT COUNT(SO.StoryId) as Count FROM StoryOrigin SO LEFT JOIN Story S ON S.Id = StoryId WHERE SO.OriginId = ?;",
                    params!(s => [id]),
                    |row| row.get("Count")
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
                    "SELECT O.Id, O.Name, O.Created, O.Updated FROM StoryOrigin SO LEFT JOIN Origin O ON SO.OriginId = O.Id WHERE SO.StoryId = $1 ORDER BY O.Name;",
                    params!(p => [story]),
                )?;

                if rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let mut origins = Vec::with_capacity(rows.len());

                for row in rows.iter() {
                    origins.push(Self {
                        id: row.get("Id"),
                        name: row.get("Name"),
                        created: row.get("Created"),
                        updated: row.get("Updated"),
                    });
                }

                Ok(origins)
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let mut stmt = conn.prepare(
                    "SELECT O.Id, O.Name, O.Created, O.Updated FROM StoryOrigin SO LEFT JOIN Origin O ON SO.OriginId = O.Id WHERE SO.StoryId = ? ORDER BY O.Name;"
                )?;

                let origins = stmt.query_map(params!(s => [story]), |row| {
                    Ok(Self {
                        id: row.get("Id")?,
                        name: row.get("Name")?,
                        created: row.get("Created")?,
                        updated: row.get("Updated")?,
                    })
                })?;

                origins.map(|a| a.map_err(Error::from)).collect()
            } //#endregion
        }
    }

    pub fn find_or_create(backend: Backend, name: &str) -> Result<String, Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let rows = conn.query(
                    "SELECT Id FROM Origin WHERE Name = $1;",
                    params!(p => [name]),
                )?;

                if rows.is_empty() {
                    let id = crate::nanoid!();

                    let mut trans = conn.transaction()?;

                    trans.execute(
                        "INSERT INTO Origin(Id, Name) VALUES ($1, $2);",
                        params!(p => [id, name]),
                    )?;

                    trans.commit()?;

                    Ok(id)
                } else {
                    Ok(rows.get(0).unwrap().get("Id"))
                }
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let mut conn = pool.get()?;

                if let Some(id) = conn
                    .query_row(
                        "SELECT Id FROM Origin WHERE Name = ?;",
                        params!(s => [name]),
                        |row| row.get("Id"),
                    )
                    .optional()?
                {
                    Ok(id)
                } else {
                    let id = crate::nanoid!();

                    let trans = conn.transaction()?;

                    trans.execute(
                        "INSERT INTO Origin(Id, Name) VALUES (?, ?);",
                        params!(s => [id, name]),
                    )?;

                    trans.commit()?;

                    Ok(id)
                }
            } //#endregion
        }
    }
}

impl Resource for Origin {
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
        ("green-600", "green-600")
    }
}

impl Schema for Origin {
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

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<a href=\"/origin/{}\">{}</a>", self.id, self.name)
    }
}
