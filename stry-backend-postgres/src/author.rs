use {
    crate::PostgresBackend,
    std::borrow::Cow,
    stry_common::{
        models::{Author, List, Story},
        BackendAuthor, BackendStory,
    },
};

#[async_trait::async_trait]
impl BackendAuthor for PostgresBackend {
    async fn all_authors(&self, offset: u32, limit: u32) -> anyhow::Result<List<Author>> {
        let conn = self.0.get().await?;

        let stmt = conn
            .prepare("SELECT id FROM author ORDER BY name DESC LIMIT $1 OFFSET $2;")
            .await?;

        let id_rows = conn.query(&stmt, &[&limit, &offset]).await?;

        let mut items = Vec::with_capacity(id_rows.len());

        for id_row in id_rows {
            let id: String = id_row.try_get(0)?;

            let author = self.get_author(id.into()).await?;

            items.push(author);
        }

        let total = conn
            .query_one("SELECT COUNT(id) as count FROM author;", &[])
            .await?;

        let list = List {
            total: total.try_get(0)?,
            items,
        };

        Ok(list)
    }

    async fn get_author(&self, id: Cow<'static, str>) -> anyhow::Result<Author> {
        let conn = self.0.get().await?;

        let row = conn
            .query_one(
                "SELECT id, name, created, updated FROM author WHERE id = $1;",
                &[&id],
            )
            .await?;

        let author = Author {
            id: row.try_get(0)?,

            name: row.try_get(1)?,

            created: row.try_get(2)?,
            updated: row.try_get(3)?,
        };

        Ok(author)
    }

    async fn author_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>> {
        let conn = self.0.get().await?;

        let stmt = conn
            .prepare("SELECT SA.story_id FROM story_author SA LEFT JOIN story S ON S.id = SA.story_id WHERE SA.author_id = $1 ORDER BY S.updated DESC LIMIT $2 OFFSET $3;")
            .await?;

        let id_rows = conn.query(&stmt, &[&limit, &offset]).await?;

        let mut items = Vec::with_capacity(id_rows.len());

        for id_row in id_rows {
            let id: String = id_row.try_get(0)?;

            let story = self.get_story(id.into()).await?;

            items.push(story);
        }

        let total = conn
            .query_one("SELECT COUNT(SA.story_id) as id FROM story_author SA LEFT JOIN story S ON S.id = SA.story_id WHERE SA.author_id = $1;", &[&id])
            .await?;

        let list = List {
            total: total.try_get(0)?,
            items,
        };

        Ok(list)
    }
}
