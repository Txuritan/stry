pub mod assets;
pub mod dashboard;
pub mod edit;
pub mod explore;
pub mod item;
pub mod search;
pub mod story;

use {
    crate::{
        pages::StoryList,
        utils::{self, wrap},
    },
    chrono::Utc,
    stry_backend::DataBackend,
    stry_models::Paging,
    warp::{Rejection, Reply},
};

#[stry_macros::get("/")]
pub async fn index(
    #[data] backend: DataBackend,
    #[header("Accept-Language")] languages: String,
    #[query] paging: Paging,
) -> Result<impl Reply, Rejection> {
    wrap(move || async move {
        let time = Utc::now();

        let norm = paging.normalize();

        let user_lang = utils::get_languages(&languages);

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
                    user_lang,
                )
                .into_string()?;

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
