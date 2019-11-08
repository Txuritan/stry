use {
    crate::{
        models::{Paging, Resource, Story},
        schema::{Backend, Schema},
        Error,
    },
    chrono::{DateTime, Utc},
    rusqlite::{
        types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
        OptionalExtension, Result as RusqliteResult,
    },
    std::fmt,
};

const SQLITE_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Tag (
        Id          TEXT    PRIMARY KEY                         NOT NULL    UNIQUE,
        Name        TEXT                                        NOT NULL,
        Type        TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const SQLITE_TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StoryTag (
        StoryId     TEXT    REFERENCES Story(Id)                ON UPDATE CASCADE   NOT NULL,
        TagId       TEXT    REFERENCES Tag(Id)                  ON UPDATE CASCADE   NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL
    );";

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Tag {
    pub id: String,

    pub name: String,
    #[serde(rename = "type")]
    pub typ: TagType,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Tag {
    pub fn all(backend: Backend, page: u32) -> Result<(u32, Vec<Self>), Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let rows = conn.query(
                    "SELECT Id, Name, Type, Created, Updated FROM Tag ORDER BY Name LIMIT 100 OFFSET ?;",
                    &[&(10 * page)],
                )?;

                if rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let mut tags = Vec::with_capacity(rows.len());

                for row in rows.iter() {
                    tags.push(Self {
                        id: row.get("Id"),
                        name: row.get("Name"),
                        typ: row.get("Type"),
                        created: row.get("Created"),
                        updated: row.get("Updated"),
                    });
                }

                let count_rows = conn.query("SELECT COUNT(Id) as Count FROM Tag;", &[])?;

                if count_rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let count = count_rows.get(0).unwrap().get("Count");

                Ok((count, tags))
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let mut stmt = conn.prepare(
                    "SELECT Id, Name, Type, Created, Updated FROM Tag ORDER BY Name LIMIT 100 OFFSET ?;",
                )?;

                let tag_rows = stmt.query_map(rusqlite::params![10 * page], |row| {
                    Ok(Self {
                        id: row.get("Id")?,
                        name: row.get("Name")?,
                        typ: row.get("Type")?,
                        created: row.get("Created")?,
                        updated: row.get("Updated")?,
                    })
                })?;

                let mut tags = Vec::new();

                for tag in tag_rows {
                    tags.push(tag?);
                }

                let count = conn.query_row(
                    "SELECT COUNT(Id) as Count FROM Tag;",
                    rusqlite::NO_PARAMS,
                    |row| row.get("Count"),
                )?;

                Ok((count, tags))
            } //#endregion
        }
    }

    pub fn all_of_type(
        backend: Backend,
        typ: TagType,
        paging: Paging,
    ) -> Result<(u32, Vec<Self>), Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let rows = conn.query(
                    "SELECT Id, Name, Type, Created, Updated FROM Tag WHERE Type = $1 ORDER BY Name LIMIT $2 OFFSET $3;",
                    &[&typ, &paging.page_size, &(paging.page_size * paging.page)]
                )?;

                if rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let mut tags = Vec::with_capacity(rows.len());

                for row in rows.iter() {
                    tags.push(Self {
                        id: row.get("Id"),
                        name: row.get("Name"),
                        typ: row.get("Type"),
                        created: row.get("Created"),
                        updated: row.get("Updated"),
                    });
                }

                let count_rows = conn.query(
                    "SELECT COUNT(Id) as Count FROM Tag WHERE Type = $1;",
                    &[&typ],
                )?;

                if count_rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let count = count_rows.get(0).unwrap().get("Count");

                Ok((count, tags))
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let mut stmt = conn.prepare(
                    "SELECT Id, Name, Type, Created, Updated FROM Tag WHERE Type = ? ORDER BY Name LIMIT ? OFFSET ?;",
                )?;

                let tag_rows = stmt.query_map(
                    rusqlite::params![typ, paging.page_size, paging.page_size * paging.page],
                    |row| {
                        Ok(Self {
                            id: row.get("Id")?,
                            name: row.get("Name")?,
                            typ: row.get("Type")?,
                            created: row.get("Created")?,
                            updated: row.get("Updated")?,
                        })
                    },
                )?;

                let mut tags = Vec::new();

                for tag in tag_rows {
                    tags.push(tag?);
                }

                let count = conn.query_row(
                    "SELECT COUNT(Id) as Count FROM Tag WHERE Type = ?;",
                    rusqlite::params![typ],
                    |row| row.get("Count"),
                )?;

                Ok((count, tags))
            } //#endregion
        }
    }

    pub fn get(backend: Backend, id: &str) -> Result<Self, Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let rows = conn.query(
                    "SELECT Id, Name, Type, Created, Updated FROM Origin WHERE Id = $1;",
                    &[&id],
                )?;

                if rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let row = rows.get(0).unwrap();

                Ok(Self {
                    id: row.get("Id"),
                    name: row.get("Name"),
                    typ: row.get("Type"),
                    created: row.get("Created"),
                    updated: row.get("Updated"),
                })
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let tag = conn.query_row(
                    "SELECT Id, Name, Type, Created, Updated FROM Origin WHERE Id = ?;",
                    rusqlite::params![id],
                    |row| {
                        Ok(Self {
                            id: row.get("Id")?,
                            name: row.get("Name")?,
                            typ: row.get("Type")?,
                            created: row.get("Created")?,
                            updated: row.get("Updated")?,
                        })
                    },
                )?;

                Ok(tag)
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
                    "SELECT ST.StoryId FROM StoryTag ST LEFT JOIN Story S ON S.Id = StoryId WHERE ST.TagId = $1 ORDER BY S.Updated DESC LIMIT $2 OFFSET $;",
                    &[&id, &paging.page_size, &(paging.page_size * paging.page)],
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
                    "SELECT COUNT(ST.StoryId) as Count FROM StoryTag ST LEFT JOIN Story S ON S.Id = StoryId WHERE ST.TagId = $1;",
                    &[&id]
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
                    "SELECT ST.StoryId FROM StoryTag ST LEFT JOIN Story S ON S.Id = StoryId WHERE ST.TagId = ? ORDER BY S.Updated DESC LIMIT 10 OFFSET ?;"
                )?;

                let story_rows = stmt.query_map(
                    rusqlite::params![id, paging.page_size * paging.page],
                    |row| row.get::<_, String>("StoryId"),
                )?;

                let mut stories = Vec::new();

                for story in story_rows {
                    stories.push(Story::get(backend.clone(), &story?)?);
                }

                let count = conn.query_row(
                    "SELECT COUNT(ST.StoryId) as Count FROM StoryTag ST LEFT JOIN Story S ON S.Id = StoryId WHERE ST.TagId = ?;",
                    rusqlite::params![id], |row|
                    row.get("Count")
                )?;

                Ok((count, stories))
            } //#endregion
        }
    }

    pub fn find_or_create(backend: Backend, name: &str, typ: TagType) -> Result<String, Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let rows = conn.query(
                    "SELECT Id FROM Tag WHERE Name = $1 AND Type = $2;",
                    &[&name, &typ],
                )?;

                if rows.is_empty() {
                    let id = crate::nanoid!();

                    let mut trans = conn.transaction()?;

                    trans.execute(
                        "INSERT INTO Tag(Id, Name, Type) VALUES ($1, $2, $3);",
                        &[&id, &name, &typ],
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
                        "SELECT Id FROM Tag WHERE Name = ? AND Type = ?;",
                        rusqlite::params![name, typ],
                        |row| row.get("Id"),
                    )
                    .optional()?
                {
                    Ok(id)
                } else {
                    let id = crate::nanoid!();

                    let trans = conn.transaction()?;

                    trans.execute(
                        "INSERT INTO Tag(Id, Name, Type) VALUES (?, ?, ?);",
                        rusqlite::params![id, name, typ],
                    )?;

                    trans.commit()?;

                    Ok(id)
                }
            } //#endregion
        }
    }

    pub fn of_story(backend: Backend, story: &str) -> Result<Vec<Self>, Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let rows = conn.query(
                    "SELECT T.Id, T.Name, T.Type, T.Created, T.Updated FROM StoryTag ST LEFT JOIN Tag T ON ST.TagId = T.Id WHERE ST.StoryId = $1 ORDER BY T.Name;",
                    &[&story]
                )?;

                if rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let mut tags = Vec::with_capacity(rows.len());

                for row in rows.iter() {
                    tags.push(Self {
                        id: row.get("Id"),
                        name: row.get("Name"),
                        typ: row.get("Type"),
                        created: row.get("Created"),
                        updated: row.get("Updated"),
                    });
                }

                tags.sort_by(|a, b| a.typ.cmp(&b.typ));

                Ok(tags)
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let mut stmt = conn.prepare(
                    "SELECT T.Id, T.Name, T.Type, T.Created, T.Updated FROM StoryTag ST LEFT JOIN Tag T ON ST.TagId = T.Id WHERE ST.StoryId = ? ORDER BY T.Name;"
                )?;

                let tag_rows = stmt.query_map(rusqlite::params![&story], |row| {
                    Ok(Self {
                        id: row.get("Id")?,
                        name: row.get("Name")?,
                        typ: row.get("Type")?,
                        created: row.get("Created")?,
                        updated: row.get("Updated")?,
                    })
                })?;

                let mut tags = Vec::new();

                for tag in tag_rows {
                    tags.push(tag?);
                }

                tags.sort_by(|a, b| a.typ.cmp(&b.typ));

                Ok(tags)
            } //#endregion
        }
    }
}

impl Resource for Tag {
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
        (self.typ.str(), self.typ.str())
    }
}

impl Schema for Tag {
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

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<a label bg=\"{}\" href=\"/tag/{}\">{}</a>",
            self.typ, self.id, self.name
        )
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(postgres_derive::FromSql, postgres_derive::ToSql)]
#[postgres(name = "tag_type")]
pub enum TagType {
    #[postgres(name = "warning")]
    #[serde(rename = "warning")]
    Warning,

    #[postgres(name = "pairing")]
    #[serde(rename = "pairing")]
    Pairing,

    #[postgres(name = "character")]
    #[serde(rename = "character")]
    Character,

    #[postgres(name = "general")]
    #[serde(rename = "general")]
    General,
}

impl TagType {
    fn str(self) -> &'static str {
        match self {
            TagType::Warning => "red-500",
            TagType::Pairing => "orange-500",
            TagType::Character => "purple-500",
            TagType::General => "gray-700",
        }
    }
}

impl fmt::Display for TagType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TagType::Warning => "red-500",
                TagType::Pairing => "orange-500",
                TagType::Character => "purple-500",
                TagType::General => "gray-700",
            }
        )
    }
}

impl FromSql for TagType {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "warning" => TagType::Warning,
            "pairing" => TagType::Pairing,
            "character" => TagType::Character,
            "general" => TagType::General,
            _ => unreachable!(),
        })
    }
}

impl ToSql for TagType {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(match self {
            TagType::Warning => "warning",
            TagType::Pairing => "pairing",
            TagType::Character => "character",
            TagType::General => "general",
        }
        .into())
    }
}
