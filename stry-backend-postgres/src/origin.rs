use {
    crate::PostgresPoolConnection,
    std::borrow::Cow,
    stry_common::{
        models::{Entity, List, Origin, Story},
        BackendOrigin,
    },
};

#[async_trait::async_trait]
impl BackendOrigin for PostgresPoolConnection {
    async fn all_origins(&mut self, offset: u32, limit: u32) -> anyhow::Result<List<Origin>> {
        todo!()
    }

    async fn get_origin(&mut self, id: Cow<'static, str>) -> anyhow::Result<Origin> {
        todo!()
    }

    async fn origin_stories(
        &mut self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>> {
        todo!()
    }
}
