pub mod sites;

pub mod list;

use {
    self::{list::Site, sites::FanFiction},
    crate::{Error, Pool},
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
    }

    Ok(())
}
