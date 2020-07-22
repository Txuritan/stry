use {
    crate::PostgresBackend,
    std::borrow::Cow,
    stry_common::{
        backend::{BackendStory, BackendWarning},
        models::{List, Story, Warning},
    },
};

#[async_trait::async_trait]
impl BackendWarning for PostgresBackend {
    #[tracing::instrument(skip(self))]
    async fn all_warnings(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Warning>>> {
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
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        todo!()
    }
}
