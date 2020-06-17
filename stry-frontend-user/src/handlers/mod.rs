pub mod dashboard;
// pub mod explore;
// pub mod item;
pub mod search;
pub mod story;

use {
    crate::{
        pages::{ResourceList, StoryList},
        utils::wrap,
    },
    askama::Template,
    std::borrow::Cow,
    stry_backend::DataBackend,
    stry_common::backend::{BackendAuthor, BackendOrigin, BackendStory, BackendTag},
    stry_common::models::{Paging, Resource, RouteType},
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
            RouteType::Authors => {
                match backend.all_authors(norm.page, norm.page_size).await? {
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
                        // TODO: return 404 page
                        todo!()
                    }
                }
            }
            RouteType::Origins => {
                match backend.all_origins(norm.page, norm.page_size).await? {
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
                        // TODO: return 404 page
                        todo!()
                    }
                }
            }
            RouteType::Characters | RouteType::Pairings | RouteType::Tags | RouteType::Warnings => {
                // let (count, tags) = backend
                //     .all_tags_of_type(
                //         match rt {
                //             RouteType::Characters => TagType::Character,
                //             RouteType::Pairings => TagType::Pairing,
                //             RouteType::Tags => TagType::General,
                //             RouteType::Warnings => TagType::Warning,
                //             _ => unreachable!(),
                //         },
                //         norm.page,
                //         norm.page_size,
                //     )
                //     .await?
                //     .into_parts();

                // let rendered: String = ResourceList::new(
                //     format!("{} | {} | explore", paging.page, rt),
                //     rt,
                //     paging.page,
                //     (count + (norm.page_size - 1)) / norm.page_size,
                //     tags.iter().map(|a| a as &dyn Resource).collect(),
                // )
                // .render()?;

                // Ok(rendered)

                // TODO: separate now that these are different tables
                todo!()
            }
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

        let (title, count, stories, url) = match rt {
            RouteType::Authors => {
                match backend
                    .author_stories(id.clone(), norm.page, norm.page_size)
                    .await?
                {
                    Some(list) => {
                        let (count, stories) = list.into_parts();

                        // UNWRAP: database wouldn't return any stories if the author didn't exist
                        let author = backend.get_author(id.clone()).await?.unwrap();

                        (
                            format!("{} | {} | authors", paging.page, author.name),
                            count,
                            stories,
                            format!("/authors/{}", id),
                        )
                    }
                    None => {
                        // TODO: return 404 page
                        todo!()
                    }
                }
            }
            RouteType::Origins => {
                match backend
                    .origin_stories(id.clone(), norm.page, norm.page_size)
                    .await?
                {
                    Some(list) => {
                        let (count, stories) = list.into_parts();

                        // UNWRAP: database wouldn't return any stories if the origin didn't exist
                        let origin = backend.get_origin(id.clone()).await?.unwrap();

                        (
                            format!("{} | {} | origins", paging.page, origin.name),
                            count,
                            stories,
                            format!("/origins/{}", id),
                        )
                    }
                    None => {
                        // TODO: return 404 page
                        todo!()
                    }
                }
            }
            RouteType::Characters | RouteType::Pairings | RouteType::Tags | RouteType::Warnings => {
                // let (count, stories) = backend
                //     .tag_stories(id.clone(), norm.page, norm.page_size)
                //     .await?
                //     .into_parts();

                // let tag = backend.get_tag(id.clone()).await?;

                // (
                //     format!("{} | {} | {}", paging.page, tag.name, rt),
                //     count,
                //     stories,
                //     format!("/{}/{}", rt, id),
                // )

                // TODO: separate now that these are different tables
                todo!()
            }
        };

        let rendered: String = StoryList::new(
            title,
            url,
            paging.page,
            (count + (norm.page_size - 1)) / norm.page_size,
            stories,
        )
        .render()?;

        Ok(rendered)
    })
    .await
}
