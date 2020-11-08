use {
    crate::{Site, Sites},
    stry_backend::DataBackend,
    stry_evermore::Worker,
    stry_models::WorkerSite,
};

macro_rules! stop {
    ($lbl:lifetime, $state:expr) => {
        // tracing::info!("Delay for 1 second");

        tokio::time::delay_for(tokio::time::Duration::from_millis(1)).await;

        if $state.should_stop() {
            tracing::info!("Received shutdown signal, shutting down");

            break $lbl;
        }
    };
}

#[allow(clippy::unit_arg)]
#[tracing::instrument(skip(worker), err)]
pub async fn task(worker: Worker<DataBackend>) -> anyhow::Result<()> {
    'l: loop {
        stop!('l, worker);

        let task = match worker.data.get_new_task().await? {
            Some(task) => task,
            None => {
                // Task check runs every 30 seconds
                tokio::time::delay_for(tokio::time::Duration::from_secs(30)).await;

                continue;
            }
        };

        // TODO: take ownership of the task

        stop!('l, worker);

        let site = match task.site {
            WorkerSite::ArchiveOfOurOwn => Sites::ArchiveOfOurOwn,
            WorkerSite::FanFictionNet => Sites::FanFictionNet,
        };

        let mut init = site.init_from_url(task.url.as_str())?;

        stop!('l, worker);

        let _details = init.get_details().await?;

        stop!('l, worker);

        // Get story chapter (loop)

        stop!('l, worker);

        // Store story chapter (loop)

        stop!('l, worker);

        // Move story from working

        stop!('l, worker);
    }

    Ok(())
}
