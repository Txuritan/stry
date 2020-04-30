mod controllers;
mod filters;
mod handlers;
mod models;
mod pages;
mod server;

// mod aworker;
mod blocking;
mod config;
mod nanoid;
mod pagination;
// mod requests;
mod schema;
// mod scraper;
// mod worker;

pub(crate) use crate::{blocking::Blocking, nanoid::nanoid, pagination::Pagination};

use {crate::config::Config, db_derive::Pool, std::fmt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        // .json()
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    tracing_log::LogTracer::init()?;

    let cfg = Config::new()?;

    let pool = cfg.create_pool()?;

    match &pool {
        Pool::PostgreSQL(r2) => {
            r2.get()?.batch_execute(&schema(pool.clone())?)?;
        }
        Pool::SQLite(r2) => {
            r2.get()?.execute_batch(&schema(pool.clone())?)?;
        }
    }

    // tracing::info!("Spawning background task worker");
    // let scraper = crate::aworker::Scheduler::new(/* crate::scraper::runner */);
    // tokio::runtime::Handle::current().spawn(scraper);

    tracing::info!("Starting main websever");
    server::start(pool.clone()).await.await;

    // scraper
    //     .join_all()
    //     .expect("unable to send shutdown to scraper");

    Ok(())
}

fn schema(pool: Pool) -> Result<String, fmt::Error> {
    use crate::{
        models::{Author, Chapter, Notification, Origin, Story, Tag},
        schema::Schema,
    };

    let mut buf = String::new();

    Story::schema(pool.clone(), &mut buf)?;

    Author::schema(pool.clone(), &mut buf)?;
    Chapter::schema(pool.clone(), &mut buf)?;
    Origin::schema(pool.clone(), &mut buf)?;
    Tag::schema(pool.clone(), &mut buf)?;

    Notification::schema(pool, &mut buf)?;

    Ok(buf)
}

pub trait Readable: fmt::Display {
    fn readable(&self) -> String {
        let mut num_str = self.to_string();

        let values: Vec<char> = num_str.chars().collect();

        let mut s = String::with_capacity(values.len() + 6);

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
