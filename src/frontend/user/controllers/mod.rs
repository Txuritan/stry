pub mod assets;
pub mod dashboard;
pub mod explore;
pub mod item;
pub mod search;
pub mod story;

use {
    crate::{
        backend::{BackendStory, DataBackend},
        frontend::user::{pages::StoryList, utils::wrap},
        models::{Paging},
    },
    askama::Template,
    chrono::Utc,
    warp::{Rejection, Reply},
};

#[warp_macros::get("/")]
pub async fn index(#[data] backend: DataBackend, #[query] paging: Paging) -> Result<impl Reply, Rejection> {
    wrap(move || async move {
        let time = Utc::now();

        let norm = paging.normalize();

        match backend.all_stories(norm.page, paging.page_size).await? {
            Some(list) => {
                let (total, items) = list.into_parts();

                let rendered = StoryList::new(
                    "home",
                    time,
                    "/",
                    paging.page,
                    (total + (paging.page_size - 1)) / paging.page_size,
                    items,
                )
                .render()?;

                Ok(rendered)
            }
            None => {
                // TODO: return empty home page
                todo!()
            }
        }
    })
    .await
}