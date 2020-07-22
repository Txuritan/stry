use {
    crate::PostgresBackend,
    std::borrow::Cow,
    stry_common::{
        backend::{BackendOrigin, BackendStory},
        models::{List, Origin, Story},
    },
};

#[async_trait::async_trait]
impl BackendOrigin for PostgresBackend {
    #[tracing::instrument(skip(self))]
    async fn all_origins(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Origin>>> {
        let conn = self.0.get().await?;

        let stmt = conn
            .prepare("SELECT id FROM origin ORDER BY name DESC LIMIT $1 OFFSET $2;")
            .await?;

        let id_rows = conn.query(&stmt, &[&limit, &offset]).await?;

        let mut items = Vec::with_capacity(id_rows.len());

        for id_row in id_rows {
            let id: String = id_row.try_get(0)?;

            let origin = match self.get_origin(id.into()).await? {
                Some(origin) => origin,
                None => return Ok(None),
            };

            items.push(origin);
        }

        let total = conn
            .query_one("SELECT COUNT(id) as count FROM origin;", &[])
            .await?;

        let list = List {
            total: total.try_get(0)?,
            items,
        };

        Ok(Some(list))
    }

    #[tracing::instrument(skip(self))]
    async fn get_origin(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Origin>> {
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

        Ok(Some(origin))
    }

    #[tracing::instrument(skip(self))]
    async fn origin_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        let conn = self.0.get().await?;

        let stmt = conn
            .prepare("SELECT SO.story_id FROM story_origin SO LEFT JOIN story S ON S.id = SO.story_id WHERE SO.origin_id = $1 ORDER BY S.updated DESC LIMIT $2 OFFSET $3;")
            .await?;

        let id_rows = conn.query(&stmt, &[&limit, &offset]).await?;

        let mut items = Vec::with_capacity(id_rows.len());

        for id_row in id_rows {
            let id: String = id_row.try_get(0)?;

            let story = match self.get_story(id.into()).await? {
                Some(story) => story,
                None => return Ok(None),
            };

            items.push(story);
        }

        let total = conn
            .query_one("SELECT COUNT(SO.story_id) as id FROM story_origin SO LEFT JOIN story S ON S.id = SO.story_id WHERE SO.origin_id = $1;", &[&id])
            .await?;

        let list = List {
            total: total.try_get(0)?,
            items,
        };

        Ok(Some(list))
    }
}
