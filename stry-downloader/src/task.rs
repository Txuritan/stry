use {crate::worker::WorkerData, std::sync::atomic::Ordering};

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

pub async fn task(state: WorkerData) {
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
