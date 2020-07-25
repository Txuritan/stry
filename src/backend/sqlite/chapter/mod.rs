use {
    crate::{
        backend::{
            sqlite::utils::{FromRow, SqliteExt},
            BackendChapter, SqliteBackend,
        },
        models::Chapter,
    },
    anyhow::Context,
    std::borrow::Cow,
};

impl FromRow for Chapter {
    fn from_row(row: &rusqlite::Row) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Chapter {
            id: row
                .get(0)
                .context("Attempting to get row index 0 for chapter")?,

            name: row
                .get(1)
                .context("Attempting to get row index 1 for chapter")?,

            pre: row
                .get(2)
                .context("Attempting to get row index 2 for chapter")?,
            main: row
                .get(3)
                .context("Attempting to get row index 3 for chapter")?,
            post: row
                .get(4)
                .context("Attempting to get row index 4 for chapter")?,

            words: row
                .get(5)
                .context("Attempting to get row index 5 for chapter")?,

            created: row
                .get(6)
                .context("Attempting to get row index 6 for chapter")?,
            updated: row
                .get(7)
                .context("Attempting to get row index 7 for chapter")?,
        })
    }
}

#[async_trait::async_trait]
impl BackendChapter for SqliteBackend {
    #[tracing::instrument(level = "debug", skip(self))]
    async fn get_chapter(
        &self,
        story_id: Cow<'static, str>,
        chapter_number: u32,
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
