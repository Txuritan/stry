use {
    crate::{
        pages,
        utils::{self, wrap},
    },
    chrono::Utc,
    stry_backend::DataBackend,
    warp::{
        http::{
            header::{HeaderValue, LOCATION},
            Response, StatusCode,
        },
        hyper::Body,
        reply, Rejection, Reply,
    },
};

#[stry_macros::get("/{_story_id}")]
pub async fn index(
    #[data] _backend: DataBackend,
    #[header("Accept-Language")] _languages: String,
    _story_id: String,
) -> Result<impl Reply, Rejection> {
    Ok(reply::html("story"))
}

#[stry_macros::get("/{story_id}/{chapter_page}")]
pub async fn chapter(
    #[data] backend: DataBackend,
    #[header("Accept-Language")] languages: String,
    story_id: String,
    chapter_page: u32,
) -> Result<impl Reply, Rejection> {
    wrap(move || async move {
        let time = Utc::now();

        let mut chapter_page = chapter_page as i32;

        if chapter_page == 0 {
            chapter_page = 1;
        }

        let user_lang = utils::get_languages(&languages);

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
                                time,
                                chapter_page,
                                story,
                                chapter,
                                user_lang,
                            )
                            .into_string()?;

                            Ok(rendered.into_response())
                        }
                        None => {
                            let rendered = pages::ErrorPage::server_error(
                                format!("503 server error | {}", story.name),
                                time,
                                user_lang,
                            )
                            .into_string()?;

                            Ok(rendered.into_response())
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
                let rendered = pages::ErrorPage::server_error("404 not found", time, user_lang)
                    .into_string()?;

                Ok(rendered.into_response())
            }
        }
    })
    .await
}
