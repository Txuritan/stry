use {crate::PostgresBackend, std::borrow::Cow, stry_models::Chapter};

/// Handles any and all queries that deal with a Story's Chapters.
#[cfg_attr(feature = "boxed-futures", stry_macros::box_async)]
impl PostgresBackend {
    #[tracing::instrument(skip(self), err)]
    pub async fn get_chapter(
        &self,
        story_id: Cow<'static, str>,
        chapter_number: i32,
    ) -> anyhow::Result<Option<Chapter>> {
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

        Ok(Some(chapter))
    }

    #[allow(clippy::unnecessary_operation, clippy::unit_arg)]
    #[tracing::instrument(level = "trace", skip(self, _pre, _main, _post), err)]
    pub async fn update_chapter(
        &self,
        story_id: Cow<'static, str>,
        chapter_number: i32,
        _pre: Cow<'static, str>,
        _main: Cow<'static, str>,
        _post: Cow<'static, str>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
