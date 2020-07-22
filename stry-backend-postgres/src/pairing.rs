use {
    crate::PostgresBackend,
    std::borrow::Cow,
    stry_common::{
        backend::{BackendPairing, BackendStory},
        models::{List, Pairing, Story},
    },
};

#[async_trait::async_trait]
impl BackendPairing for PostgresBackend {
    #[tracing::instrument(skip(self))]
    async fn all_pairings(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Pairing>>> {
        todo!()
    }

    #[tracing::instrument(skip(self))]
    async fn get_pairing(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Pairing>> {
        todo!()
    }

    #[tracing::instrument(skip(self))]
    async fn pairing_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        todo!()
    }
}
