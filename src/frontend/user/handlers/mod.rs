pub mod dashboard;
mod explore;
mod item;
pub mod search;
pub mod story;

use {
    crate::{
        backend::{BackendStory, DataBackend},
        frontend::user::{pages::StoryList, utils::wrap},
        models::{Paging, RouteType},
    },
    askama::Template,
    warp::{reject::not_found, Rejection, Reply},
};

pub use crate::frontend::user::handlers::{explore::explore, item::item};

pub async fn index(paging: Paging, backend: DataBackend) -> Result<impl Reply, Rejection> {
    wrap(move || async move {
        let norm = paging.normalize();

        match backend.all_stories(norm.page, paging.page_size).await? {
            Some(list) => {
                let (total, items) = list.into_parts();

                let rendered = StoryList::new(
                    "home",
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

pub fn parse(typ: &str) -> Result<RouteType, Rejection> {
    match typ {
        "authors" => Ok(RouteType::Authors),
        "characters" => Ok(RouteType::Characters),
        "origins" => Ok(RouteType::Origins),
        "pairings" => Ok(RouteType::Pairings),
        "tags" => Ok(RouteType::Tags),
        "warnings" => Ok(RouteType::Warnings),
        _ => Err(not_found()),
    }
}
