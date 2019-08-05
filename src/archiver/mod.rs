pub mod sites;

pub mod list;

use {
    self::{list::Site, sites::FanFiction},
    crate::{Error, Pool},
    rand::Rng,
    std::{thread, time},
};

pub fn begin(pool: Pool) -> Result<(), Error> {
    let archive = self::list::Archiver::read();

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

pub fn sleep() {
    let length = rand::thread_rng().gen_range(10, 31);
    log::info!("[util] Sleeping for {} seconds", length);
    thread::sleep(time::Duration::from_secs(length));
}
