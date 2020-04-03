pub mod dashboard;
pub mod search;
pub mod story;

use {
    crate::{
        controllers::{self, author, origin, tag},
        models::{Paging, Resource, RouteType, TagType},
        pages::{ResourceList, StoryList},
        Blocking,
    },
    futures::{FutureExt, StreamExt},
    askama::Template,
    db_derive::Pool,
    warp::{Rejection, Reply},
};

pub async fn index(paging: Paging, pool: Pool) -> Result<impl Reply, Rejection> {
    Blocking::spawn(move || {
        let norm = paging.normalize();

        let (count, stories) = controllers::story::all(&pool, norm.page, paging.page_size)?;

        let rendered = StoryList::new(
            "home",
            "/",
            paging.page,
            (count + (paging.page_size - 1)) / paging.page_size,
            stories,
        )
        .render()?;

        Ok(rendered)
    })
    .await
}

pub async fn explore(typ: String, paging: Paging, pool: Pool) -> Result<impl Reply, Rejection> {
    let rt = RouteType::parse(&typ)?;

    Blocking::spawn({
        let pool = pool.clone();

        move || {
            let mut norm = paging.normalize();

            if norm.page_size == Paging::default().page_size {
                norm.page_size = 50;
            }

            match rt {
                RouteType::Authors => {
                    let (count, authors) = author::all(&pool, norm.page, norm.page_size)?;

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
                RouteType::Origins => {
                    let (count, origins) = origin::all(&pool, norm.page, norm.page_size)?;

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
                RouteType::Characters
                | RouteType::Pairings
                | RouteType::Tags
                | RouteType::Warnings => {
                    let (count, tags) = tag::all_of_type(
                        &pool,
                        match rt {
                            RouteType::Characters => TagType::Character,
                            RouteType::Pairings => TagType::Pairing,
                            RouteType::Tags => TagType::General,
                            RouteType::Warnings => TagType::Warning,
                            _ => unreachable!(),
                        },
                        norm.page,
                        norm.page_size,
                    )?;

                    let rendered: String = ResourceList::new(
                        format!("{} | {} | explore", paging.page, rt),
                        rt,
                        paging.page,
                        (count + (norm.page_size - 1)) / norm.page_size,
                        tags.iter().map(|a| a as &dyn Resource).collect(),
                    )
                    .render()?;

                    Ok(rendered)
                }
            }
        }
    })
    .await
}

pub async fn item(
    typ: String,
    id: String,
    paging: Paging,
    pool: Pool,
) -> Result<impl Reply, Rejection> {
    let rt = RouteType::parse(&typ)?;

    Blocking::spawn(move || {
        let norm = paging.normalize();

        let (title, count, stories, url) = match rt {
            RouteType::Authors => {
                let (count, stories) = author::stories(&pool, &id, norm.page, norm.page_size)?;

                let author = author::get(&pool, &id)?;

                (
                    format!("{} | {} | authors", paging.page, author.name),
                    count,
                    stories,
                    format!("/authors/{}", id),
                )
            }
            RouteType::Origins => {
                let (count, stories) = origin::stories(&pool, &id, norm.page, norm.page_size)?;

                let origin = origin::get(&pool, &id)?;

                (
                    format!("{} | {} | origins", paging.page, origin.name),
                    count,
                    stories,
                    format!("/origins/{}", id),
                )
            }
            RouteType::Characters | RouteType::Pairings | RouteType::Tags | RouteType::Warnings => {
                let (count, stories) = tag::stories(&pool, &id, norm.page, norm.page_size)?;

                let tag = tag::get(&pool, &id)?;

                (
                    format!("{} | {} | {}", paging.page, tag.name, rt),
                    count,
                    stories,
                    format!("/{}/{}", rt, id),
                )
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
