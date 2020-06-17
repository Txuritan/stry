use {
    crate::{pages, utils::wrap},
    askama::Template,
    stry_backend::DataBackend,
    stry_common::{
        backend::BackendStory,
        models::{Paging, Search},
    },
    warp::{Rejection, Reply},
};

pub async fn index(
    paging: Paging,
    search: Search,
    pool: DataBackend,
) -> Result<impl Reply, Rejection> {
    wrap(move || async move {
        let norm = paging.normalize();

        match pool
            .search_stories(search.search.clone().into(), norm.page, norm.page_size)
            .await?
        {
            Some(stories) => {
                let (total, items) = stories.into_parts();

                let page = pages::Search::new(
                    search.search.clone(),
                    search.search,
                    paging.page,
                    total / norm.page_size,
                    items,
                )?;

                let rendered: String = page.render()?;

                Ok(rendered)
            }
            None => {
                // TODO: return no stories found search page
                Ok(String::new())
            }
        }
    })
    .await
}
