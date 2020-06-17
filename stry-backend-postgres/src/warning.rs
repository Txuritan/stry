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
    async fn all_warnings(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Warning>>> {
        todo!()
    }

    async fn get_warning(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Warning>> {
        todo!()
    }

    async fn warning_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        todo!()
    }
}
