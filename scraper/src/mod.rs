pub mod sites;

pub mod list;

use {
    self::{list::Site, sites::FanFiction},
    crate::{Error, ErrorKind, Pool},
    rand::Rng,
    std::{thread, time},
};

pub fn begin(pool: Pool) -> Result<(), Error> {
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
            ErrorKind::Json { .. } => Err(err),
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