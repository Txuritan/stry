#![allow(clippy::unknown_clippy_lints)] // because of pin-project
//! Evermore is a library allows you to run an asynchronous task repeatedly
//! until a shutdown signal is sent out.
//!
//! # Examples
//!
//! The example below shows the normal usage of Evermore (with dummy
//! tasks and data), with tokio's [`broadcast channel`] being used as a
//! shutdown signal sent using [`ctrlc`].
//!
//! ```rust,no_run
//! use stry_evermore::{Evermore, Worker};
//!
//! #[derive(Clone, Debug, Default)]
//! struct Data {}
//!
//! #[tokio::main]
//! async fn main() {
//!     tracing_subscriber::fmt()
//!         .with_max_level(tracing::Level::TRACE)
//!         .with_target(true)
//!         .init();
//!
//!     let (tx, mut rx) = tokio::sync::broadcast::channel(1);
//!
//!     ctrlc::set_handler(move || {
//!         if tx.send(()).is_err() {
//!             tracing::error!("Unable to send shutdown signal");
//!         }
//!     })
//!     .expect("Unable to set CTRL-C handler");
//!
//!     let signal = async move { rx.recv().await.expect("Failed to listen for event") };
//!
//!     Evermore::new(signal, 4, Data::default(), |data: Worker<Data>| {
//!         Box::pin(task(data))
//!     })
//!     .await;
//! }
//!
//! #[tracing::instrument(skip(data))]
//! async fn task(worker: Worker<Data>) -> anyhow::Result<()> {
//!     loop {
//!         tokio::time::delay_for(tokio::time::Duration::from_millis(1)).await;
//!
//!         if worker.should_stop() {
//!             tracing::info!("Received shutdown signal, shutting down");
//!
//!             break;
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! [`broadcast channel`]: https://docs.rs/tokio/0.2.22/tokio/sync/broadcast/fn.channel.html
//! [`ctrlc`]: https://crates.io/crates/ctrlc

use {
    futures::TryFuture,
    std::{
        future::Future,
        marker::Unpin,
        pin::Pin,
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        task::{Context, Poll},
    },
};

/// An graceful shutdown enabled repeating asynchronous task runner.
#[pin_project::pin_project]
pub struct Evermore<S, D, F>
where
    S: Future<Output = ()> + Send,
    D: Clone,
    F: Unpin + factory::Factory<D>,
{
    data: Worker<D>,
    span: tracing::Span,
    workers: Vec<(bool, PinnedWorkerFactory<D, F>)>,
    #[pin]
    signal: S,
}

impl<S, D, F> Evermore<S, D, F>
where
    S: Future<Output = ()> + Send,
    D: Clone,
    F: Unpin + factory::Factory<D>,
    <F as factory::Factory<D>>::Future: Unpin,
{
    pub fn new(signal: S, worker_count: u8, data: D, factory: F) -> Self {
        debug_assert!(worker_count >= 1, "Worker count but not be 0");

        let worker_data = Worker {
            data,
            stop: Arc::new(AtomicBool::new(false)),
        };

        let mut workers = Vec::with_capacity(worker_count as usize);

        for i in 0..(worker_count - 1) {
            workers.push((
                true,
                Box::pin(WorkerFactory::new(
                    i + 1,
                    worker_data.clone(),
                    factory.clone(),
                )),
            ));
        }

        // Push the skipped worker, consuming the factory parameter
        workers.push((
            true,
            Box::pin(WorkerFactory::new(
                worker_count,
                worker_data.clone(),
                factory,
            )),
        ));

        Self {
            data: worker_data,
            span: tracing::info_span!("evermore"),
            workers,
            signal,
        }
    }
}

impl<S, D, F> Future for Evermore<S, D, F>
where
    S: Future<Output = ()> + Send,
    D: Clone,
    F: Unpin + factory::Factory<D>,
    <<F as factory::Factory<D>>::Future as TryFuture>::Error: Into<anyhow::Error> + Send,
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.as_mut().project();

        let _entered = this.span.enter();

        let data: &mut Worker<D> = this.data;
        let workers: &mut Vec<(bool, PinnedWorkerFactory<D, F>)> = this.workers;

        if !data.stop.load(Ordering::SeqCst) {
            tracing::trace!("Polling shutdown signal");

            if let Poll::Ready(()) = this.signal.poll(cx) {
                tracing::debug!("Received shutdown signal, setting `stop` to `true`");

                data.stop.store(true, Ordering::SeqCst);
            }
        }

        if data.stop.load(Ordering::SeqCst) {
            // Only runs once the shutdown signal has been sent
            for entry in workers.iter_mut() {
                let (running, worker): &mut (bool, PinnedWorkerFactory<D, F>) = entry;

                tracing::trace!(id = worker.id, "Polling worker");

                let worker: Pin<&mut WorkerFactory<D, F>> = worker.as_mut();

                let poll: Poll<<<F as factory::Factory<D>>::Future as TryFuture>::Ok> =
                    worker.poll(cx);

                if let Poll::Ready(_res) = poll {
                    *running = false;
                }
            }

            if workers.iter().any(|(running, _)| *running) {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        } else {
            // Poll over every worker until the shutdown signal is sent
            for entry in workers.iter_mut() {
                let (running, worker): &mut (bool, PinnedWorkerFactory<D, F>) = entry;

                let id = worker.id;

                tracing::trace!(id = id, "Polling worker");

                // Only poll the worker if its still running
                // This is incase of the event of a worker returning early
                if *running {
                    let worker: Pin<&mut WorkerFactory<D, F>> = worker.as_mut();

                    let poll: Poll<<<F as factory::Factory<D>>::Future as TryFuture>::Ok> =
                        worker.poll(cx);

                    match poll {
                        Poll::Pending => {}
                        Poll::Ready(_res) => {
                            // TODO: handle value of returned future
                            // Maybe return the error and add it to a error chain
                            tracing::error!(id = id, "Worker has stopped, without the shutdown signal, and has not restarted");

                            *running = false;
                        }
                    }
                }
            }

            Poll::Pending
        }
    }
}

/// The task worker running this task, stores the users shared data.
///
/// This does not allow you to send a shutdown signal or interact
/// with the worker in anyway, it is only used to store user data
/// and the shared stop signal.
#[derive(Debug)]
pub struct Worker<D>
where
    D: Clone,
{
    stop: Arc<AtomicBool>,

    /// The users shared data.
    pub data: D,
}

impl<D> Worker<D>
where
    D: Clone,
{
    /// Returns `true` if the running task should cleanup and shutdown.
    pub fn should_stop(&self) -> bool {
        self.stop.load(Ordering::Acquire)
    }
}

impl<D> Clone for Worker<D>
where
    D: Clone,
{
    fn clone(&self) -> Self {
        Self {
            stop: self.stop.clone(),
            data: self.data.clone(),
        }
    }
}

type PinnedWorkerFactory<D, F> = Pin<Box<WorkerFactory<D, F>>>;

#[pin_project::pin_project]
struct WorkerFactory<D, F>
where
    D: Clone,
    F: Unpin + factory::Factory<D>,
{
    id: u8,
    generation: usize,
    data: Worker<D>,

    #[pin]
    state: FactoryState<F::Future>,
    #[pin]
    factory: F,
}

impl<D, F> WorkerFactory<D, F>
where
    D: Clone,
    F: Unpin + factory::Factory<D>,
{
    #[inline]
    fn new(id: u8, data: Worker<D>, factory: F) -> Self {
        Self {
            id,
            data,
            factory,
            generation: 1,
            state: FactoryState::Idle,
        }
    }
}

impl<D, F> Future for WorkerFactory<D, F>
where
    D: Clone,
    F: Unpin + factory::Factory<D>,
    <<F as factory::Factory<D>>::Future as TryFuture>::Error: Into<anyhow::Error> + Send,
{
    type Output = <<F as factory::Factory<D>>::Future as TryFuture>::Ok;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let span = tracing::info_span!("worker", id = self.id);
        let _entered = span.enter();

        loop {
            let this = self.as_mut().project();

            let generation: &mut usize = this.generation;
            let data: &mut Worker<D> = this.data;

            let mut factory: Pin<&mut F> = this.factory;

            let state = match this.state.project() {
                FactoryStateProject::Idle => {
                    tracing::trace!("No future task, creating from factory");

                    FactoryState::Waiting {
                        task: factory.new(data.clone()),
                    }
                }
                FactoryStateProject::Waiting { task } => {
                    let task: Pin<&mut <F as factory::Factory<D>>::Future> = task;

                    match futures::ready!(task.try_poll(cx)) {
                        Ok(x) => {
                            *generation = 1;

                            return Poll::Ready(x);
                        }
                        Err(e) => {
                            *generation += 1;

                            let err: anyhow::Error = e.into();

                            tracing::error!(error = ?err, "Task failed with error");

                            FactoryState::Waiting {
                                task: factory.new(data.clone()),
                            }
                        }
                    }
                }
            };

            self.as_mut().project().state.set(state);
        }
    }
}

#[pin_project::pin_project(project = FactoryStateProject)]
enum FactoryState<F> {
    Idle,
    Waiting {
        #[pin]
        task: F,
    },
}

mod factory {
    use {super::Worker, futures::TryFuture};

    pub trait Factory<D>: Clone
    where
        D: Clone,
    {
        type Future: TryFuture;

        fn new(&mut self, data: Worker<D>) -> Self::Future;
    }

    impl<D, T, F> Factory<D> for T
    where
        D: Clone,
        T: Unpin + Clone + FnMut(Worker<D>) -> F,
        F: TryFuture,
    {
        type Future = F;

        #[inline]
        fn new(&mut self, data: Worker<D>) -> Self::Future {
            (self)(data)
        }
    }
}
