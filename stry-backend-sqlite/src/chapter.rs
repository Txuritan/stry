use {
    crate::SqliteBackend,
    std::borrow::Cow,
    stry_common::{models::Chapter, BackendChapter},
};

#[async_trait::async_trait]
impl BackendChapter for SqliteBackend {
    async fn get_chapter(
        &self,
        story_id: Cow<'static, str>,
        chapter_number: u32,
    ) -> anyhow::Result<Chapter> {
        let res = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Chapter> {
            let conn = inner.0.get()?;

            let row = conn.query_row(
                "SELECT C.id, C.name, C.pre, C.main, C.post, C.words, C.created, C.updated FROM story_chapter SC LEFT JOIN chapter C ON SC.chapter_id = C.Id WHERE SC.story_id = ? AND SC.place = ?;",
                rusqlite::params![story_id, chapter_number],
                |row| {
                    Ok(Chapter {
                        id: row.get(0)?,

                        name: row.get(1)?,

                        pre: row.get(2)?,
                        main: row.get(3)?,
                        post: row.get(4)?,

                        words: row.get(5)?,

                        created: row.get(6)?,
                        updated: row.get(7)?,
                    })
                },
            )?;

            Ok(row)
        }
        })
        .await??;

        Ok(res)
    }
}
