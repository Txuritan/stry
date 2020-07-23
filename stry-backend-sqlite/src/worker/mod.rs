use {
    crate::{
        utils::{FromRow, SqliteExt},
        SqliteBackend,
    },
    anyhow::Context,
    stry_common::{backend::BackendWorker, models::WorkerTask},
};

impl FromRow for WorkerTask {
    fn from_row(row: &rusqlite::Row<'_>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(WorkerTask {
            id: row
                .get(0)
                .context("Attempting to get row index 0 for worker task")?,

            name: row
                .get(1)
                .context("Attempting to get row index 1 for worker task")?,
            site: row
                .get(2)
                .map(
                    |sites: crate::sync::Sites| -> stry_common::models::sync::Sites {
                        sites.into()
                    },
                )
                .context("Attempting to get row index 2 for worker task")?,
            url: row
                .get(3)
                .context("Attempting to get row index 3 for worker task")?,

            chapter: row
                .get(4)
                .context("Attempting to get row index 4 for worker task")?,
            chapters: row
                .get(5)
                .context("Attempting to get row index 5 for worker task")?,
            next: row
                .get(6)
                .context("Attempting to get row index 6 for worker task")?,

            completed: row
                .get(7)
                .context("Attempting to get row index 7 for worker task")?,

            created: row
                .get(8)
                .context("Attempting to get row index 8 for worker task")?,
            updated: row
                .get(9)
                .context("Attempting to get row index 9 for worker task")?,
        })
    }
}

#[async_trait::async_trait]
impl BackendWorker for SqliteBackend {
    #[tracing::instrument(level = "debug", skip(self))]
    async fn get_new_task(&self) -> anyhow::Result<Option<WorkerTask>> {
        let task = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<WorkerTask>> {
                let conn = inner.0.get()?;

                let task: WorkerTask = match conn.type_query_row_anyhow(
                    "SELECT WT.Id, WT.Name, WT.Site, WT.Url, WT.Chapter, WT.Chapters, WT.Next, WT.Completed, WT.Created, WT.Updated FROM WorkerTask WT WHERE WT.Complete = TRUE AND WT.Id IS NOT IN (SELECT Task FROM Worker) LIMIT 1",
                    rusqlite::params![],
                )? {
                    Some(task) => task,
                    None => return Ok(None),
                };

                Ok(Some(task))
            }
        }).await??;

        Ok(task)
    }
}
