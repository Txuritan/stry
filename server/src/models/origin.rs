use {
    crate::{models::story, Error, Pool},
    common::models::{Origin, Story},
};
pub fn all(pool: Pool, page: u32) -> Result<(u32, Vec<Origin>), Error> {
    let conn = pool.get()?;

    let mut stmt = conn.prepare(
        "SELECT Id, Name, Created, Updated FROM Origin ORDER BY Name DESC LIMIT 100 OFFSET ?;",
    )?;

    let origin_rows = stmt.query_map(rusqlite::params![10 * page], |row| {
        Ok(Origin {
            id: row.get("Id")?,
            name: row.get("Name")?,
            created: row.get("Created")?,
            updated: row.get("Updated")?,
        })
    })?;

    let mut origins = Vec::new();

    for origin in origin_rows {
        origins.push(origin?);
    }

    let count = conn.query_row(
        "SELECT COUNT(Id) as Count FROM Origin;",
        rusqlite::NO_PARAMS,
        |row| row.get("Count"),
    )?;

    Ok((count, origins))
}

pub fn for_stories(pool: Pool, id: &str, page: u32) -> Result<(u32, Vec<Story>), Error> {
    let conn = pool.get()?;

    let mut stmt = conn.prepare("SELECT SO.StoryId FROM StoryOrigin SO LEFT JOIN Story S ON S.Id = SO.StoryId WHERE SO.OriginId = ? ORDER BY S.Updated DESC LIMIT 10 OFFSET ?;")?;

    let story_rows = stmt.query_map(rusqlite::params![id, 10 * page], |row| {
        row.get::<_, String>("StoryId")
    })?;

    let mut stories = Vec::<Story>::new();

    for story in story_rows {
        stories.push(story::get(pool.clone(), &story?)?);
    }

    let count = conn.query_row("SELECT COUNT(SO.StoryId) as Count FROM StoryOrigin SO LEFT JOIN Story S ON S.Id = StoryId WHERE SO.OriginId = ?;", rusqlite::params![id], |row| row.get("Count"))?;

    Ok((count, stories))
}

pub fn of_story(pool: Pool, story: &str) -> Result<Vec<Origin>, Error> {
    let conn = pool.get()?;

    let mut stmt = conn.prepare(
            "SELECT O.Id, O.Name, O.Created, O.Updated FROM StoryOrigin SO LEFT JOIN Origin O ON SO.OriginId = O.Id WHERE SO.StoryId = ? ORDER BY O.Name;"
        )?;

    let origins = stmt.query_map(
        rusqlite::params![&story],
        |row| -> rusqlite::Result<Origin> {
            Ok(Origin {
                id: row.get("Id")?,
                name: row.get("Name")?,
                created: row.get("Created")?,
                updated: row.get("Updated")?,
            })
        },
    )?;

    origins.map(|a| a.map_err(Error::from)).collect()
}
