use {
    crate::{
        backend::{
            BackendAuthor, BackendCharacter, BackendOrigin, BackendTag, BackendWarning, DataBackend,
        },
        frontend::user::{
            pages::{ErrorPage, StoryList},
            utils::{wrap, Items},
        },
        models::Paging,
    },
    anyhow::Context,
    askama::Template,
    std::borrow::Cow,
    warp::{Rejection, Reply},
};

pub async fn item(
    item: Items,
    id: String,
    paging: Paging,
    backend: DataBackend,
) -> Result<impl Reply, Rejection> {
    wrap(move || async move {
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
            Items::Friends => None,
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
            Items::Pairings => None,
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
                    url,
                    paging.page,
                    (total + (norm.page_size - 1)) / norm.page_size,
                    items,
                )
                .render()
                .context("Unable to render item page")?;

                Ok(rendered)
            }
            None => {
                let rendered = ErrorPage::not_found("404 not found").render()?;

                Ok(rendered)
            }
        }
    })
    .await
}
