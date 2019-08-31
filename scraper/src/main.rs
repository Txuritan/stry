pub mod logger;
pub mod sites;

pub mod error;
pub mod list;
#[macro_use]
mod nanoid;

use {
    crate::{error::ErrorKind, list::Site, logger::init_with_level, sites::FanFiction},
    log::Level,
    rand::Rng,
    std::{path::Path, thread, time},
};

pub use crate::error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Level::Info, Path::new("stry-scraper.log"))?;

    let pool = r2d2::Pool::new(r2d2_sqlite::SqliteConnectionManager::file("stry2.db"))?;

    begin(pool)
}

pub fn begin(
    pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
) -> Result<(), Box<dyn std::error::Error>> {
    match self::list::Archiver::read() {
        Ok(archive) => {
            for story in archive.imports {
                match story.site {
                    Site::ArchiveOfOurOwn => {}
                    Site::FanFiction => {
                        FanFiction::scrape(pool.clone(), &story.id, &story.origins, &story.tags)?;
                    }
                }

                sleep();
            }

            Ok(())
        }
        Err(err) => match err.kind {
            ErrorKind::Json { .. } => Err(err.into()),
            _ => Ok(()),
        },
    }
}

pub fn sleep() {
    let length = rand::thread_rng().gen_range(10, 31);
    log::info!("[util] Sleeping for {} seconds", length);
    thread::sleep(time::Duration::from_secs(length));
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
