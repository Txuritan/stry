use {
    crate::PostgresBackend,
    std::borrow::Cow,
    stry_models::{List, Pairing, Story},
};

/// Handles any and all queries that deal with Pairing and their Stories.
#[cfg_attr(feature = "boxed-futures", stry_macros::box_async)]
impl PostgresBackend {
    #[tracing::instrument(skip(self), err)]
    pub async fn all_pairings(
        &self,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Pairing>>> {
        todo!()
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn get_pairing(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Pairing>> {
        todo!()
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn pairing_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        todo!()
    }
}
