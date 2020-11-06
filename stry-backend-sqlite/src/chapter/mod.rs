use {
    crate::SqliteBackend, anyhow::Context, rewryte::sqlite::ConnectionExt, std::borrow::Cow,
    stry_models::Chapter,
};

#[stry_macros::box_async]
impl SqliteBackend {
    #[tracing::instrument(level = "trace", skip(self), err)]
    pub async fn get_chapter(
        &self,
        story_id: Cow<'static, str>,
        chapter_number: i32,
    ) -> anyhow::Result<Option<Chapter>> {
        let res = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<Chapter>> {
                let conn = inner.0.get()?;

                let row: Option<Chapter> = tracing::trace_span!("get")
                    .in_scope(|| {
                        conn.type_query_one_opt(
                            include_str!("get-item.sql"),
                            rusqlite::params![story_id, chapter_number],
                        )
                    })
                    .context("Unable to get story chapter")?;

                Ok(row)
            }
        })
        .await??;

        Ok(res)
    }

    #[allow(clippy::unnecessary_operation, clippy::unit_arg)]
    #[tracing::instrument(level = "trace", skip(self, pre, main, post), err)]
    pub async fn update_chapter(
        &self,
        story_id: Cow<'static, str>,
        chapter_number: i32,
        pre: Cow<'static, str>,
        main: Cow<'static, str>,
        post: Cow<'static, str>,
    ) -> anyhow::Result<()> {
        tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<()> {
                let mut conn = inner.0.get()?;

                let trans = conn.transaction()?;

                trans.execute(
                    "UPDATE Chapter (Pre, Main, Post) SET Pre = ?, Main = ?, Post = ? WHERE ",
                    rusqlite::params![pre, main, post],
                )?;

                trans.commit()?;

                Ok(())
            }
        })
        .await??;

        Ok(())
    }
}
