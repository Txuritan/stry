use {
    crate::{models::story, Error, Pool},
    common::models::{Story, Tag, TagType},
    rusqlite::OptionalExtension,
};

pub fn all(pool: Pool, page: u32) -> Result<(u32, Vec<Tag>), Error> {
    let conn = pool.get()?;

    let mut stmt = conn.prepare(
        "SELECT Id, Name, Type, Created, Updated FROM Tag ORDER BY Name LIMIT 100 OFFSET ?;",
    )?;

    let tag_rows = stmt.query_map(rusqlite::params![10 * page], |row| {
        Ok(Tag {
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
}

pub fn all_of_type(pool: Pool, typ: TagType, page: u32) -> Result<(u32, Vec<Tag>), Error> {
    let conn = pool.get()?;

    let mut stmt = conn.prepare(
            "SELECT Id, Name, Type, Created, Updated FROM Tag WHERE Type = ? ORDER BY Name LIMIT 100 OFFSET ?;",
        )?;

    let tag_rows = stmt.query_map(rusqlite::params![typ, 10 * page], |row| {
        Ok(Tag {
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
        "SELECT COUNT(Id) as Count FROM Tag WHERE Type = ?;",
        rusqlite::params![typ],
        |row| row.get("Count"),
    )?;

    Ok((count, tags))
}

pub fn for_stories(pool: Pool, id: &str, page: u32) -> Result<(u32, Vec<Story>), Error> {
    let conn = pool.get()?;

    let mut stmt = conn.prepare("SELECT ST.StoryId FROM StoryTag ST LEFT JOIN Story S ON S.Id = StoryId WHERE ST.TagId = ? ORDER BY S.Updated DESC LIMIT 10 OFFSET ?;")?;

    let story_rows = stmt.query_map(rusqlite::params![id, 10 * page], |row| {
        row.get::<_, String>("StoryId")
    })?;

    let mut stories = Vec::new();

    for story in story_rows {
        stories.push(story::get(pool.clone(), &story?)?);
    }

    let count = conn.query_row("SELECT COUNT(ST.StoryId) as Count FROM StoryTag ST LEFT JOIN Story S ON S.Id = StoryId WHERE ST.TagId = ?;", rusqlite::params![id], |row| row.get("Count"))?;

    Ok((count, stories))
}

pub fn find_or_create(pool: Pool, name: &str, typ: TagType) -> Result<String, Error> {
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
}

pub fn of_story(pool: Pool, story: &str) -> Result<Vec<Tag>, Error> {
    let conn = pool.get()?;

    let mut stmt = conn.prepare(
            "SELECT T.Id, T.Name, T.Type, T.Created, T.Updated FROM StoryTag ST LEFT JOIN Tag T ON ST.TagId = T.Id WHERE ST.StoryId = ? ORDER BY T.Name;"
        )?;

    let tag_rows = stmt.query_map(rusqlite::params![&story], |row| self::row(row))?;

    let mut tags = Vec::<Tag>::new();

    for tag in tag_rows {
        tags.push(tag?);
    }

    tags.sort_by(|a, b| a.typ.cmp(&b.typ));

    Ok(tags)
}

fn row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Tag> {
    Ok(Tag {
        id: row.get("Id")?,
        name: row.get("Name")?,
        typ: row.get("Type")?,
        created: row.get("Created")?,
        updated: row.get("Updated")?,
    })
}
