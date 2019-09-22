use {
    crate::{Error, Pool},
    common::models::Chapter,
};

pub fn of_story(pool: Pool, story: &str, place: u32) -> Result<Chapter, Error> {
    let conn = pool.get()?;

    let chapter = conn.query_row(
            "SELECT C.Id, C.Name, C.Raw, C.Words, C.Created, C.Updated FROM StoryChapter SC LEFT JOIN Chapter C ON SC.ChapterId = C.Id WHERE SC.StoryId = ? AND SC.Place = ?;",
            rusqlite::params![&story, &place], |row| -> rusqlite::Result<Chapter> {
                Ok(Chapter {
                    id: row.get("Id")?,
                    name: row.get("Name")?,
                    raw: row.get("Raw")?,
                    words: row.get("Words")?,
                    created: row.get("Created")?,
                    updated: row.get("Updated")?,
                })
            }
        )?;

    Ok(chapter)
}
