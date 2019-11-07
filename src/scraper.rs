use {
    crate::{error::Error, schema::Backend},
    std::thread,
};

pub(crate) fn start(backend: Backend) -> thread::JoinHandle<Result<(), Error>> {
    log::info!("starting scraper");

    thread::spawn({ move || Ok(()) })
}
