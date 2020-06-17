use {
    crate::{
        utils::{SqliteExt, SqliteStmtExt},
        SqliteBackend,
    },
    anyhow::Context,
    std::borrow::Cow,
    stry_common::{
        backend::{BackendOrigin, BackendStory},
        models::{Entity, List, Origin, Story},
    },
};

#[async_trait::async_trait]
impl BackendOrigin for SqliteBackend {
    async fn all_origins(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Origin>>> {
        let origins = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Origin>>> {
                let conn = inner.0.get()?;

                let mut stmt =
                    conn.prepare("SELECT Id, Name, Created, Updated FROM Origin ORDER BY Name ASC LIMIT ? OFFSET ?;")?;

                let items = match stmt
                    .query_map_anyhow(rusqlite::params![limit, offset * limit], |row| {
                        Ok(Origin {
                            id: row.get(0).context("Attempting to get row index 0 for origin")?,

                            name: row.get(1).context("Attempting to get row index 1 for origin")?,

                            created: row.get(2).context("Attempting to get row index 2 for origin")?,
                            updated: row.get(3).context("Attempting to get row index 3 for origin")?,
                        })
                    })?.map(|tags| {
                        tags.collect::<Result<_, _>>()
                    }) {
                        Some(items) => items?,
                        None => return Ok(None),
                    };

                let total = match conn.query_row_anyhow(
                    "SELECT COUNT(Id) as Count FROM Origin;",
                    rusqlite::params![],
                    |row| Ok(row.get(0).context("Attempting to get row index 0 for origin count")?),
                )? {
                    Some(total) => total,
                    None => return Ok(None),
                };

                Ok(Some(List { total, items }))
            }
        })
        .await??;

        Ok(origins)
    }

    async fn get_origin(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Origin>> {
        let res = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<Origin>> {
                let conn = inner.0.get()?;

                let row = conn.query_row_anyhow(
                    "SELECT Id, Name, Created, Updated FROM Origin WHERE Id = ?;",
                    rusqlite::params![id],
                    |row| {
                        Ok(Origin {
                            id: row
                                .get(0)
                                .context("Attempting to get row index 0 for origin")?,

                            name: row
                                .get(1)
                                .context("Attempting to get row index 1 for origin")?,

                            created: row
                                .get(2)
                                .context("Attempting to get row index 2 for origin")?,
                            updated: row
                                .get(3)
                                .context("Attempting to get row index 3 for origin")?,
                        })
                    },
                )?;

                Ok(row)
            }
        })
        .await??;

        Ok(res)
    }

    async fn origin_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        let ids = match tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Entity>>> {
                let conn = inner.0.get()?;

                let mut stmt = conn.prepare("SELECT SO.StoryId FROM StoryOrigin SO LEFT JOIN Story S ON S.Id = SO.StoryId WHERE SO.OriginId = ? ORDER BY S.Updated DESC LIMIT ? OFFSET ?;")?;

                let items: Vec<Entity> = match stmt.query_map_anyhow(rusqlite::params![id, limit, offset], |row| Ok(Entity {
                    id: row.get(0).context("Attempting to get row index 0 for origin story id")?,
                }))?.map(|items| {
                    items.collect::<Result<_, _>>()
                }) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let total = match conn.query_row_anyhow(
                    "SELECT COUNT(SO.StoryId) as Id FROM StoryOrigin SO LEFT JOIN Story S ON S.Id = SO.StoryId WHERE SO.OriginId = ?;",
                    rusqlite::params![id],
                    |row| Ok(row.get(0).context("Attempting to get row index 0 for origin story count")?),
                )? {
                    Some(total) => total,
                    None => return Ok(None),
                };

                Ok(Some(List {
                    total,
                    items,
                }))
            }
        }).await?? {
            Some(ids) => ids,
            None => return Ok(None),
        };

        let (total, entities) = ids.into_parts();

        let mut items = Vec::with_capacity(limit as usize);

        for Entity { id } in entities {
            let story = match self.get_story(id.into()).await? {
                Some(story) => story,
                None => return Ok(None),
            };

            items.push(story);
        }

        Ok(Some(List { total, items }))
    }
}
