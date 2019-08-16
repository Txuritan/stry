mod http;
mod logger;
mod models;
mod pages;
mod server;

mod error;
mod nanoid;
mod schema;
mod typemap;

pub const CSS: &str = include_str!("./css/anatu.css");
pub const GIT: &str = env!("GIT_VERSION");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Conn = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub use crate::{
    error::{Error, ErrorKind},
    models::*,
    pages::*,
    schema::Schema,
};

use {
    crate::{
        logger::init_with_level,
        server::{Router, Server},
    },
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
            .get("/", Index::home)
            .get("/story/:story/:chapter", ChapterPage::index)
            .get("/author/:author", Index::author)
            .get("/origin/:origin", Index::origin)
            .get("/tag/:tag", Index::tag),
    )?
    .run();

    Ok(())
}

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
    let mut sch = String::new();

    Story::schema(&mut sch)?;

    Author::schema(&mut sch)?;
    Chapter::schema(&mut sch)?;
    Origin::schema(&mut sch)?;
    Tag::schema(&mut sch)?;

    Ok(sch)
}

fn readable(num: u32) -> String {
    let mut num_str = num.to_string();
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
