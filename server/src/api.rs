use {
    crate::{
        models::{author, chapter, origin, story, tag},
        server::{Request, Response},
        Error, Pool,
    },
    common::models::{
        api::{
            AuthorResponse, ChapterResponse, OriginResponse, SearchRequest, StoryResponse,
            TagResponse, Wrapper,
        },
        Story, TagType,
    },
};

fn get_page<T: serde::Serialize>(
    req: Request,
    query: impl FnOnce(Pool, u32) -> Result<(u32, Vec<T>), Error>,
    res: impl FnOnce(u32, Vec<T>) -> Result<Response, Error>,
) -> Result<Response, Error> {
    let pool = req
        .state()
        .get::<Pool>()
        .ok_or_else(|| Error::state("Pool doesn't exist in server state"))?;

    let mut page: u32 = req
        .params
        .get("page")
        .and_then(|n| n.parse().ok())
        .unwrap_or(1);

    if page == 0 {
        page = 1;
    }

    page -= 1;

    let (count, vec) = query(pool.clone(), page)?;

    res(count, vec)
}

// /api/authors/:page
pub fn authors(req: Request) -> Result<Response, Error> {
    get_page(
        req,
        |pool, page| author::all(pool.clone(), page),
        |count, vec| {
            Ok(Response::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .json(Wrapper::ok(AuthorResponse {
                    count,
                    pages: (f64::from(count) / 100.0).ceil() as u32,
                    authors: vec,
                })))
        },
    )
}

// /api/characters/:page
pub fn characters(req: Request) -> Result<Response, Error> {
    get_page(
        req,
        |pool, page| tag::all_of_type(pool.clone(), TagType::Character, page),
        |count, vec| {
            Ok(Response::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .json(Wrapper::ok(TagResponse {
                    count,
                    pages: (f64::from(count) / 100.0).ceil() as u32,
                    tags: vec,
                })))
        },
    )
}

// /api/origins/:page
pub fn origins(req: Request) -> Result<Response, Error> {
    get_page(
        req,
        |pool, page| origin::all(pool.clone(), page),
        |count, vec| {
            Ok(Response::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .json(Wrapper::ok(OriginResponse {
                    count,
                    pages: (f64::from(count) / 100.0).ceil() as u32,
                    origins: vec,
                })))
        },
    )
}

// /api/pairings/:page
pub fn pairings(req: Request) -> Result<Response, Error> {
    get_page(
        req,
        |pool, page| tag::all_of_type(pool.clone(), TagType::Pairing, page),
        |count, vec| {
            Ok(Response::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .json(Wrapper::ok(TagResponse {
                    count,
                    pages: (f64::from(count) / 100.0).ceil() as u32,
                    tags: vec,
                })))
        },
    )
}

// /api/tags/:page
pub fn tags(req: Request) -> Result<Response, Error> {
    get_page(
        req,
        |pool, page| tag::all_of_type(pool.clone(), TagType::General, page),
        |count, vec| {
            Ok(Response::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .json(Wrapper::ok(TagResponse {
                    count,
                    pages: (f64::from(count) / 100.0).ceil() as u32,
                    tags: vec,
                })))
        },
    )
}

// /api/stories/:page
pub fn stories(req: Request) -> Result<Response, Error> {
    get_page(
        req,
        |pool, page| story::all(pool.clone(), page),
        |count, vec| {
            Ok(Response::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .json(Wrapper::ok(StoryResponse {
                    count,
                    pages: (f64::from(count) / 10.0).ceil() as u32,
                    stories: vec,
                })))
        },
    )
}

// /api/warnings/:page
pub fn warnings(req: Request) -> Result<Response, Error> {
    get_page(
        req,
        |pool, page| tag::all_of_type(pool.clone(), TagType::Warning, page),
        |count, vec| {
            Ok(Response::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .json(Wrapper::ok(TagResponse {
                    count,
                    pages: (f64::from(count) / 100.0).ceil() as u32,
                    tags: vec,
                })))
        },
    )
}

fn get_id_page<T: serde::Serialize>(
    req: Request,
    query: impl FnOnce(Pool, &str, u32) -> Result<(u32, Vec<T>), Error>,
    res: impl FnOnce(u32, Vec<T>) -> Result<Response, Error>,
) -> Result<Response, Error> {
    let pool = req
        .state()
        .get::<Pool>()
        .ok_or_else(|| Error::state("Pool doesn't exist in server state"))?;

    match req.params.get("id") {
        Some(id) => {
            let mut page: u32 = req
                .params
                .get("page")
                .and_then(|n| n.parse().ok())
                .unwrap_or(1);

            if page == 0 {
                page = 1;
            }

            page -= 1;

            let (count, vec) = query(pool.clone(), id, page)?;

            res(count, vec)
        }
        None => Ok(Response::InternalError()
            .header("Content-Type", "text/plain")
            .body("500: Internal Server Error")),
    }
}

// /api/author/:id/:page
pub fn author_stories(req: Request) -> Result<Response, Error> {
    get_id_page(
        req,
        |pool, id, page| author::for_stories(pool.clone(), id, page),
        |count, vec| {
            Ok(Response::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .json(Wrapper::ok(StoryResponse {
                    count,
                    pages: (f64::from(count) / 10.0).ceil() as u32,
                    stories: vec,
                })))
        },
    )
}

// /api/origin/:id/:page
pub fn origin_stories(req: Request) -> Result<Response, Error> {
    get_id_page(
        req,
        |pool, id, page| origin::for_stories(pool.clone(), id, page),
        |count, vec| {
            Ok(Response::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .json(Wrapper::ok(StoryResponse {
                    count,
                    pages: (f64::from(count) / 10.0).ceil() as u32,
                    stories: vec,
                })))
        },
    )
}

// /api/tag/:id/:page
pub fn tag_stories(req: Request) -> Result<Response, Error> {
    get_id_page(
        req,
        |pool, id, page| tag::for_stories(pool.clone(), id, page),
        |count, vec| {
            Ok(Response::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .json(Wrapper::ok(StoryResponse {
                    count,
                    pages: (f64::from(count) / 10.0).ceil() as u32,
                    stories: vec,
                })))
        },
    )
}

// /api/story/:id/chapter/:chapter
pub fn story_chapter(req: Request) -> Result<Response, Error> {
    let pool = req
        .state()
        .get::<Pool>()
        .ok_or_else(|| Error::state("Pool doesn't exist in server state"))?;

    match req.params.get("id") {
        Some(id) => {
            let mut chapter_number: u32 = req
                .params
                .get("chapter")
                .and_then(|n| n.parse().ok())
                .unwrap_or(1);

            if chapter_number == 0 {
                chapter_number = 1;
            }

            let story = story::get(pool.clone(), id)?;

            if chapter_number <= story.chapters && chapter_number != 0 {
                let chapter = chapter::of_story(pool.clone(), &story.id, chapter_number)?;

                Ok(Response::Ok()
                    .header("Access-Control-Allow-Origin", "*")
                    .json(Wrapper::ok(ChapterResponse {
                        chapter: chapter,
                        story: story,
                    })))
            } else {
                Ok(Response::BadRequest()
                    .header("Content-Type", "text/plain")
                    .json(Wrapper::err(400, vec![String::from("Bad Request")])))
            }
        }
        None => Ok(Response::InternalError()
            .header("Content-Type", "text/plain")
            .body("500: Internal Server Error")),
    }
}

pub fn search(mut req: Request) -> Result<Response, Error> {
    if let Some(size) = req.body_length() {
        if size >= 1024 {
            return Ok(Response::BadRequest()
                .header("Access-Control-Allow-Origin", "*")
                .json(Wrapper::err(400, vec![String::from("Bad Request")])));
        }
    }

    if let Some(body) = req.body.as_mut() {
        let search = serde_json::from_str::<SearchRequest>(&body)?;

        if let Some((search_query, count_query, and, not)) = create_search(&search.search) {
            let pool = req
                .state()
                .get::<Pool>()
                .ok_or_else(|| Error::state("Pool doesn't exist in server state"))?;

            // collect stories
            let conn = pool.get()?;
            let mut stmt = conn.prepare(&search_query)?;

            let mut params = and
                .iter()
                .chain(&not)
                .map(|t| t as &dyn rusqlite::ToSql)
                .collect::<Vec<_>>();

            let and_len = and.len() as u32;

            params.push(&and_len);

            let offset = 10 * search.page;

            let count = conn.query_row(&count_query, &params, |row| row.get::<_, i32>("Count"))?;

            params.push(&offset);

            let story_rows = stmt.query_map(&params, |row| row.get::<_, String>("Id"))?;

            let mut stories: Vec<Story> = Vec::new();

            for id in story_rows {
                stories.push(story::get(pool.clone(), &id?)?);
            }

            Ok(Response::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .json(Wrapper::ok(StoryResponse {
                    count: count as u32,
                    pages: (f64::from(count) / 10.0).ceil() as u32,
                    stories,
                })))
        } else {
            Ok(Response::BadRequest()
                .header("Access-Control-Allow-Origin", "*")
                .json(Wrapper::err(400, vec![String::from("Bad Request")])))
        }
    } else {
        Ok(Response::BadRequest()
            .header("Access-Control-Allow-Origin", "*")
            .json(Wrapper::err(400, vec![String::from("Bad Request")])))
    }
}

fn create_search(search: &str) -> Option<(String, String, Vec<String>, Vec<String>)> {
    let mut and = Vec::new();
    let mut not = Vec::new();

    let tags = search.split(',');

    for tag in tags {
        let tag = tag.trim_start().trim_end().to_string();

        if tag.starts_with('-') {
            not.push(
                tag.trim_start_matches('-')
                    .trim_start()
                    .trim_end()
                    .to_string(),
            );
        } else {
            and.push(tag.trim_start().trim_end().to_string());
        }
    }

    if and.is_empty() {
        return None;
    }

    // create search query (using value replacers)
    let mut search_query = String::from("SELECT S.Id");

    query(&mut search_query, &and, &not, false);

    // create count query
    let mut count_query = String::from("SELECT COUNT(*) AS Count FROM (SELECT S.Id");

    query(&mut count_query, &and, &not, true);

    Some((search_query, count_query, and, not))
}

fn query(buf: &mut String, and: &[String], not: &[String], search: bool) {
    buf.push_str(" FROM StoryTag ST, Story S, Tag T WHERE ST.TagId = T.Id AND S.Id = ST.StoryId");

    query_and(buf, and);
    query_not(buf, not);

    buf.push_str(" GROUP BY S.Id HAVING COUNT(S.Id) = ? ORDER BY S.Updated DESC");

    if search {
        buf.push_str(");");
    } else {
        buf.push_str(" LIMIT 10 OFFSET ?;");
    }
}

fn query_and(buf: &mut String, and: &[String]) {
    if !and.is_empty() {
        buf.push_str(" AND (LOWER(T.Name) IN (");

        for (i, _) in and.iter().enumerate() {
            buf.push_str("LOWER(?)");

            if i != and.len() - 1 {
                buf.push_str(", ");
            }
        }

        buf.push_str("))");
    }
}

fn query_not(buf: &mut String, not: &[String]) {
    if !not.is_empty() {
        buf.push_str(" AND S.Id NOT IN (SELECT S.Id FROM Story S, StoryTag ST, Tag T WHERE S.Id = ST.StoryId AND ST.TagId = T.Id AND (LOWER(T.Name) IN (");

        for (i, _) in not.iter().enumerate() {
            buf.push_str("LOWER(?)");

            if i != not.len() - 1 {
                buf.push_str(", ");
            }
        }

        buf.push_str(")))");
    }
}
