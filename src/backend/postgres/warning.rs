use {
    crate::{
        backend::{BackendWarning, PostgresBackend},
        models::{List, Story, Warning},
    },
    std::borrow::Cow,
};

#[async_trait::async_trait]
impl BackendWarning for PostgresBackend {
    #[tracing::instrument(skip(self))]
    async fn all_warnings(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Warning>>> {
        todo!()
    }

    #[tracing::instrument(skip(self))]
    async fn get_warning(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Warning>> {
        todo!()
    }

    #[tracing::instrument(skip(self))]
    async fn warning_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        todo!()
    }
}
