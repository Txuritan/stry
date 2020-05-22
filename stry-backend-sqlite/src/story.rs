use {
    crate::SqliteBackend,
    std::borrow::Cow,
    stry_common::{
        models::{story::StoryPart, tag::TagType, Entity, List, Square, Story, Warning},
        BackendAuthor, BackendOrigin, BackendStory, BackendTag,
    },
};

#[async_trait::async_trait]
impl BackendStory for SqliteBackend {
    async fn all_stories(&mut self, offset: u32, limit: u32) -> anyhow::Result<List<Story>> {
        let mut inner = self.clone();

        let ids = tokio::task::spawn_blocking({
            let inner = inner.clone();

            move || -> anyhow::Result<List<Entity>> {
                let conn = inner.0.get()?;

                let mut stmt =
                    conn.prepare("SELECT id FROM story ORDER BY updated DESC LIMIT ? OFFSET ?;")?;

                let items = stmt
                    .query_map(rusqlite::params![limit, offset], |row| {
                        Ok(Entity { id: row.get(0)? })
                    })?
                    .collect::<Result<_, _>>()?;

                let total = conn.query_row(
                    "SELECT COUNT(id) as count FROM story;",
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
            let story = inner.get_story(id.into()).await?;

            items.push(story);
        }

        Ok(List { total, items })
    }

    async fn get_story(&mut self, id: Cow<'static, str>) -> anyhow::Result<Story> {
        type StoryParts = (StoryPart, u32, u32, Vec<Entity>, Vec<Entity>, Vec<Entity>);

        let mut inner = self.clone();

        let (story_part, chapters, words, author_entities, origin_entities, tag_entities) = tokio::task::spawn_blocking({
            let inner = inner.clone();

            move || -> anyhow::Result<StoryParts> {
                let conn = inner.0.get()?;

                let mut author_stmt = conn.prepare("SELECT A.id FROM story_author SA LEFT JOIN author A ON SA.author_id = A.id WHERE SA.story_id = ? ORDER BY A.name;")?;

                let mut origin_stmt = conn.prepare("SELECT O.id FROM story_origin SO LEFT JOIN origin O ON SO.origin_id = O.id WHERE SO.story_id = ? ORDER BY O.name;")?;

                let mut tag_stmt = conn.prepare("SELECT T.id FROM story_tag ST LEFT JOIN tag T ON ST.tag_id = T.id WHERE ST.story_id = ? ORDER BY T.name;")?;

                let story_part = conn.query_row(
                    "SELECT id, name, summary, rating, state, created, updated FROM story WHERE id = ?;",
                    rusqlite::params![id],
                    |row| {
                        Ok(StoryPart {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            summary: row.get(2)?,
                            rating: row.get(3)?,
                            state: row.get(4)?,
                            created: row.get(5)?,
                            updated: row.get(6)?,
                        })
                    },
                )?;

                let chapters = conn.query_row("SELECT COUNT(story_id) as count FROM story_chapter WHERE story_id = ?;", rusqlite::params![id], |row| Ok(row.get(0)?))?;

                let words = conn.query_row("SELECT SUM(C.words) as words FROM story_chapter SC LEFT JOIN chapter C ON C.id = SC.chapter_id WHERE SC.story_id = ?;", rusqlite::params![id], |row| Ok(row.get(0)?))?;

                let author_entities = author_stmt.query_map(rusqlite::params![id], |row| Ok(Entity {
                    id: row.get(0)?,
                }))?.collect::<Result<_, _>>()?;

                let origin_entities = origin_stmt.query_map(rusqlite::params![id], |row| Ok(Entity {
                    id: row.get(0)?,
                }))?.collect::<Result<_, _>>()?;

                let tag_entities = tag_stmt.query_map(rusqlite::params![id], |row| Ok(Entity {
                    id: row.get(0)?,
                }))?.collect::<Result<_, _>>()?;

                Ok((story_part, chapters, words, author_entities, origin_entities, tag_entities))
            }
        })
        .await??;

        let mut authors = Vec::with_capacity(author_entities.len());

        for Entity { id } in author_entities {
            let author = inner.get_author(id.into()).await?;

            authors.push(author);
        }

        let mut origins = Vec::with_capacity(origin_entities.len());

        for Entity { id } in origin_entities {
            let origin = inner.get_origin(id.into()).await?;

            origins.push(origin);
        }

        let mut tags = Vec::with_capacity(tag_entities.len());

        for Entity { id } in tag_entities {
            let tag = inner.get_tag(id.into()).await?;

            tags.push(tag);
        }

        let story = Story {
            id: story_part.id,

            name: story_part.name,
            summary: story_part.summary,

            square: Square {
                rating: story_part.rating,
                warnings: if tags.iter().any(|t| t.typ == TagType::Warning) {
                    Warning::Using
                } else {
                    Warning::None
                },
                state: story_part.state,
            },

            chapters,
            words,

            authors,
            origins,
            tags,
            // TODO
            series: None,

            created: story_part.created,
            updated: story_part.updated,
        };

        Ok(story)
    }
}
