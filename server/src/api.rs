use crate::{
    models::tag::TagType,
    server::{Request, Response},
    Author, Chapter, Error, Origin, Pool, Story, Tag,
};

fn get_page<T: Into<json::JsonValue>>(
    req: Request,
    per: f64,
    tag: &'static str,
    query: impl FnOnce(Pool, u32) -> Result<(u32, Vec<T>), Error>,
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

    Ok(Response::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .json(json::object! {
            "data" => json::object! {
                "count" => count,
                "pages" => (f64::from(count) / per).ceil(),
                tag => vec,
            },
        }))
}

// /api/authors/:page
pub fn authors(req: Request) -> Result<Response, Error> {
    get_page(req, 100.0, "authors", |pool, page| {
        Author::all(pool.clone(), page)
    })
}

// /api/characters/:page
pub fn characters(req: Request) -> Result<Response, Error> {
    get_page(req, 100.0, "tags", |pool, page| {
        Tag::all_of_type(pool.clone(), TagType::Character, page)
    })
}

// /api/origins/:page
pub fn origins(req: Request) -> Result<Response, Error> {
    get_page(req, 100.0, "origins", |pool, page| {
        Origin::all(pool.clone(), page)
    })
}

// /api/pairings/:page
pub fn pairings(req: Request) -> Result<Response, Error> {
    get_page(req, 100.0, "tags", |pool, page| {
        Tag::all_of_type(pool.clone(), TagType::Pairing, page)
    })
}

// /api/tags/:page
pub fn tags(req: Request) -> Result<Response, Error> {
    get_page(req, 100.0, "tags", |pool, page| {
        Tag::all_of_type(pool.clone(), TagType::General, page)
    })
}

// /api/stories/:page
pub fn stories(req: Request) -> Result<Response, Error> {
    get_page(req, 10.0, "stories", |pool, page| {
        Story::all(pool.clone(), page)
    })
}

// /api/warnings/:page
pub fn warnings(req: Request) -> Result<Response, Error> {
    get_page(req, 100.0, "tags", |pool, page| {
        Tag::all_of_type(pool.clone(), TagType::Warning, page)
    })
}

fn get_id_page<T: Into<json::JsonValue>>(
    req: Request,
    tag: &'static str,
    query: impl FnOnce(Pool, &str, u32) -> Result<(u32, Vec<T>), Error>,
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

            Ok(Response::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .json(json::object! {
                    "data" => json::object! {
                        "count" => count,
                        "pages" => (f64::from(count) / 10.0).ceil(),
                        tag => vec,
                    },
                }))
        }
        None => Ok(Response::InternalError()
            .header("Content-Type", "text/plain")
            .body("500: Internal Server Error")),
    }
}

// /api/author/:id/:page
pub fn author_stories(req: Request) -> Result<Response, Error> {
    get_id_page(req, "stories", |pool, id, page| {
        Author::for_stories(pool.clone(), id, page)
    })
}

// /api/origin/:id/:page
pub fn origin_stories(req: Request) -> Result<Response, Error> {
    get_id_page(req, "stories", |pool, id, page| {
        Origin::for_stories(pool.clone(), id, page)
    })
}

// /api/tag/:id/:page
pub fn tag_stories(req: Request) -> Result<Response, Error> {
    get_id_page(req, "stories", |pool, id, page| {
        Tag::for_stories(pool.clone(), id, page)
    })
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

            let story = Story::get(pool.clone(), id)?;

            if chapter_number <= story.chapters && chapter_number != 0 {
                let chapter = Chapter::of_story(pool.clone(), &story.id, chapter_number)?;

                Ok(Response::Ok()
                    .header("Access-Control-Allow-Origin", "*")
                    .json(json::object! {
                        "data" => json::object! {
                            "chapter" => chapter,
                            "story" => story,
                        },
                    }))
            } else {
                Ok(Response::BadRequest()
                    .header("Content-Type", "text/plain")
                    .json(json::object! {
                        "error" => json::object! {
                            "code" => 400i32,
                            "title" => "Bad Request",
                        },
                    }))
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

                                let pool = req.state().get::<Pool>().ok_or_else(|| {
                                    Error::state("Pool doesn't exist in server state")
                                })?;

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

                                if and.is_empty() {
                                    return Ok(Response::BadRequest()
                                        .header("Access-Control-Allow-Origin", "*")
                                        .json(json::object! {
                                            "error" => json::object! {
                                                "code" => 400i32,
                                                "title" => "Bad Request",
                                            },
                                        }));
                                }

                                // create search query (using value replacers)
                                let mut search_query = String::from("SELECT S.Id");

                                query(&mut search_query, &and, &not, false);

                                // create count query
                                let mut count_query =
                                    String::from("SELECT COUNT(*) AS Count FROM (SELECT S.Id");

                                query(&mut count_query, &and, &not, true);

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

                                let offset = 10 * page;

                                let count = conn.query_row(&count_query, &params, |row| {
                                    row.get::<_, i32>("Count")
                                })?;

                                params.push(&offset);

                                let story_rows =
                                    stmt.query_map(&params, |row| row.get::<_, String>("Id"))?;

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
                            _ => Ok(Response::BadRequest()
                                .header("Access-Control-Allow-Origin", "*")
                                .json(json::object! {
                                    "error" => json::object! {
                                        "code" => 400i32,
                                        "title" => "Bad Request",
                                    },
                                })),
                        }
                    }
                    None => Ok(Response::BadRequest()
                        .header("Access-Control-Allow-Origin", "*")
                        .json(json::object! {
                            "error" => json::object! {
                                "code" => 400i32,
                                "title" => "Bad Request",
                            },
                        })),
                }
            }
            _ => Ok(Response::BadRequest()
                .header("Access-Control-Allow-Origin", "*")
                .json(json::object! {
                    "error" => json::object! {
                        "code" => 400i32,
                        "title" => "Bad Request",
                    },
                })),
        }
    } else {
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
