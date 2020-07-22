use {crate::worker::WorkerData, std::sync::atomic::Ordering, stry_common::{backend::BackendWorker, models::sync}, story_dl::{Site, Sites}};

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

pub async fn task(state: WorkerData) -> anyhow::Result<()> {
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
            sync::Sites::ArchiveOfOurOwn => Sites::ArchiveOfOurOwn,
            sync::Sites::FanFictionNet => Sites::FanFictionNet,
        };

        // let details = site.get_details(&task.url).await?;

        stop!('l, state);

        // Store story details

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
