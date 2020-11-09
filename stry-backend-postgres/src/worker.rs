use {crate::PostgresBackend, stry_models::WorkerTask};

/// Handles any and all queries that deal with Workers.
#[cfg_attr(feature = "boxed-futures", stry_macros::box_async)]
impl PostgresBackend {
    #[tracing::instrument(skip(self), err)]
    pub async fn get_new_task(&self) -> anyhow::Result<Option<WorkerTask>> {
        todo!()
    }
}
