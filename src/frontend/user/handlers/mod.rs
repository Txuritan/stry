pub mod dashboard;
// pub mod explore;
// pub mod item;
pub mod search;
pub mod story;

use {
    crate::{
        backend::{
            BackendAuthor, BackendCharacter, BackendOrigin, BackendStory, BackendTag,
            BackendWarning, DataBackend,
        },
        frontend::user::{
            pages::{ErrorPage, ResourceList, StoryList},
            utils::wrap,
        },
        models::{Paging, Resource, RouteType},
    },
    anyhow::Context,
    askama::Template,
    std::borrow::Cow,
    warp::{reject::not_found, Rejection, Reply},
};

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

pub async fn explore(
    typ: String,
    paging: Paging,
    backend: DataBackend,
) -> Result<impl Reply, Rejection> {
    let rt = parse(&typ)?;

    wrap(move || async move {
        let mut norm = paging.normalize();

        if norm.page_size == Paging::default().page_size {
            norm.page_size = 50;
        }

        match rt {
            RouteType::Authors => match backend.all_authors(norm.page, norm.page_size).await? {
                Some(list) => {
                    let (count, authors) = list.into_parts();

                    let rendered: String = ResourceList::new(
                        format!("{} | authors | explore", paging.page),
                        rt,
                        paging.page,
                        (count + (norm.page_size - 1)) / norm.page_size,
                        authors.iter().map(|a| a as &dyn Resource).collect(),
                    )
                    .render()?;

                    Ok(rendered)
                }
                None => {
                    let rendered =
                        ErrorPage::not_found("404 not found | authors | explore").render()?;

                    Ok(rendered)
                }
            },
            RouteType::Characters => {
                match backend.all_characters(norm.page, norm.page_size).await? {
                    Some(list) => {
                        let (count, characters) = list.into_parts();

                        let rendered: String = ResourceList::new(
                            format!("{} | characters | explore", paging.page),
                            rt,
                            paging.page,
                            (count + (norm.page_size - 1)) / norm.page_size,
                            characters.iter().map(|a| a as &dyn Resource).collect(),
                        )
                        .render()?;

                        Ok(rendered)
                    }
                    None => {
                        let rendered = ErrorPage::not_found("404 not found | characters | explore")
                            .render()?;

                        Ok(rendered)
                    }
                }
            }
            RouteType::Origins => match backend.all_origins(norm.page, norm.page_size).await? {
                Some(list) => {
                    let (count, origins) = list.into_parts();

                    let rendered: String = ResourceList::new(
                        format!("{} | origins | explore", paging.page),
                        rt,
                        paging.page,
                        (count + (norm.page_size - 1)) / norm.page_size,
                        origins.iter().map(|a| a as &dyn Resource).collect(),
                    )
                    .render()?;

                    Ok(rendered)
                }
                None => {
                    let rendered =
                        ErrorPage::not_found("404 not found | origins | explore").render()?;

                    Ok(rendered)
                }
            },
            RouteType::Pairings => {
                // TODO: finish the backend handle for this
                todo!()
            }
            RouteType::Tags => match backend.all_tags(norm.page, norm.page_size).await? {
                Some(list) => {
                    let (count, tags) = list.into_parts();

                    let rendered: String = ResourceList::new(
                        format!("{} | tags | explore", paging.page),
                        rt,
                        paging.page,
                        (count + (norm.page_size - 1)) / norm.page_size,
                        tags.iter().map(|a| a as &dyn Resource).collect(),
                    )
                    .render()?;

                    Ok(rendered)
                }
                None => {
                    let rendered =
                        ErrorPage::not_found("404 not found | tags | explore").render()?;

                    Ok(rendered)
                }
            },
            RouteType::Warnings => match backend.all_warnings(norm.page, norm.page_size).await? {
                Some(list) => {
                    let (count, warnings) = list.into_parts();

                    let rendered: String = ResourceList::new(
                        format!("{} | warnings | explore", paging.page),
                        rt,
                        paging.page,
                        (count + (norm.page_size - 1)) / norm.page_size,
                        warnings.iter().map(|a| a as &dyn Resource).collect(),
                    )
                    .render()?;

                    Ok(rendered)
                }
                None => {
                    let rendered =
                        ErrorPage::not_found("404 not found | warnings | explore").render()?;

                    Ok(rendered)
                }
            },
        }
    })
    .await
}

pub async fn item(
    typ: String,
    id: String,
    paging: Paging,
    backend: DataBackend,
) -> Result<impl Reply, Rejection> {
    let rt = parse(&typ)?;

    wrap(move || async move {
        let norm = paging.normalize();

        let id: Cow<'static, str> = id.into();

        let ret = match rt {
            RouteType::Authors => {
                match backend
                    .author_stories(id.clone(), norm.page, norm.page_size)
                    .await
                    .context("Unable to search backend for authors stories")?
                {
                    Some(list) => {
                        let (count, stories) = list.into_parts();

                        // UNWRAP: database wouldn't return any stories if the author didn't exist
                        let author = backend.get_author(id.clone()).await?.unwrap();

                        Some((
                            format!("{} | {} | authors", paging.page, author.name),
                            count,
                            stories,
                            format!("/authors/{}", id),
                        ))
                    }
                    None => None,
                }
            }
            RouteType::Characters => {
                match backend
                    .character_stories(id.clone(), norm.page, norm.page_size)
                    .await
                    .context("Unable to search backend for character stories")?
                {
                    Some(list) => {
                        let (count, stories) = list.into_parts();

                        // UNWRAP: database wouldn't return any stories if the origin didn't exist
                        let character = backend.get_character(id.clone()).await?.unwrap();

                        Some((
                            format!("{} | {} | characters", paging.page, character.name),
                            count,
                            stories,
                            format!("/characters/{}", id),
                        ))
                    }
                    None => None,
                }
            }
            RouteType::Origins => {
                match backend
                    .origin_stories(id.clone(), norm.page, norm.page_size)
                    .await
                    .context("Unable to search backend for origin stories")?
                {
                    Some(list) => {
                        let (count, stories) = list.into_parts();

                        // UNWRAP: database wouldn't return any stories if the origin didn't exist
                        let origin = backend.get_origin(id.clone()).await?.unwrap();

                        Some((
                            format!("{} | {} | origins", paging.page, origin.name),
                            count,
                            stories,
                            format!("/origins/{}", id),
                        ))
                    }
                    None => None,
                }
            }
            RouteType::Pairings => {
                // TODO: finish the backend handle for this
                todo!()
            }
            RouteType::Tags => {
                match backend
                    .tag_stories(id.clone(), norm.page, norm.page_size)
                    .await
                    .context("Unable to search backend for tag stories")?
                {
                    Some(list) => {
                        let (count, stories) = list.into_parts();

                        // UNWRAP: database wouldn't return any stories if the origin didn't exist
                        let tag = backend.get_tag(id.clone()).await?.unwrap();

                        Some((
                            format!("{} | {} | tags", paging.page, tag.name),
                            count,
                            stories,
                            format!("/tags/{}", id),
                        ))
                    }
                    None => None,
                }
            }
            RouteType::Warnings => {
                match backend
                    .warning_stories(id.clone(), norm.page, norm.page_size)
                    .await
                    .context("Unable to search backend for warning stories")?
                {
                    Some(list) => {
                        let (count, stories) = list.into_parts();

                        // UNWRAP: database wouldn't return any stories if the origin didn't exist
                        let warning = backend.get_warning(id.clone()).await?.unwrap();

                        Some((
                            format!("{} | {} | warnings", paging.page, warning.name),
                            count,
                            stories,
                            format!("/warnings/{}", id),
                        ))
                    }
                    None => None,
                }
            }
        };

        if let Some((title, count, stories, url)) = ret {
            let rendered: String = StoryList::new(
                title,
                url,
                paging.page,
                (count + (norm.page_size - 1)) / norm.page_size,
                stories,
            )
            .render()
            .context("Unable to render item page")?;

            Ok(rendered)
        } else {
            let rendered = ErrorPage::not_found(format!(
                "404 not found | {}",
                match rt {
                    RouteType::Authors => "authors",
                    RouteType::Characters => "characters",
                    RouteType::Origins => "origins",
                    RouteType::Pairings => "pairings",
                    RouteType::Tags => "tags",
                    RouteType::Warnings => "warnings",
                }
            ))
            .render()?;

            Ok(rendered)
        }
    })
    .await
}
