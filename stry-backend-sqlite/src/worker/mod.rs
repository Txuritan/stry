use {
    stry_common::{backend::BackendWorker, models::WorkerTask},
    crate::{SqliteBackend, utils::{FromRow, SqliteExt}},
    anyhow::Context,
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

            site: row
                .get(1)
                .context("Attempting to get row index 1 for worker task")?,
            url: row
                .get(2)
                .context("Attempting to get row index 1 for worker task")?,
            chapter: row
                .get(3)
                .context("Attempting to get row index 3 for worker task")?,

            complete: row
                .get(4)
                .context("Attempting to get row index 4 for worker task")?,

            created: row
                .get(5)
                .context("Attempting to get row index 5 for worker task")?,
            updated: row
                .get(6)
                .context("Attempting to get row index 6 for worker task")?,
        })
    }
}

#[async_trait::async_trait]
impl BackendWorker for SqliteBackend {
    async fn get_new_task(&self) -> anyhow::Result<Option<WorkerTask>> {
        let task = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<WorkerTask>> {
                let conn = inner.0.get()?;

                let task: WorkerTask = match conn.type_query_row_anyhow(
                    "SELECT WT.Id, WT.Site, WT.Url, WT.Chapter, WT.Complete, WT.Created, WT.Updated FROM WorkerTask WT WHERE WT.Complete = TRUE AND WT.Id IS NOT IN (SELECT Task FROM Worker) LIMIT 1",
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
