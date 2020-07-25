use crate::{
    backend::{BackendWorker, PostgresBackend},
    models::WorkerTask,
};

#[async_trait::async_trait]
impl BackendWorker for PostgresBackend {
    #[tracing::instrument(skip(self))]
    async fn get_new_task(&self) -> anyhow::Result<Option<WorkerTask>> {
        todo!()
    }
}
