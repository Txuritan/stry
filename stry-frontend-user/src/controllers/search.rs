use {
    crate::{pages, utils::wrap},
    chrono::Utc,
    stry_backend::DataBackend,
    stry_models::{Paging, Search},
    warp::{Rejection, Reply},
};

#[warp_macros::get("/search")]
pub async fn index(
    #[data] backend: DataBackend,
    #[query] paging: Paging,
    #[query] search: Search,
) -> Result<impl Reply, Rejection> {
    wrap(move || async move {
        let time = Utc::now();

        let norm = paging.normalize();

        match backend
            .search_stories(search.search.clone().into(), norm.page, norm.page_size)
            .await?
        {
            Some(stories) => {
                let (total, items) = stories.into_parts();

                let page = pages::Search::new(
                    search.search.clone(),
                    search.search,
                    time,
                    paging.page,
                    total / norm.page_size,
                    items,
                )?;

                let rendered: String = page.into_string()?;

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
