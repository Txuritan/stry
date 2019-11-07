use {
    crate::{
        models::{Paging, Search, Story},
        schema::Backend,
        Error, Readable,
    },
    askama::Template,
    warp::{reject::custom, reply, Rejection, Reply},
};

#[derive(serde::Serialize)]
struct SearchUrl<'s> {
    search: &'s str,
}

#[derive(Template)]
#[template(path = "search.html")]
struct SearchPage {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,

    search_url: String,

    page: u32,
    pages: u32,
    prev: u32,
    next: u32,

    stories: Vec<Story>,
}

impl SearchPage {
    fn new(
        title: impl Into<String>,
        search: String,
        page: u32,
        pages: u32,
        stories: Vec<Story>,
    ) -> Result<Self, Error> {
        Ok(Self {
            version: crate::VERSION,
            git: crate::GIT_VERSION,
            title: title.into(),
            prev: if page >= 1 { page - 1 } else { page },
            next: if page <= pages { page + 1 } else { page },
            search_url: serde_urlencoded::to_string(SearchUrl { search: &search })?,
            search: Some(search),
            page,
            pages,
            stories,
        })
    }
}

pub fn index(paging: Paging, search: Search, backend: Backend) -> Result<impl Reply, Rejection> {
    let norm = paging.normalize();

    if let Some((stories, pages)) = create_search(backend.clone(), norm, &search.search)
        .map_err(|err| custom(Error::new(err)))?
    {
        let page = SearchPage::new(
            search.search.clone(),
            search.search,
            paging.page,
            pages,
            stories,
        )
        .map_err(|err| custom(Error::new(err)))?;

        let rendered: String = page.render().map_err(|err| custom(Error::new(err)))?;

        Ok(reply::html(rendered))
    } else {
        Ok(reply::html(format!("page: {}", paging.page)))
    }
}

fn create_search(
    backend: Backend,
    paging: Paging,
    search: &str,
) -> Result<Option<(Vec<Story>, u32)>, Error> {
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
        return Ok(None);
    }

    // create search query (using value replacers)
    let mut search_query = String::from("SELECT S.Id");

    search_query.reserve(search.len() * 18);

    query(&mut search_query, paging.page_size, &and, &not, false);

    // create count query
    let mut count_query = String::from("SELECT COUNT(*) AS Count FROM (SELECT S.Id");

    count_query.reserve(search.len() * 18);

    query(&mut count_query, paging.page_size, &and, &not, true);

    let and_len = and.len() as u32;

    let offset = paging.page_size * (paging.page - 1);

    match &backend {
        //#region[rgba(241,153,31,0.1)] PostgreSQL
        Backend::PostgreSQL { pool } => {
            let conn = pool.get()?;

            let mut params = and
                .iter()
                .chain(&not)
                .map(|t| t as &dyn postgres::types::ToSql)
                .collect::<Vec<_>>();

            params.push(&and_len);

            params.push(&offset);

            let rows = conn.query(&search_query, &params)?;

            let mut stories: Vec<Story> = Vec::with_capacity(rows.len());

            for row in rows.iter() {
                stories.push(Story::get(backend.clone(), &row.get::<_, String>("Id"))?);
            }

            let count_rows = conn.query(&count_query, &params)?;

            if count_rows.is_empty() {
                return Err(Error::no_rows_returned());
            }

            let count = count_rows.get(0).get::<_, u32>("Count");

            Ok(Some((
                stories,
                (count + (paging.page_size - 1)) / paging.page_size,
            )))
        }
        //#endregion

        //#region[rgba(51,103,145,0.1)] SQLite
        Backend::SQLite { pool } => {
            let conn = pool.get()?;
            let mut stmt = conn.prepare(&search_query)?;

            let mut params = and
                .iter()
                .chain(&not)
                .map(|t| t as &dyn rusqlite::ToSql)
                .collect::<Vec<_>>();

            params.push(&and_len);

            let count =
                conn.query_row(&count_query, &params, |row| row.get::<_, i32>("Count"))? as u32;

            params.push(&offset);

            let story_rows = stmt.query_map(&params, |row| row.get::<_, String>("Id"))?;

            let mut stories: Vec<Story> = Vec::new();

            for id in story_rows {
                stories.push(Story::get(backend.clone(), &id?)?);
            }

            Ok(Some((
                stories,
                (count + (paging.page_size - 1)) / paging.page_size,
            )))
        } //#endregion
    }
}

fn query(buf: &mut String, page_size: u32, and: &[String], not: &[String], search: bool) {
    buf.push_str(" FROM StoryTag ST, Story S, Tag T WHERE ST.TagId = T.Id AND S.Id = ST.StoryId");

    query_and(buf, and);
    query_not(buf, not);

    buf.push_str(" GROUP BY S.Id HAVING COUNT(S.Id) = ? ORDER BY S.Updated DESC");

    if search {
        buf.push_str(");");
    } else {
        buf.push_str(" LIMIT ");

        buf.push_str(&page_size.to_string());

        buf.push_str(" OFFSET ?;");
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
