use {
    crate::SqliteBackend, anyhow::Context, rewryte::sqlite::SqliteExt, std::borrow::Cow,
    stry_common::backend::BackendChapter, stry_common::models::Chapter,
};

#[async_trait::async_trait]
impl BackendChapter for SqliteBackend {
    #[tracing::instrument(level = "trace", skip(self), err)]
    async fn get_chapter(
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
                        conn.type_query_row_anyhow(
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
}
