use {
    crate::{Site, Sites},
    std::sync::atomic::Ordering,
    stry_backend::DataBackend,
    stry_common::worker::WorkerData,
    stry_models::WorkerSite,
};

macro_rules! stop {
    ($lbl:lifetime, $state:expr) => {
        // tracing::info!("Delay for 1 second");

        tokio::time::delay_for(tokio::time::Duration::from_millis(1)).await;

        if $state.stop.load(Ordering::Acquire) {
            tracing::info!("Received shutdown signal, shutting down");

            break $lbl;
        }
    };
}

#[allow(clippy::unit_arg)]
#[tracing::instrument(skip(state), err)]
pub async fn task(state: WorkerData<DataBackend>) -> anyhow::Result<()> {
    'l: loop {
        stop!('l, state);

        let task = match state.backend.get_new_task().await? {
            Some(task) => task,
            None => {
                // Task check runs every 30 seconds
                tokio::time::delay_for(tokio::time::Duration::from_secs(30)).await;

                continue;
            }
        };

        // TODO: take ownership of the task

        stop!('l, state);

        let site = match task.site {
            WorkerSite::ArchiveOfOurOwn => Sites::ArchiveOfOurOwn,
            WorkerSite::FanFictionNet => Sites::FanFictionNet,
        };

        let mut init = site.init_from_url(task.url.as_str())?;

        stop!('l, state);

        let _details = init.get_details().await?;

        stop!('l, state);

        // Get story chapter (loop)

        stop!('l, state);

        // Store story chapter (loop)

        stop!('l, state);

        // Move story from working

        stop!('l, state);
    }

    Ok(())
}
