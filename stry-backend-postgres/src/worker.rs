use {
    crate::PostgresBackend,
    stry_common::{backend::BackendWorker, models::WorkerTask},
};

#[async_trait::async_trait]
impl BackendWorker for PostgresBackend {
    #[tracing::instrument(skip(self), err)]
    async fn get_new_task(&self) -> anyhow::Result<Option<WorkerTask>> {
        todo!()
    }
}
