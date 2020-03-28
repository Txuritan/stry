use {
    crate::{
        controllers::{chapter, story},
        pages, Blocking,
    },
    askama::Template,
    db_derive::Pool,
    warp::{reply, Rejection, Reply},
};

pub async fn index(_story_id: String, _pool: Pool) -> Result<impl Reply, Rejection> {
    Ok(reply::html("story"))
}

pub async fn chapter(
    story_id: String,
    mut chapter_page: u32,
    pool: Pool,
) -> Result<impl Reply, Rejection> {
    Blocking::spawn(move || {
        if chapter_page == 0 {
            chapter_page = 1;
        }

        let story = story::get(&pool, &story_id)?;

        if chapter_page <= story.chapters && chapter_page != 0 {
            let chapter = chapter::get(&pool, &story.id, chapter_page)?;

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

            Ok(Blocking::Text(rendered))
        } else {
            Ok(Blocking::Location(format!("/story/{}/1", story_id)))
        }
    })
    .await
}
