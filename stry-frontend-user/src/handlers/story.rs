use {
    crate::{pages, utils::wrap},
    askama::Template,
    stry_backend::DataBackend,
    stry_common::backend::{BackendChapter, BackendStory},
    warp::{
        http::{
            header::{HeaderValue, LOCATION},
            Response, StatusCode,
        },
        hyper::Body,
        reply, Rejection, Reply,
    },
};

pub async fn index(_story_id: String, _pool: DataBackend) -> Result<impl Reply, Rejection> {
    Ok(reply::html("story"))
}

pub async fn chapter(
    story_id: String,
    mut chapter_page: u32,
    backend: DataBackend,
) -> Result<impl Reply, Rejection> {
    wrap(move || async move {
        if chapter_page == 0 {
            chapter_page = 1;
        }

        match backend.get_story(story_id.clone().into()).await? {
            Some(story) => {
                if chapter_page <= story.chapters && chapter_page != 0 {
                    match backend
                        .get_chapter(story.id.clone().into(), chapter_page)
                        .await?
                    {
                        Some(chapter) => {
                            let rendered: String = pages::story::Chapter::new(
                                format!(
                                    "chapter {}: {} | {}",
                                    chapter_page, chapter.name, story.name
                                ),
                                chapter_page,
                                story,
                                chapter,
                            )
                            .render()?;

                            Ok(rendered.into_response())
                        }
                        None => {
                            // TODO: return page
                            todo!()
                        }
                    }
                } else {
                    let mut res = Response::new(Body::empty());

                    res.headers_mut().insert(
                        LOCATION,
                        HeaderValue::from_str(&format!("/story/{}/1", story_id)).unwrap(),
                    );

                    *res.status_mut() = StatusCode::MOVED_PERMANENTLY;

                    Ok(res)
                }
            }
            None => {
                // TODO: return a 404
                todo!()
            }
        }
    })
    .await
}
