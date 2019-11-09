use {
    crate::{
        models::{Paging, Resource, Story},
        row, rows, execute, params,
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

                let origins = rows!(p[conn] => (
                    "SELECT Id, Name, Created, Updated FROM Origin ORDER BY Name DESC LIMIT $1 OFFSET $2;",
                    [paging.page_size, (paging.page_size * paging.page)],
                    |row| Ok(Self {
                        id: row.get("Id"),
                        name: row.get("Name"),
                        created: row.get("Created"),
                        updated: row.get("Updated"),
                    })
                ));

                let count = row!(p[conn] => (
                    "SELECT COUNT(Id) as Count FROM Origin;",
                    |row| Ok(row.get("Count"))
                ));

                Ok((count, origins))
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let origins = rows!(s[conn] => (
                    "SELECT Id, Name, Created, Updated FROM Origin ORDER BY Name DESC LIMIT ? OFFSET ?;",
                    [paging.page_size, (paging.page_size * paging.page)],
                    |row| Ok(Self {
                        id: row.get("Id")?,
                        name: row.get("Name")?,
                        created: row.get("Created")?,
                        updated: row.get("Updated")?,
                    })
                ));

                let count = row!(s[conn] => (
                    "SELECT COUNT(Id) as Count FROM Origin;",
                    |row| Ok(row.get("Count")?)
                ));

                Ok((count, origins))
            } //#endregion
        }
    }

    pub fn get(backend: Backend, id: &str) -> Result<Self, Error> {
        match backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let origin = row!(p[conn] => (
                    "SELECT Id, Name, Created, Updated FROM Origin WHERE Id = $1;",
                    [id],
                    |row| Ok(Self {
                        id: row.get("Id"),
                        name: row.get("Name"),
                        created: row.get("Created"),
                        updated: row.get("Updated"),
                    })
                ));

                Ok(origin)
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let origin = row!(s[conn] => (
                    "SELECT Id, Name, Created, Updated FROM Origin WHERE Id = ?;",
                    [id],
                    |row| Ok(Self {
                        id: row.get("Id")?,
                        name: row.get("Name")?,
                        created: row.get("Created")?,
                        updated: row.get("Updated")?,
                    })
                ));

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

                let stories = rows!(p[conn] => (
                    "SELECT SO.StoryId FROM StoryOrigin SO LEFT JOIN Story S ON S.Id = SO.StoryId WHERE SO.OriginId = $1 ORDER BY S.Updated DESC LIMIT $2 OFFSET $3;",
                    [id, paging.page_size, (paging.page_size * paging.page)],
                    |row| Ok(Story::get(backend.clone(), &row.get::<_, String>("StoryId"))?)
                ));

                let count = row!(p[conn] => (
                    "SELECT COUNT(SO.StoryId) as Count FROM StoryOrigin SO LEFT JOIN Story S ON S.Id = StoryId WHERE SO.OriginId = $1;",
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
                    "SELECT SO.StoryId FROM StoryOrigin SO LEFT JOIN Story S ON S.Id = SO.StoryId WHERE SO.OriginId = ? ORDER BY S.Updated DESC LIMIT ? OFFSET ?;",
                    [id, paging.page_size, (paging.page_size * paging.page)],
                    |row| Ok(Story::get(backend.clone(), &row.get::<_, String>("StoryId")?)?)
                ));

                let count = row!(s[conn] => (
                    "SELECT COUNT(SO.StoryId) as Count FROM StoryOrigin SO LEFT JOIN Story S ON S.Id = StoryId WHERE SO.OriginId = ?;",
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

                let origins = rows!(p[conn] => (
                    "SELECT O.Id, O.Name, O.Created, O.Updated FROM StoryOrigin SO LEFT JOIN Origin O ON SO.OriginId = O.Id WHERE SO.StoryId = $1 ORDER BY O.Name;",
                    [story],
                    |row| Ok(Self {
                        id: row.get("Id"),
                        name: row.get("Name"),
                        created: row.get("Created"),
                        updated: row.get("Updated"),
                    })
                ));

                Ok(origins)
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let origins = rows!(s[conn] => (
                    "SELECT O.Id, O.Name, O.Created, O.Updated FROM StoryOrigin SO LEFT JOIN Origin O ON SO.OriginId = O.Id WHERE SO.StoryId = ? ORDER BY O.Name;",
                    [story],
                    |row| Ok(Self {
                        id: row.get("Id")?,
                        name: row.get("Name")?,
                        created: row.get("Created")?,
                        updated: row.get("Updated")?,
                    })
                ));

                Ok(origins)
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
