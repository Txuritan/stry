use {
    crate::{
        controllers::story,
        models::{Paging, Search, Story},
        pages, Blocking,
    },
    askama::Template,
    db_derive::Pool,
    warp::{Rejection, Reply},
};

pub async fn index(paging: Paging, search: Search, pool: Pool) -> Result<impl Reply, Rejection> {
    Blocking::spawn(concat!(module_path!(), "::index"), move || {
        let norm = paging.normalize();

        if let Some((stories, pages)) = create_search(&pool, norm, &search.search)? {
            let page = pages::Search::new(
                search.search.clone(),
                search.search,
                paging.page,
                pages,
                stories,
            )?;

            let rendered: String = page.render()?;

            Ok(rendered)
        } else {
            todo!("Return no results page")
        }
    })
    .await
}

// TODO: change over value parameters to support postgresql
fn create_search(
    pool: &Pool,
    paging: Paging,
    search: &str,
) -> anyhow::Result<Option<(Vec<Story>, u32)>> {
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

    match pool {
        //#region[rgba(241,153,31,0.1)] PostgreSQL
        Pool::PostgreSQL(r2) => {
            let mut conn = r2.get()?;

            let mut params = and
                .iter()
                .chain(&not)
                .map(|t| t as &(dyn postgres::types::ToSql + Sync))
                .collect::<Vec<_>>();

            params.push(&and_len);

            params.push(&offset);

            let rows = conn.query(search_query.as_str(), &params)?;

            let mut stories: Vec<Story> = Vec::with_capacity(rows.len());

            for row in rows.iter() {
                stories.push(story::get(pool, &row.get::<_, String>("Id"))?);
            }

            let count_rows = conn.query(count_query.as_str(), &params)?;

            if count_rows.is_empty() {
                return Ok(None);
            }

            let count = count_rows.get(0).unwrap().get::<_, u32>("Count");

            Ok(Some((
                stories,
                (count + (paging.page_size - 1)) / paging.page_size,
            )))
        }
        //#endregion

        //#region[rgba(51,103,145,0.1)] SQLite
        Pool::SQLite(r2) => {
            let conn = r2.get()?;
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
                stories.push(story::get(pool, &id?)?);
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
