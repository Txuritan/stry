use {
    crate::{
        backend::{BackendPairing, PostgresBackend},
        models::{List, Pairing, Story},
    },
    std::borrow::Cow,
};

#[async_trait::async_trait]
impl BackendPairing for PostgresBackend {
    #[tracing::instrument(skip(self))]
    async fn all_pairings(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Pairing>>> {
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
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        todo!()
    }
}