use {
    crate::{
        pages::{ErrorPage, StoryList},
        utils::{wrap, Items},
    },
    anyhow::Context,
    chrono::Utc,
    std::borrow::Cow,
    stry_backend::DataBackend,
    stry_models::Paging,
    warp::{Rejection, Reply},
};

#[warp_macros::get("/{item}/{id}")]
pub async fn item(
    #[data] backend: DataBackend,
    item: Items,
    id: String,
    #[query] paging: Paging,
) -> Result<impl Reply, Rejection> {
    wrap(move || async move {
        let time = Utc::now();

        let norm = paging.normalize();

        let id: Cow<'static, str> = id.into();

        let data = match item {
            Items::Authors => {
                let stories = backend
                    .author_stories(id.clone(), norm.page, norm.page_size)
                    .await
                    .context(format!("Unable to search backend for {}s stories", item))?;

                match stories {
                    Some(list) => {
                        let (count, stories) = list.into_parts();

                        // UNWRAP: database wouldn't return any stories it author didn't exist
                        let entity = backend.get_author(id.clone()).await?.unwrap();

                        Some((
                            format!("{} | {} | {}", paging.page, entity.name, item),
                            format!("/{}/{}", item, id),
                            count,
                            stories,
                        ))
                    }
                    None => None,
                }
            }
            Items::Characters => {
                let stories = backend
                    .character_stories(id.clone(), norm.page, norm.page_size)
                    .await
                    .context(format!("Unable to search backend for {}s stories", item))?;

                match stories {
                    Some(list) => {
                        let (count, stories) = list.into_parts();

                        // UNWRAP: database wouldn't return any stories it author didn't exist
                        let entity = backend.get_character(id.clone()).await?.unwrap();

                        Some((
                            format!("{} | {} | {}", paging.page, entity.name, item),
                            format!("/{}/{}", item, id),
                            count,
                            stories,
                        ))
                    }
                    None => None,
                }
            }
            Items::Origins => {
                let stories = backend
                    .origin_stories(id.clone(), norm.page, norm.page_size)
                    .await
                    .context(format!("Unable to search backend for {}s stories", item))?;

                match stories {
                    Some(list) => {
                        let (count, stories) = list.into_parts();

                        // UNWRAP: database wouldn't return any stories it author didn't exist
                        let entity = backend.get_origin(id.clone()).await?.unwrap();

                        Some((
                            format!("{} | {} | {}", paging.page, entity.name, item),
                            format!("/{}/{}", item, id),
                            count,
                            stories,
                        ))
                    }
                    None => None,
                }
            }
            Items::Pairings => {
                let stories = backend
                    .pairing_stories(id.clone(), norm.page, norm.page_size)
                    .await
                    .context(format!("Unable to search backend for {}s stories", item))?;

                match stories {
                    Some(list) => {
                        let (count, stories) = list.into_parts();

                        // UNWRAP: database wouldn't return any stories it author didn't exist
                        let entity = backend.get_pairing(id.clone()).await?.unwrap();

                        Some((
                            format!(
                                "{} | {} | {}",
                                paging.page,
                                entity
                                    .characters
                                    .iter()
                                    .map(|c| &*c.name)
                                    .collect::<Vec<&str>>()
                                    .join(if entity.platonic { "&" } else { "/" }),
                                item
                            ),
                            format!("/{}/{}", item, id),
                            count,
                            stories,
                        ))
                    }
                    None => None,
                }
            }
            Items::Tags => {
                let stories = backend
                    .tag_stories(id.clone(), norm.page, norm.page_size)
                    .await
                    .context(format!("Unable to search backend for {}s stories", item))?;

                match stories {
                    Some(list) => {
                        let (count, stories) = list.into_parts();

                        // UNWRAP: database wouldn't return any stories it author didn't exist
                        let entity = backend.get_tag(id.clone()).await?.unwrap();

                        Some((
                            format!("{} | {} | {}", paging.page, entity.name, item),
                            format!("/{}/{}", item, id),
                            count,
                            stories,
                        ))
                    }
                    None => None,
                }
            }
            Items::Warnings => {
                let stories = backend
                    .warning_stories(id.clone(), norm.page, norm.page_size)
                    .await
                    .context(format!("Unable to search backend for {}s stories", item))?;

                match stories {
                    Some(list) => {
                        let (count, stories) = list.into_parts();

                        // UNWRAP: database wouldn't return any stories it author didn't exist
                        let entity = backend.get_warning(id.clone()).await?.unwrap();

                        Some((
                            format!("{} | {} | {}", paging.page, entity.name, item),
                            format!("/{}/{}", item, id),
                            count,
                            stories,
                        ))
                    }
                    None => None,
                }
            }
        };

        match data {
            Some((title, url, total, items)) => {
                let rendered: String = StoryList::new(
                    title,
                    time,
                    url,
                    paging.page,
                    (total + (norm.page_size - 1)) / norm.page_size,
                    items,
                )
                .into_string()
                .context("Unable to render item page")?;

                Ok(rendered)
            }
            None => {
                let rendered = ErrorPage::not_found("404 not found", time).into_string()?;

                Ok(rendered)
            }
        }
    })
    .await
}
