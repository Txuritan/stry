use {
    crate::{
        pages::{explore, ResourceList, StoryList},
        utils::wrap,
    },
    askama::Template,
    std::borrow::Cow,
    stry_backend::DataBackend,
    stry_common::backend::{BackendAuthor, BackendOrigin, BackendStory, BackendTag},
    stry_common::models::{Paging, Resource, RouteType},
    warp::{reject::not_found, Rejection, Reply},
};

pub async fn authors(paging: Paging, backend: DataBackend) -> Result<impl Reply, Rejection> {
    wrap(move || async move {
        let mut norm = paging.normalize();

        if norm.page_size == Paging::default().page_size {
            norm.page_size = 50;
        }

        let (count, authors) = backend
            .all_authors(norm.page, norm.page_size)
            .await?
            .into_parts();

        let rendered: String = explore::AuthorList::new(
            format!("{} | authors | explore", paging.page),
            paging.page,
            (count + (norm.page_size - 1)) / norm.page_size,
            authors,
        )
        .render()?;

        Ok(rendered)
    }).await
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
                let (count, authors) = backend
                    .all_authors(norm.page, norm.page_size)
                    .await?
                    .into_parts();

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
                let (count, origins) = backend
                    .all_origins(norm.page, norm.page_size)
                    .await?
                    .into_parts();

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
