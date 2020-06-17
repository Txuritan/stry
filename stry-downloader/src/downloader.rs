use {
    std::{
        future::Future,
        pin::Pin,
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        task::{Context, Poll},
    },
    stry_backend::DataBackend,
};

type Worker<'t> = Pin<Box<dyn Future<Output = ()> + Send + Sync + 't>>;

pub struct WorkerState {
    pub id: usize,

    pub stopping: Arc<AtomicBool>,

    pub backend: DataBackend,
}

pub struct Downloader<'t, S>
where
    S: 't,
    S: Future<Output = ()> + Send + Sync,
{
    signal: Pin<Box<S>>,

    workers: Vec<(bool, Worker<'t>)>,

    stopping: Arc<AtomicBool>,
}

impl<'t, S> Downloader<'t, S>
where
    S: 't,
    S: Future<Output = ()> + Send + Sync,
{
    pub fn new<T>(
        signal: S,
        backend: DataBackend,
        worker_count: usize,
        task: impl Fn(WorkerState) -> T,
    ) -> Self
    where
        T: 't,
        T: Future<Output = ()> + Send + Sync,
    {
        let stopping = Arc::new(AtomicBool::new(false));

        let mut workers: Vec<(bool, Worker<'t>)> = Vec::with_capacity(worker_count);

        for id in 1..=worker_count {
            workers.push((
                false,
                Box::pin(task(WorkerState {
                    id,
                    stopping: stopping.clone(),
                    backend: backend.clone(),
                })),
            ));
        }

        Self {
            signal: Box::pin(signal),

            workers,

            stopping,
        }
    }
}

impl<'t, S> Future for Downloader<'t, S>
where
    S: 't,
    S: Future<Output = ()> + Send + Sync,
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.stopping.load(Ordering::SeqCst) {
            if let Poll::Ready(()) = self.signal.as_mut().poll(ctx) {
                tracing::debug!("Shutdown signal received, starting shutdown");

                self.stopping.store(true, Ordering::SeqCst);
            }
        }

        let stopping = self.stopping.load(Ordering::SeqCst);

        for (i, pair) in self.workers.iter_mut().enumerate() {
            let stopped = &mut pair.0;
            let worker = &mut pair.1;

            if !*stopped {
                if let Poll::Ready(()) = worker.as_mut().poll(ctx) {
                    *stopped = true;

                    if !stopping {
                        tracing::warn!(
                            worker_id = i,
                            "Worker has stopped before the shutdown signal, this is a problem"
                        );
                    }
                }
            }
        }

        if stopping {
            // Wait for workers to stop
            if self.workers.iter().all(|(s, _)| *s) {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        } else {
            // Waiting for shutdown
            Poll::Pending
        }
    }
}
