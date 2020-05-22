use {
    crate::PostgresPoolConnection,
    std::borrow::Cow,
    stry_common::{
        models::{List, Origin, Story},
        BackendOrigin, BackendStory,
    },
};

#[async_trait::async_trait]
impl BackendOrigin for PostgresPoolConnection {
    async fn all_origins(&mut self, offset: u32, limit: u32) -> anyhow::Result<List<Origin>> {
        let inner = self.clone();
        let conn = inner.0.get().await?;

        let stmt = conn
            .prepare("SELECT id FROM origin ORDER BY name DESC LIMIT $1 OFFSET $2;")
            .await?;

        let id_rows = conn.query(&stmt, &[&limit, &offset]).await?;

        let mut items = Vec::with_capacity(id_rows.len());

        for id_row in id_rows {
            let id: String = id_row.try_get(0)?;

            let origin = self.get_origin(id.into()).await?;

            items.push(origin);
        }

        let total = conn
            .query_one("SELECT COUNT(id) as count FROM origin;", &[])
            .await?;

        let list = List {
            total: total.try_get(0)?,
            items,
        };

        Ok(list)
    }

    async fn get_origin(&mut self, id: Cow<'static, str>) -> anyhow::Result<Origin> {
        let conn = self.0.get().await?;

        let row = conn
            .query_one(
                "SELECT id, name, created, updated FROM origin WHERE id = $1;",
                &[&id],
            )
            .await?;

        let origin = Origin {
            id: row.try_get(0)?,

            name: row.try_get(1)?,

            created: row.try_get(2)?,
            updated: row.try_get(3)?,
        };

        Ok(origin)
    }

    async fn origin_stories(
        &mut self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>> {
        let inner = self.clone();
        let conn = inner.0.get().await?;

        let stmt = conn
            .prepare("SELECT SO.story_id FROM story_origin SO LEFT JOIN story S ON S.id = SO.story_id WHERE SO.origin_id = $1 ORDER BY S.updated DESC LIMIT $2 OFFSET $3;")
            .await?;

        let id_rows = conn.query(&stmt, &[&limit, &offset]).await?;

        let mut items = Vec::with_capacity(id_rows.len());

        for id_row in id_rows {
            let id: String = id_row.try_get(0)?;

            let story = self.get_story(id.into()).await?;

            items.push(story);
        }

        let total = conn
            .query_one("SELECT COUNT(SO.story_id) as id FROM story_origin SO LEFT JOIN story S ON S.id = SO.story_id WHERE SO.origin_id = $1;", &[&id])
            .await?;

        let list = List {
            total: total.try_get(0)?,
            items,
        };

        Ok(list)
    }
}
