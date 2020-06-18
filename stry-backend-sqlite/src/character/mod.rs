use {
    crate::{
        utils::{SqliteExt, SqliteStmtExt, FromRow},
        SqliteBackend,
    },
    anyhow::Context,
    std::borrow::Cow,
    stry_common::{
        backend::{BackendCharacter, BackendStory},
        models::{Character, Entity, List, Story},
    },
};

impl FromRow for Character {
    fn from_row(row: &rusqlite::Row) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Character {
            id: row.get(0).context("Attempting to get row index 0 for character")?,

            name: row.get(1).context("Attempting to get row index 1 for character")?,

            created: row.get(2).context("Attempting to get row index 2 for character")?,
            updated: row.get(3).context("Attempting to get row index 3 for character")?,
        })
    }
}

#[async_trait::async_trait]
impl BackendCharacter for SqliteBackend {
    async fn all_characters(
        &self,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Character>>> {
        let characters = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Character>>> {
                let conn = inner.0.get()?;

                let mut stmt =
                    conn.prepare(include_str!("all-items.sql"))?;

                let items = match stmt
                    .query_map_anyhow(rusqlite::params![limit, offset * limit], |row| {
                        Ok(Character {
                            id: row.get(0).context("Attempting to get row index 0 for character")?,

                            name: row.get(1).context("Attempting to get row index 1 for character")?,

                            created: row.get(2).context("Attempting to get row index 2 for character")?,
                            updated: row.get(3).context("Attempting to get row index 3 for character")?,
                        })
                    })?.map(|items| {
                        items.collect::<Result<_, _>>()
                    }) {
                        Some(items) => items?,
                        None => return Ok(None),
                    };

                let total = match conn.query_row_anyhow(
                    include_str!("all-count.sql"),
                    rusqlite::params![],
                    |row| Ok(row.get(0).context("Attempting to get row index 0 for character count")?),
                ).context("Unable to get total character count")? {
                    Some(total) => total,
                    None => return Ok(None),
                };

                Ok(Some(List { total, items }))
            }
        })
        .await??;

        Ok(characters)
    }

    async fn get_character(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Character>> {
        let res = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<Character>> {
                let conn = inner.0.get()?;

                let row = conn.query_row_anyhow(
                    include_str!("get-item.sql"),
                    rusqlite::params![id],
                    |row| {
                        Ok(Character {
                            id: row
                                .get(0)
                                .context("Attempting to get row index 0 for character")?,

                            name: row
                                .get(1)
                                .context("Attempting to get row index 1 for character")?,

                            created: row
                                .get(2)
                                .context("Attempting to get row index 2 for character")?,
                            updated: row
                                .get(3)
                                .context("Attempting to get row index 3 for character")?,
                        })
                    },
                )?;

                Ok(row)
            }
        })
        .await??;

        Ok(res)
    }

    async fn character_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        let ids = match tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Entity>>> {
                let conn = inner.0.get()?;

                let mut stmt = conn.prepare(include_str!("stories-items.sql"))?;

                let items: Vec<Entity> = match stmt.query_map_anyhow(rusqlite::params![id, limit, offset], |row| Ok(Entity {
                    id: row.get(0).context("Attempting to get row index 0 for character story")?,
                }))?.map(|items| {
                    items.collect::<Result<_, _>>()
                }) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let total = match conn.query_row_anyhow(
                    include_str!("stories-count.sql"),
                    rusqlite::params![id],
                    |row| Ok(row.get(0).context("Attempting to get row index 0 for character story count")?)
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
