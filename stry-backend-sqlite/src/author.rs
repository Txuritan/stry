use {
    crate::{
        utils::{FromRow, SqliteExt, SqliteStmtExt},
        SqliteBackend,
    },
    anyhow::Context,
    std::borrow::Cow,
    stry_common::{
        backend::{BackendAuthor, BackendStory},
        models::{Author, Entity, List, Story},
    },
};

impl FromRow for Author {
    fn from_row(row: &rusqlite::Row) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Author {
            id: row
                .get(0)
                .context("Attempting to get row index 0 for author")?,

            name: row
                .get(1)
                .context("Attempting to get row index 1 for author")?,

            created: row
                .get(2)
                .context("Attempting to get row index 2 for author")?,
            updated: row
                .get(3)
                .context("Attempting to get row index 3 for author")?,
        })
    }
}

#[async_trait::async_trait]
impl BackendAuthor for SqliteBackend {
    async fn all_authors(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Author>>> {
        let authors = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Author>>> {
                let conn = inner.0.get()?;

                let mut stmt =
                    conn.prepare("SELECT Id, Name, Created, Updated FROM Author ORDER BY Name ASC LIMIT ? OFFSET ?;")?;

                let items = match stmt
                    .query_map_anyhow(rusqlite::params![limit, offset * limit], |row| {
                        Ok(Author {
                            id: row.get(0).context("Attempting to get row index 0 for author")?,

                            name: row.get(1).context("Attempting to get row index 1 for author")?,

                            created: row.get(2).context("Attempting to get row index 2 for author")?,
                            updated: row.get(3).context("Attempting to get row index 3 for author")?,
                        })
                    })?
                    .map(|items| {
                        items.collect::<Result<_, _>>()
                    }) {
                        Some(items) => items?,
                        None => return Ok(None),
                    };

                let total = match conn.query_row_anyhow(
                    "SELECT COUNT(id) as Count FROM Author;",
                    rusqlite::params![],
                    |row| Ok(row.get(0).context("Attempting to get row index 0 for author count")?),
                )? {
                    Some(total) => total,
                    None => return Ok(None),
                };

                Ok(Some(List { total, items }))
            }
        })
        .await??;

        Ok(authors)
    }

    async fn get_author(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Author>> {
        let res = tokio::task::spawn_blocking({
            let inner = self.clone();
            move || -> anyhow::Result<Option<Author>> {
                let conn = inner.0.get()?;

                let row = conn.query_row_anyhow(
                    "SELECT Id, Name, Created, Updated FROM Author WHERE Id = ?;",
                    rusqlite::params![id],
                    |row| {
                        Ok(Author {
                            id: row
                                .get(0)
                                .context("Attempting to get row index 0 for author")?,

                            name: row
                                .get(1)
                                .context("Attempting to get row index 1 for author")?,

                            created: row
                                .get(2)
                                .context("Attempting to get row index 2 for author")?,
                            updated: row
                                .get(3)
                                .context("Attempting to get row index 3 for author")?,
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
    ) -> anyhow::Result<Option<List<Story>>> {
        let ids = match tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Entity>>> {
                let conn = inner.0.get()?;

                let mut stmt = conn.prepare("SELECT SA.StoryId FROM StoryAuthor SA LEFT JOIN Story S ON S.id = SA.StoryId WHERE SA.AuthorId = ? ORDER BY S.Updated DESC LIMIT ? OFFSET ?;")?;

                let items: Vec<Entity> = match stmt.query_map_anyhow(rusqlite::params![id, limit, offset], |row| Ok(Entity {
                    id: row.get(0).context("Attempting to get row index 0 for author story id")?,
                }))?.map(|items| {
                    items.collect::<Result<_, _>>()
                }) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let total = match conn.query_row_anyhow(
                    "SELECT COUNT(SA.StoryId) as Id FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = ?;",
                    rusqlite::params![id],
                    |row| Ok(row.get(0).context("Attempting to get row index 0 for author story count")?)
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
