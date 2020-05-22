use {
    crate::SqliteBackend,
    std::borrow::Cow,
    stry_common::{
        models::{Author, Entity, List, Story},
        BackendAuthor, BackendStory,
    },
};

#[async_trait::async_trait]
impl BackendAuthor for SqliteBackend {
    async fn all_authors(&self, offset: u32, limit: u32) -> anyhow::Result<List<Author>> {
        let ids = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<List<Entity>> {
                let conn = inner.0.get()?;

                let mut stmt =
                    conn.prepare("SELECT id FROM author ORDER BY name DESC LIMIT ? OFFSET ?;")?;

                let items = stmt
                    .query_map(rusqlite::params![limit, offset], |row| {
                        Ok(Entity { id: row.get(0)? })
                    })?
                    .collect::<Result<_, _>>()?;

                let total = conn.query_row(
                    "SELECT COUNT(id) as count FROM author;",
                    rusqlite::params![],
                    |row| Ok(row.get(0)?),
                )?;

                Ok(List { total, items })
            }
        })
        .await??;

        let (total, entities) = ids.into_parts();

        let mut items = Vec::with_capacity(limit as usize);

        for Entity { id } in entities {
            let story = self.get_author(id.into()).await?;

            items.push(story);
        }

        Ok(List { total, items })
    }

    async fn get_author(&self, id: Cow<'static, str>) -> anyhow::Result<Author> {
        let res = tokio::task::spawn_blocking({
            let inner = self.clone();
            move || -> anyhow::Result<Author> {
                let conn = inner.0.get()?;

                let row = conn.query_row(
                    "SELECT id, name, created, updated FROM author WHERE id = ?;",
                    rusqlite::params![id],
                    |row| {
                        Ok(Author {
                            id: row.get(0)?,

                            name: row.get(1)?,

                            created: row.get(2)?,
                            updated: row.get(3)?,
                        })
                    },
                )?;

                Ok(row)
            }
        })
        .await??;

        Ok(res)
    }

    async fn author_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>> {
        let ids = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<List<Entity>> {
                let conn = inner.0.get()?;

                let mut stmt = conn.prepare("SELECT SA.story_id FROM story_author SA LEFT JOIN story S ON S.id = SA.story_id WHERE SA.author_id = ? ORDER BY S.updated DESC LIMIT ? OFFSET ?;")?;

                let items: Vec<Entity> = stmt.query_map(rusqlite::params![id, limit, offset], |row| Ok(Entity {
                    id: row.get(0)?,
                }))?.collect::<Result<_, _>>()?;

                let total = conn.query_row("SELECT COUNT(SA.story_id) as id FROM story_author SA LEFT JOIN story S ON S.id = SA.story_id WHERE SA.author_id = ?;", rusqlite::params![id], |row| Ok(row.get(0)?))?;

                Ok(List {
                    total,
                    items,
                })
            }
        }).await??;

        let (total, entities) = ids.into_parts();

        let mut items = Vec::with_capacity(limit as usize);

        for Entity { id } in entities {
            let story = self.get_story(id.into()).await?;

            items.push(story);
        }

        Ok(List { total, items })
    }
}
