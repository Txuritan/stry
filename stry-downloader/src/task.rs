use {crate::downloader::WorkerState, std::sync::atomic::Ordering};

macro_rules! stop {
    ($lbl:lifetime, $state:expr) => {
        if $state.stopping.load(Ordering::SeqCst) {
            tracing::info!(
                worker = $state.id,
                "Received shutdown signal, shutting down"
            );

            break $lbl;
        }
    };
}

pub async fn task(state: WorkerState) {
    'l: loop {
        stop!('l, state);

        // Look for new task (if any)

        stop!('l, state);

        // Get story details

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
}
