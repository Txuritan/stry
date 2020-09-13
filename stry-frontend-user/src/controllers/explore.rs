use {
    crate::{
        pages::{ErrorPage, ResourceList},
        utils::{wrap, Items, Resource},
    },
    askama::Template,
    chrono::Utc,
    stry_backend::DataBackend,
    stry_common::models::Paging,
    warp::{Rejection, Reply},
};

#[warp_macros::get("/explore/{item}")]
pub async fn explore(
    #[data] backend: DataBackend,
    item: Items,
    #[query] paging: Paging,
) -> Result<impl Reply, Rejection> {
    wrap(move || async move {
        let time = Utc::now();

        let mut norm = paging.normalize();

        if norm.page_size == Paging::default().page_size {
            norm.page_size = 50;
        }

        let data: Option<(_, _, Vec<Resource>)> = match item {
            Items::Authors => match backend.all_authors(norm.page, norm.page_size).await? {
                Some(list) => {
                    let (count, entities) = list.into_parts();

                    Some((
                        format!("{} | authors | explore", paging.page),
                        count,
                        entities.into_iter().map(Resource::Author).collect(),
                    ))
                }
                None => None,
            },
            Items::Characters => match backend.all_characters(norm.page, norm.page_size).await? {
                Some(list) => {
                    let (count, entities) = list.into_parts();

                    Some((
                        format!("{} | characters | explore", paging.page),
                        count,
                        entities.into_iter().map(Resource::Character).collect(),
                    ))
                }
                None => None,
            },
            Items::Origins => match backend.all_origins(norm.page, norm.page_size).await? {
                Some(list) => {
                    let (count, entities) = list.into_parts();

                    Some((
                        format!("{} | origins | explore", paging.page),
                        count,
                        entities.into_iter().map(Resource::Origin).collect(),
                    ))
                }
                None => None,
            },
            Items::Pairings => match backend.all_pairings(norm.page, norm.page_size).await? {
                Some(list) => {
                    let (count, entities) = list.into_parts();

                    Some((
                        format!("{} | pairings | explore", paging.page),
                        count,
                        entities.into_iter().map(Resource::Pairing).collect(),
                    ))
                }
                None => None,
            },
            Items::Tags => match backend.all_tags(norm.page, norm.page_size).await? {
                Some(list) => {
                    let (count, entities) = list.into_parts();

                    Some((
                        format!("{} | tags | explore", paging.page),
                        count,
                        entities.into_iter().map(Resource::Tag).collect(),
                    ))
                }
                None => None,
            },
            Items::Warnings => match backend.all_warnings(norm.page, norm.page_size).await? {
                Some(list) => {
                    let (count, entities) = list.into_parts();

                    Some((
                        format!("{} | warnings | explore", paging.page),
                        count,
                        entities.into_iter().map(Resource::Warning).collect(),
                    ))
                }
                None => None,
            },
        };

        match data {
            Some((title, count, resources)) => {
                let rendered: String = ResourceList::new(
                    title,
                    time,
                    format!("/explore/{}", item),
                    paging.page,
                    (count + (norm.page_size - 1)) / norm.page_size,
                    resources,
                )
                .render()?;

                Ok(rendered)
            }
            None => {
                let rendered =
                    ErrorPage::not_found(format!("404 not found | {} | explore", item), time)
                        .render()?;

                Ok(rendered)
            }
        }
    })
    .await
}
