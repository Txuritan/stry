mod dashboard;
mod logger;
mod models;
mod scraper;
mod server;

mod config;
mod error;
#[macro_use]
mod macros;
mod nanoid;
mod schema;

pub const GIT_VERSION: &str = env!("GIT_VERSION");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Conn = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

pub use crate::error::Error;

use {
    crate::{logger::init_with_level, schema::Backend},
    log::Level,
    std::{
        fmt,
        path::Path,
        sync::{atomic::AtomicBool, Arc},
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Level::Info, Path::new("stry.log"))?;

    let _shutdown = Arc::new(AtomicBool::new(false));

    let backend = Backend::SQLite {
        pool: r2d2::Pool::new(r2d2_sqlite::SqliteConnectionManager::file("stry2.db"))?,
    };

    match &backend {
        Backend::PostgreSQL { pool } => {
            pool.get()?.batch_execute(&schema(backend.clone())?)?;
        }
        Backend::SQLite { pool } => {
            pool.get()?.execute_batch(&schema(backend.clone())?)?;
        }
    }

    let scraper_handle = scraper::start(backend.clone());
    let server_handle = server::start(backend.clone());

    scraper_handle
        .join()
        .expect("unable to join thread: scraper")?;
    server_handle.join().expect("unable to join thread: server");

    Ok(())
}

fn schema(b: Backend) -> Result<String, fmt::Error> {
    use crate::{
        models::{Author, Chapter, Origin, Queue, Story, Tag},
        schema::Schema,
    };

    let mut buf = String::new();

    Story::schema(b.clone(), &mut buf)?;

    Author::schema(b.clone(), &mut buf)?;
    Chapter::schema(b.clone(), &mut buf)?;
    Origin::schema(b.clone(), &mut buf)?;
    Tag::schema(b.clone(), &mut buf)?;

    Queue::schema(b.clone(), &mut buf)?;

    Ok(buf)
}

pub trait Readable: std::fmt::Display {
    fn readable(&self) -> String {
        let mut num_str = self.to_string();

        let values: Vec<char> = num_str.chars().collect();

        let mut s = String::with_capacity(values.len());

        let negative = if values[0] == '-' {
            num_str.remove(0);
            true
        } else {
            false
        };

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
