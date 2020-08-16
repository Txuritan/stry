use {
    crate::{
        backend::{BackendWorker, SqliteBackend},
        models::WorkerTask,
    },
    rewryte::sqlite::SqliteExt,
};

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
