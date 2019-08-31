use crate::{
    server::{Request, Response},
    Author, Chapter, Error, Origin, Pool, Story, Tag,
};

pub fn no_pool() -> Result<Response, Error> {
    Ok(Response::InternalError()
        .header("Content-Type", "text/plain")
        .body("500: Internal Server Error"))
}

// /api/stories/:page
pub fn stories(req: Request) -> Result<Response, Error> {
    match req.state().get::<Pool>() {
        Some(pool) => {
            let mut page: u32 = req
                .params
                .get("page")
                .and_then(|n| n.parse().ok())
                .unwrap_or(1);

            if page == 0 {
                page = 1;
            }

            page -= 1;

            let (count, stories) = Story::all(pool.clone(), page)?;

            Ok(Response::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .json(json::object! {
                    "data" => json::object! {
                        "count" => count,
                        "pages" => (f64::from(count) / 10.0).ceil(),
                        "stories" => stories,
                    },
                }))
        }
        None => no_pool(),
    }
}

// /api/author/:id/:page
pub fn author_stories(req: Request) -> Result<Response, Error> {
    match req.state().get::<Pool>() {
        Some(pool) => match req.params.get("id") {
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

                let (count, stories) = Author::all(pool.clone(), id, page)?;

                Ok(Response::Ok()
                    .header("Access-Control-Allow-Origin", "*")
                    .json(json::object! {
                        "data" => json::object! {
                            "count" => count,
                            "pages" => (f64::from(count) / 10.0).ceil(),
                            "stories" => stories,
                        },
                    }))
            }
            None => no_pool(),
        },
        None => no_pool(),
    }
}

// /api/origin/:id/:page
pub fn origin_stories(req: Request) -> Result<Response, Error> {
    match req.state().get::<Pool>() {
        Some(pool) => match req.params.get("id") {
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

                let (count, stories) = Origin::all(pool.clone(), id, page)?;

                Ok(Response::Ok()
                    .header("Access-Control-Allow-Origin", "*")
                    .json(json::object! {
                        "data" => json::object! {
                            "count" => count,
                            "pages" => (f64::from(count) / 10.0).ceil(),
                            "stories" => stories,
                        },
                    }))
            }
            None => no_pool(),
        },
        None => no_pool(),
    }
}

// /api/tag/:id/:page
pub fn tag_stories(req: Request) -> Result<Response, Error> {
    match req.state().get::<Pool>() {
        Some(pool) => match req.params.get("id") {
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

                let (count, stories) = Tag::all(pool.clone(), id, page)?;

                Ok(Response::Ok()
                    .header("Access-Control-Allow-Origin", "*")
                    .json(json::object! {
                        "data" => json::object! {
                            "count" => count,
                            "pages" => (f64::from(count) / 10.0).ceil(),
                            "stories" => stories,
                        },
                    }))
            }
            None => no_pool(),
        },
        None => no_pool(),
    }
}

// /api/story/:id/chapter/:chapter
pub fn story_chapter(req: Request) -> Result<Response, Error> {
    match req.state().get::<Pool>() {
        Some(pool) => match req.params.get("id") {
            Some(id) => {
                let mut chapter_number: u32 = req
                    .params
                    .get("chapter")
                    .and_then(|n| n.parse().ok())
                    .unwrap_or(1);

                if chapter_number == 0 {
                    chapter_number = 1;
                }

                let story = Story::get(pool.clone(), id)?;

                if chapter_number <= story.chapters && chapter_number != 0 {
                    let chapter = Chapter::story(pool.clone(), &story.id, chapter_number)?;

                    Ok(Response::Ok()
                        .header("Access-Control-Allow-Origin", "*")
                        .json(json::object! {
                            "data" => json::object! {
                                "chapter" => chapter,
                                "story" => story,
                            },
                        }))
                } else {
                    no_pool()
                }
            }
            None => no_pool(),
        },
        None => no_pool(),
    }
}

pub fn search(mut req: Request) -> Result<Response, Error> {
    if let Some(size) = req.body_length() {
        if size >= 1024 {
            log::info!("size < 1024");

            return Ok(Response::BadRequest()
                .header("Access-Control-Allow-Origin", "*")
                .json(json::object! {
                    "error" => json::object! {
                        "code" => 400i32,
                        "title" => "Bad Request",
                    },
                }));
        }
    }

    if let Some(body) = req.body.as_mut() {
        match json::parse(&body)? {
            json::JsonValue::Object(mut obj) => {
                let mut page: u32 = obj
                    .get("page")
                    .and_then(|n| match n {
                        json::JsonValue::Number(num) => Some(u32::from(*num)),
                        _ => None,
                    })
                    .unwrap_or(1);

                if page == 0 {
                    page = 1;
                }

                page -= 1;

                match obj.get_mut("search") {
                    Some(sv) => {
                        match sv {
                            json::JsonValue::String(_) | json::JsonValue::Short(_) => {
                                let search = sv.take_string().unwrap_or_default();

                                match req.state().get::<Pool>() {
                                    Some(pool) => {
                                        // split input tags
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

                                        // create search query (using value replacers)
                                        let mut search_query = String::from("SELECT S.Id");

                                        query(&mut search_query, &and, &not);

                                        // create count query
                                        let mut count_query =
                                            String::from("SELECT COUNT(S.Id) as Count");

                                        query(&mut count_query, &and, &not);

                                        // collect stories
                                        let conn = pool.get()?;
                                        let mut stmt = conn.prepare(&search_query)?;

                                        let mut params = and
                                            .iter()
                                            .chain(&not)
                                            .map(|t| t as &rusqlite::ToSql)
                                            .collect::<Vec<_>>();

                                        let and_len = and.len() as u32;

                                        params.push(&and_len);

                                        let offset = 10 * page;

                                        params.push(&offset);

                                        let story_rows = stmt
                                            .query_map(&params, |row| row.get::<_, String>("Id"))?;

                                        let count =
                                            conn.query_row(&count_query, &params, |row| {
                                                row.get::<_, i32>("Count")
                                            })?;

                                        let mut stories: Vec<Story> = Vec::new();

                                        for id in story_rows {
                                            stories.push(Story::get(pool.clone(), &id?)?);
                                        }

                                        Ok(Response::Ok()
                                            .header("Access-Control-Allow-Origin", "*")
                                            .json(json::object! {
                                                "data" => json::object! {
                                                    "count" => count,
                                                    "pages" => (f64::from(count) / 10.0).ceil(),
                                                    "stories" => stories,
                                                },
                                            }))
                                    }
                                    None => no_pool(),
                                }
                            }
                            _ => {
                                log::info!("not string");

                                Ok(Response::BadRequest()
                                    .header("Access-Control-Allow-Origin", "*")
                                    .json(json::object! {
                                        "error" => json::object! {
                                            "code" => 400i32,
                                            "title" => "Bad Request",
                                        },
                                    }))
                            }
                        }
                    }
                    None => {
                        log::info!("no search");

                        Ok(Response::BadRequest()
                            .header("Access-Control-Allow-Origin", "*")
                            .json(json::object! {
                                "error" => json::object! {
                                    "code" => 400i32,
                                    "title" => "Bad Request",
                                },
                            }))
                    }
                }
            }
            _ => {
                log::info!("not object");

                Ok(Response::BadRequest()
                    .header("Access-Control-Allow-Origin", "*")
                    .json(json::object! {
                        "error" => json::object! {
                            "code" => 400i32,
                            "title" => "Bad Request",
                        },
                    }))
            }
        }
    } else {
        log::info!("no body");

        Ok(Response::BadRequest()
            .header("Access-Control-Allow-Origin", "*")
            .json(json::object! {
                "error" => json::object! {
                    "code" => 400i32,
                    "title" => "Bad Request",
                },
            }))
    }
}

fn query(buf: &mut String, and: &[String], not: &[String]) {
    buf.push_str(" FROM StoryTag ST, Story S, Tag T WHERE ST.TagId = T.Id AND S.Id = ST.StoryId");

    query_and(buf, and);
    query_not(buf, not);

    buf.push_str(
        " GROUP BY S.Id HAVING COUNT(S.Id) = ? ORDER BY S.Updated DESC LIMIT 10 OFFSET ?;",
    );
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
