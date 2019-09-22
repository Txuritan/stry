mod logger;
mod models;
mod server;

mod api;
mod error;
mod nanoid;
mod schema;
mod typemap;

pub type Conn = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub use crate::error::{Error, ErrorKind};

use {
    crate::{
        logger::init_with_level,
        server::{Router, Server},
    },
    common::schema::Schema,
    log::Level,
    std::path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Level::Info, Path::new("stry.log"))?;

    let pool = Pool {
        inner: r2d2::Pool::new(r2d2_sqlite::SqliteConnectionManager::file("stry2.db"))?,
    };

    pool.get()?.execute_batch(&schema()?)?;

    let mut state = typemap::TypeMap::custom();

    let _ = state.insert::<Pool>(pool.clone());

    Server::new(
        "0.0.0.0:8901",
        state,
        Router::new()
            .get("/api/authors/:page", api::authors)
            .get("/api/characters/:page", api::characters)
            .get("/api/origins/:page", api::origins)
            .get("/api/pairings/:page", api::pairings)
            .get("/api/tags/:page", api::tags)
            .get("/api/stories/:page", api::stories)
            .get("/api/warnings/:page", api::warnings)
            .get("/api/author/:id/:page", api::author_stories)
            .get("/api/origin/:id/:page", api::origin_stories)
            .get("/api/tag/:id/:page", api::tag_stories)
            .get("/api/story/:id/chapter/:chapter", api::story_chapter)
            .post("/api/search", api::search)
            .get("/", |_req| {
                Ok(crate::server::Response::Ok()
                    .header("Cache-Control", "max-age=900, public")
                    .html(
                        std::str::from_utf8(
                            Assets::get("index.html")
                                .expect("Unable to find `index.html` in assets")
                                .as_ref(),
                        )
                        .expect("`index.html` is no valid UTF-8"),
                    ))
            })
            .get("/assets/:file", asset),
    )?
    .run();

    Ok(())
}

fn asset(req: crate::server::Request) -> Result<crate::server::Response, Error> {
    if let Some(file) = req.params.get("file") {
        if file == &"index.html" {
            return Ok(crate::server::Response::NotFound().body("404: Page Not Found"));
        }

        if let Some(found) = Assets::get(file) {
            if file.ends_with(".css") {
                Ok(crate::server::Response::Ok()
                    .header("Cache-Control", "max-age=900, public")
                    .css(std::str::from_utf8(&found).expect("`index.html` is no valid UTF-8")))
            } else if file.ends_with(".js") {
                Ok(crate::server::Response::Ok()
                    .header("Cache-Control", "max-age=900, public")
                    .js(std::str::from_utf8(&found).expect("`index.html` is no valid UTF-8")))
            } else {
                Ok(crate::server::Response::NotFound().body("404: Page Not Found"))
            }
        } else {
            Ok(crate::server::Response::NotFound().body("404: Page Not Found"))
        }
    } else {
        Ok(crate::server::Response::NotFound().body("404: Page Not Found"))
    }
}

#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/../client/dist"]
pub struct Assets;

#[derive(Clone)]
pub struct Pool {
    inner: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
}

impl std::ops::Deref for Pool {
    type Target = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl typemap::Key for Pool {
    type Value = Self;
}

fn schema() -> Result<String, Box<dyn std::error::Error>> {
    use common::models::{Author, Chapter, Origin, Story, Tag};

    let mut sch = String::new();

    Story::schema(&mut sch)?;

    Author::schema(&mut sch)?;
    Chapter::schema(&mut sch)?;
    Origin::schema(&mut sch)?;
    Tag::schema(&mut sch)?;

    Ok(sch)
}

pub trait Readable: std::fmt::Display {
    fn readable(&self) -> String {
        let mut num_str = self.to_string();
        let mut s = String::new();
        let mut negative = false;

        let values: Vec<char> = num_str.chars().collect();

        if values[0] == '-' {
            num_str.remove(0);
            negative = true;
        }

        for (i, char) in num_str.chars().rev().enumerate() {
            if i % 3 == 0 && i != 0 {
                s.insert(0, ',');
            }
            s.insert(0, char);
        }

        if negative {
            s.insert(0, '-');
        }

        s
    }
}

impl Readable for u32 {}
impl Readable for u64 {}

impl Readable for i32 {}
impl Readable for i64 {}
