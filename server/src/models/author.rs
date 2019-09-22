use {
    crate::{models::story, Error, Pool},
    common::models::{Author, Story},
};

pub fn all(pool: Pool, page: u32) -> Result<(u32, Vec<Author>), Error> {
    let conn = pool.get()?;

    let mut stmt = conn.prepare(
        "SELECT Id, Name, Created, Updated FROM Author ORDER BY Name DESC LIMIT 100 OFFSET ?;",
    )?;

    let author_rows = stmt.query_map(rusqlite::params![10 * page], |row| {
        Ok(Author {
            id: row.get("Id")?,
            name: row.get("Name")?,
            created: row.get("Created")?,
            updated: row.get("Updated")?,
        })
    })?;

    let mut authors = Vec::new();

    for author in author_rows {
        authors.push(author?);
    }

    let count = conn.query_row(
        "SELECT COUNT(Id) as Count FROM Author;",
        rusqlite::NO_PARAMS,
        |row| row.get("Count"),
    )?;

    Ok((count, authors))
}

pub fn for_stories(pool: Pool, id: &str, page: u32) -> Result<(u32, Vec<Story>), Error> {
    let conn = pool.get()?;

    let mut stmt = conn.prepare("SELECT SA.StoryId FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = ? ORDER BY S.Updated DESC LIMIT 10 OFFSET ?;")?;

    let story_rows = stmt.query_map(rusqlite::params![id, 10 * page], |row| {
        row.get::<_, String>("StoryId")
    })?;

    let mut stories = Vec::new();

    for story in story_rows {
        stories.push(story::get(pool.clone(), &story?)?);
    }

    let count = conn.query_row("SELECT COUNT(SA.StoryId) as Count FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = ?;", rusqlite::params![id], |row| row.get("Count"))?;

    Ok((count, stories))
}

pub fn of_story(pool: Pool, story: &str) -> Result<Vec<Author>, Error> {
    let conn = pool.get()?;

    let mut stmt = conn.prepare(
            "SELECT A.Id, A.Name, A.Created, A.Updated FROM StoryAuthor SA LEFT JOIN Author A ON SA.AuthorId = A.Id WHERE SA.StoryId = ? ORDER BY A.Name;"
        )?;

    let authors = stmt.query_map(
        rusqlite::params![&story],
        |row| -> rusqlite::Result<Author> {
            Ok(Author {
                id: row.get("Id")?,
                name: row.get("Name")?,
                created: row.get("Created")?,
                updated: row.get("Updated")?,
            })
        },
    )?;

    authors.map(|a| a.map_err(Error::from)).collect()
}
