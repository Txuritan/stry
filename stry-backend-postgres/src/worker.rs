use {crate::PostgresBackend, stry_common::models::WorkerTask};

impl PostgresBackend {
    #[tracing::instrument(skip(self), err)]
    pub async fn get_new_task(&self) -> anyhow::Result<Option<WorkerTask>> {
        todo!()
    }
}
