use {
    crate::{
        utils::{ClientExt, FromRow},
        PostgresBackend,
    },
    anyhow::Context,
    futures::try_join,
    std::borrow::Cow,
    stry_common::models::{Author, Entity, List, Story},
};

impl FromRow for Author {
    fn from_row(row: &tokio_postgres::Row) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Author {
            id: row
                .try_get(0)
                .context("Attempting to get row index 0 for author")?,

            name: row
                .try_get(1)
                .context("Attempting to get row index 1 for author")?,

            created: row
                .try_get(2)
                .context("Attempting to get row index 2 for author")?,
            updated: row
                .try_get(3)
                .context("Attempting to get row index 3 for author")?,
        })
    }
}

#[stry_macros::box_async]
impl PostgresBackend {
    #[tracing::instrument(skip(self), err)]
    pub async fn all_authors(
        &self,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Author>>> {
        let conn = self.0.get().await?;

        let pair = crate::params![limit, offset];
        let empty = crate::params![];

        let (items, total): (Option<Vec<Author>>, Option<i32>) = try_join!(
            conn.type_query_map_anyhow(include_str!("all-items.sql"), pair),
            conn.type_query_row_anyhow(include_str!("all-count.sql"), empty),
        )?;

        let list = List {
            total: crate::opt_try!(total),
            items: crate::opt_try!(items),
        };

        Ok(Some(list))
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn get_author(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Author>> {
        let conn = self.0.get().await?;

        let author = conn
            .type_query_row_anyhow(include_str!("get-item.sql"), crate::params![id])
            .await?;

        Ok(author)
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn author_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        let conn = self.0.get().await?;

        let pair = crate::params![limit, offset];
        let one = crate::params![id];

        let (entity_rows, total): (Option<Vec<Entity>>, Option<i32>) = try_join!(
            conn.type_query_map_anyhow(include_str!("stories-items.sql"), pair),
            conn.type_query_row_anyhow(include_str!("stories-count.sql"), one),
        )?;

        let entity_rows = crate::opt_try!(entity_rows);

        let mut items = Vec::with_capacity(entity_rows.len());

        for Entity { id } in entity_rows {
            let story = match self.get_story(id.into()).await? {
                Some(story) => story,
                None => return Ok(None),
            };

            items.push(story);
        }

        let list = List {
            total: crate::opt_try!(total),
            items,
        };

        Ok(Some(list))
    }
}
