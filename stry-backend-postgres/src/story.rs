use {
    crate::PostgresPoolConnection,
    std::borrow::Cow,
    stry_common::{
        models::{story::StoryPart, List, Square, Story, TagType, Warning},
        BackendAuthor, BackendOrigin, BackendStory, BackendTag,
    },
};

#[async_trait::async_trait]
impl BackendStory for PostgresPoolConnection {
    async fn all_stories(&mut self, offset: u32, limit: u32) -> anyhow::Result<List<Story>> {
        let inner = self.clone();
        let conn = inner.0.get().await?;

        let stmt = conn
            .prepare("SELECT id FROM story ORDER BY name DESC LIMIT $1 OFFSET $2;")
            .await?;

        let id_rows = conn.query(&stmt, &[&limit, &offset]).await?;

        let mut items = Vec::with_capacity(id_rows.len());

        for id_row in id_rows {
            let id: String = id_row.try_get(0)?;

            let story = self.get_story(id.into()).await?;

            items.push(story);
        }

        let total = conn
            .query_one("SELECT COUNT(id) as count FROM story;", &[])
            .await?;

        let list = List {
            total: total.try_get(0)?,
            items,
        };

        Ok(list)
    }

    async fn get_story(&mut self, id: Cow<'static, str>) -> anyhow::Result<Story> {
        let inner = self.clone();
        let conn = inner.0.get().await?;

        let author_stmt = conn
            .prepare("SELECT A.id FROM story_author SA LEFT JOIN author A ON SA.author_id = A.id WHERE SA.story_id = $1 ORDER BY A.name;")
            .await?;

        let origin_stmt = conn
            .prepare("SELECT O.id FROM story_origin SO LEFT JOIN origin O ON SO.origin_id = O.id WHERE SO.story_id = $1 ORDER BY O.name;")
            .await?;

        let tag_stmt = conn
            .prepare("SELECT T.id FROM story_tag ST LEFT JOIN tag T ON ST.tag_id = T.id WHERE ST.story_id = $1 ORDER BY T.name;")
            .await?;

        let row = conn
            .query_one("SELECT id, name, summary, rating, state, created, updated FROM story WHERE id = $1;", &[&id])
            .await?;

        let story_part = StoryPart {
            id: row.try_get(0)?,

            name: row.try_get(1)?,

            summary: row.try_get(2)?,

            rating: row.try_get(3)?,
            state: row.try_get(4)?,

            created: row.try_get(5)?,
            updated: row.try_get(6)?,
        };

        let chapters = conn
            .query_one(
                "SELECT COUNT(story_id) as count FROM story_chapter WHERE story_id = ?;",
                &[&id],
            )
            .await?
            .try_get(0)?;

        let words = conn
            .query_one("SELECT SUM(C.words) as words FROM story_chapter SC LEFT JOIN chapter C ON C.id = SC.chapter_id WHERE SC.story_id = ?;", &[&id])
            .await?
            .try_get(0)?;

        let author_rows = conn.query(&author_stmt, &[&id]).await?;

        let mut authors = Vec::with_capacity(author_rows.len());

        for id_row in author_rows {
            let id: String = id_row.try_get(0)?;

            let author = self.get_author(id.into()).await?;

            authors.push(author);
        }

        let origin_rows = conn.query(&author_stmt, &[&id]).await?;

        let mut origins = Vec::with_capacity(origin_rows.len());

        for id_row in origin_rows {
            let id: String = id_row.try_get(0)?;

            let origin = self.get_origin(id.into()).await?;

            origins.push(origin);
        }

        let tag_rows = conn.query(&author_stmt, &[&id]).await?;

        let mut tags = Vec::with_capacity(tag_rows.len());

        for id_row in tag_rows {
            let id: String = id_row.try_get(0)?;

            let tag = self.get_tag(id.into()).await?;

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
