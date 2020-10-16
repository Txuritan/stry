use {
    crate::PostgresBackend,
    anyhow::Context,
    futures::try_join,
    rewryte::postgres::{ClientExt, FromRow},
    std::borrow::Cow,
    stry_models::{Author, Entity, List, Story},
};

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
            conn.type_query_opt(include_str!("all-items.sql"), pair),
            conn.type_query_one_opt(include_str!("all-count.sql"), empty),
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
            .type_query_one_opt(include_str!("get-item.sql"), crate::params![id])
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

        let (story_ids, total): (Option<Vec<String>>, Option<i32>) = try_join!(
            conn.type_query_opt(include_str!("stories-items.sql"), pair),
            conn.type_query_one_opt(include_str!("stories-count.sql"), one),
        )?;

        let story_ids = crate::opt_try!(story_ids);

        let mut items = Vec::with_capacity(story_ids.len());

        for id in story_ids {
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
