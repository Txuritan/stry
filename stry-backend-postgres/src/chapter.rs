use {
    crate::PostgresBackend,
    std::borrow::Cow,
    stry_common::{models::Chapter, BackendChapter},
};

#[async_trait::async_trait]
impl BackendChapter for PostgresBackend {
    async fn get_chapter(
        &self,
        story_id: Cow<'static, str>,
        chapter_number: u32,
    ) -> anyhow::Result<Chapter> {
        let conn = self.0.get().await?;

        let row = conn
            .query_one(
                "SELECT C.id, C.name, C.pre, C.main, C.post, C.words, C.created, C.updated FROM story_chapter SC LEFT JOIN chapter C ON SC.chapter_id = C.Id WHERE SC.story_id = $1 AND SC.place = $2;",
                &[&story_id, &chapter_number],
            )
            .await?;

        let chapter = Chapter {
            id: row.try_get(0)?,

            name: row.try_get(1)?,

            pre: row.try_get(2)?,
            main: row.try_get(3)?,
            post: row.try_get(4)?,

            words: row.try_get(5)?,

            created: row.try_get(6)?,
            updated: row.try_get(7)?,
        };

        Ok(chapter)
    }
}
