use {
    crate::{
        backend::{
            BackendAuthor, BackendCharacter, BackendOrigin, BackendTag, BackendWarning, DataBackend,
        },
        frontend::user::{
            pages::{ErrorPage, ResourceList},
            utils::{wrap, Items, Resource},
        },
        models::Paging,
    },
    askama::Template,
    warp::{Rejection, Reply},
};

pub async fn explore(
    item: Items,
    paging: Paging,
    backend: DataBackend,
) -> Result<impl Reply, Rejection> {
    wrap(move || async move {
        let mut norm = paging.normalize();

        if norm.page_size == Paging::default().page_size {
            norm.page_size = 50;
        }

        let data = match item {
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
            Items::Friends => {
                // TODO: finish the backend handle for this
                todo!()
            }
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
            Items::Pairings => {
                // TODO: finish the backend handle for this
                todo!()
            }
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
                    ErrorPage::not_found(format!("404 not found | {} | explore", item)).render()?;

                Ok(rendered)
            }
        }
    })
    .await
}
