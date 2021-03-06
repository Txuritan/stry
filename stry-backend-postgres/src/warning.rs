use {
    crate::PostgresBackend,
    std::borrow::Cow,
    stry_models::{List, Story, Warning},
};

/// Handles any and all queries that deal with Warnings and their Stories.
#[cfg_attr(feature = "boxed-futures", stry_macros::box_async)]
impl PostgresBackend {
    #[tracing::instrument(skip(self), err)]
    pub async fn all_warnings(
        &self,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Warning>>> {
        todo!()
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn get_warning(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Warning>> {
        todo!()
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn warning_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        todo!()
    }
}
