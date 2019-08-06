mod archiver;
mod models;
mod pages;

mod error;
mod schema;

pub const CSS: &str = include_str!("./css/anatu.css");
pub const GIT: &str = git_version::git_version!();
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Conn = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

pub use crate::{error::Error, models::*, pages::*, schema::Schema};

use {
    actix_http::KeepAlive,
    actix_web::{middleware, web, App, HttpServer},
    std::net::{IpAddr, Ipv4Addr, SocketAddr},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info)?;

    let pool = r2d2::Pool::new(r2d2_sqlite::SqliteConnectionManager::file("stry2.db"))?;

    pool.get()?.execute_batch(&schema()?)?;

    crate::archiver::begin(pool.clone())?;

    let sys = actix_rt::System::new("stry2");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(Index::home))
            .route(
                "/story/{id}/{chapter}",
                web::get().to_async(ChapterPage::index),
            )
            .route("/author/{author_id}", web::get().to_async(Index::author))
            .route("/origin/{origin_id}", web::get().to_async(Index::origin))
            .route("/tag/{tag_id}", web::get().to_async(Index::tag))
    })
    .keep_alive(KeepAlive::Timeout(60))
    .bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8901))?
    .start();

    sys.run()?;

    Ok(())
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

fn word_count(str: &str) -> u32 {
    str.split_whitespace()
        .filter(|s| match *s {
            "---" => false,
            "#" | "##" | "###" | "####" | "#####" | "######" => false,
            "*" | "**" => false,
            _ => true,
        })
        .count() as u32
}
