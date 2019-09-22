use {
    crate::{Error, Pool},
    common::models::Series,
};

pub fn story(pool: Pool, story: &str) -> Result<Vec<Series>, Error> {
    let conn = pool.get()?;

    let mut stmt = conn.prepare(
            "SELECT A.Id, A.Name, A.Summary, A.Created, A.Updated FROM StorySeries SA LEFT JOIN Series A ON SA.SeriesId = A.Id WHERE SA.StoryId = ? ORDER BY A.Name;"
        )?;

    let series = stmt.query_map(
        rusqlite::params![&story],
        |row| -> rusqlite::Result<Series> {
            Ok(Series {
                id: row.get("Id")?,
                name: row.get("Name")?,
                summary: row.get("Summary")?,
                created: row.get("Created")?,
                updated: row.get("Updated")?,
                place: None,
            })
        },
    )?;

    series.map(|a| a.map_err(Error::from)).collect()
}
