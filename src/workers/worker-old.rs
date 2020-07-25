use {
    fenn::BoolExt,
    futures::task::AtomicWaker,
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
    stry_common::config::FourCount,
};

macro_rules! new_worker {
    ($id:expr, $stop:expr, $backend:expr, $task:expr) => {
        Box::pin(Worker {
            id: $id,
            stop: $stop.clone(),
            stopped: false,
            task: Box::pin($task(WorkerData {
                id: $id,
                stop: $stop,
                backend: $backend,
            })),
            waker: AtomicWaker::new(),
        })
    };
}

macro_rules! poll_worker {
    ($worker:expr, $ctx:expr, $stop:expr) => {{
        if !$worker.stopped {
            if $stop {
                $worker.waker.wake();
            }

            if let Poll::Ready(()) = $worker.as_mut().poll($ctx) {
                $worker.stopped = true;

                if !$stop {
                    tracing::warn!(
                        worker_id = $worker.id,
                        "Worker has stopped before the shutdown signal, this is a problem"
                    );
                }
            }
        }
    }};
}

macro_rules! new_worker_group {
    ($id:expr, $count:expr, $stop:expr, $backend:expr, $task:expr) => {{
        let __temp_worker_group_number = if $id == 1 { $id } else { (($id - 1) * 4) + 1 };

        ($count >= $id).some(Box::pin(WorkerGroup {
            id: $id,

            stopped: false,
            stop: $stop,

            one: new_worker!(__temp_worker_group_number, $stop, $backend, $task),
            two: new_worker!(__temp_worker_group_number + 1, $stop, $backend, $task),
            three: new_worker!(__temp_worker_group_number + 2, $stop, $backend, $task),
            four: new_worker!(__temp_worker_group_number + 3, $stop, $backend, $task),

            waker: AtomicWaker::new(),
        }))
    }};
}

macro_rules! poll_worker_group {
    ($ctx:expr, $worker:expr, $stop:expr) => {
        // Poll specific worker group if it exists
        if let Some(group) = $worker.as_mut() {
            let group: &mut PinnedWorkerGroup<'t> = group;

            if $stop {
                group.waker.wake();
            }

            if !group.stopped {
                if let Poll::Ready(()) = group.as_mut().poll($ctx) {
                    if $stop {
                        tracing::warn!(
                            group_id = group.id,
                            "Worker group has stopped before the shutdown signal, this is a problem"
                        );
                    }
                }
            }
        }
    };
}

macro_rules! is_worker_group_stopped {
    ($worker_group:expr) => {
        $worker_group
            .as_ref()
            .map(|w| w.all_stopped())
            .unwrap_or_else(|| true)
    };
}

type PinnedWorker<'t> = Pin<Box<Worker<'t>>>;

type PinnedWorkerGroup<'t> = Pin<Box<WorkerGroup<'t>>>;

pub struct WorkerData {
    pub id: usize,

    pub stop: Arc<AtomicBool>,

    pub backend: DataBackend,
}

pub struct WorkerPool<'t, S>
where
    S: 't,
    S: Future<Output = ()> + Send + Sync,
{
    signal: Pin<Box<S>>,

    stop: Arc<AtomicBool>,

    worker_groups: usize,

    one: Option<PinnedWorkerGroup<'t>>,
    two: Option<PinnedWorkerGroup<'t>>,
    three: Option<PinnedWorkerGroup<'t>>,
    four: Option<PinnedWorkerGroup<'t>>,
    five: Option<PinnedWorkerGroup<'t>>,
    six: Option<PinnedWorkerGroup<'t>>,
    seven: Option<PinnedWorkerGroup<'t>>,
    eight: Option<PinnedWorkerGroup<'t>>,
}

impl<'t, S> WorkerPool<'t, S>
where
    S: 't,
    S: Future<Output = ()> + Send + Sync,
{
    #[allow(clippy::redundant_clone)]
    pub fn new<T>(
        signal: S,
        backend: DataBackend,
        worker_count: FourCount,
        task: impl Fn(WorkerData) -> T,
    ) -> Self
    where
        T: 't,
        T: Future<Output = ()> + Send + Sync,
    {
        let stop = Arc::new(AtomicBool::new(false));

        let worker_groups = worker_count.as_count() / 4;

        tracing::debug!(
            workers = worker_count.as_count(),
            worker_groups = worker_groups,
            "Creating workers"
        );

        Self {
            signal: Box::pin(signal),

            worker_groups,

            one: new_worker_group!(1, worker_groups, stop.clone(), backend.clone(), task),
            two: new_worker_group!(2, worker_groups, stop.clone(), backend.clone(), task),
            three: new_worker_group!(3, worker_groups, stop.clone(), backend.clone(), task),
            four: new_worker_group!(4, worker_groups, stop.clone(), backend.clone(), task),
            five: new_worker_group!(5, worker_groups, stop.clone(), backend.clone(), task),
            six: new_worker_group!(6, worker_groups, stop.clone(), backend.clone(), task),
            seven: new_worker_group!(7, worker_groups, stop.clone(), backend.clone(), task),
            eight: new_worker_group!(8, worker_groups, stop.clone(), backend.clone(), task),

            stop,
        }
    }

    pub fn all_stopped(&self) -> bool {
        is_worker_group_stopped!(self.one)
            && is_worker_group_stopped!(self.two)
            && is_worker_group_stopped!(self.three)
            && is_worker_group_stopped!(self.four)
            && is_worker_group_stopped!(self.five)
            && is_worker_group_stopped!(self.six)
            && is_worker_group_stopped!(self.seven)
            && is_worker_group_stopped!(self.eight)
    }
}

impl<'t, S> Future for WorkerPool<'t, S>
where
    S: 't,
    S: Future<Output = ()> + Send + Sync,
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let span = tracing::debug_span!("WorkerPool::poll", worker_groups = self.worker_groups);

        let _guard = span.enter();

        let mut stop = self.stop.load(Ordering::Acquire);

        if !stop {
            if let Poll::Ready(()) = self.signal.as_mut().poll(ctx) {
                tracing::debug!("Shutdown signal received, starting shutdown");

                self.stop.store(true, Ordering::Release);

                stop = true;
            }
        }

        // This checks if a worker group exists in the array and polls it
        poll_worker_group!(ctx, self.one, stop);
        poll_worker_group!(ctx, self.two, stop);
        poll_worker_group!(ctx, self.three, stop);
        poll_worker_group!(ctx, self.four, stop);
        poll_worker_group!(ctx, self.five, stop);
        poll_worker_group!(ctx, self.six, stop);
        poll_worker_group!(ctx, self.seven, stop);
        poll_worker_group!(ctx, self.eight, stop);

        if stop {
            // Wait for workers to stop
            if self.all_stopped() {
                tracing::debug!("Worker poll has stopped");

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

// To remove iterations from polling, workers are separated into groups of 4
struct WorkerGroup<'t> {
    id: usize,

    stopped: bool,
    stop: Arc<AtomicBool>,

    one: PinnedWorker<'t>,
    two: PinnedWorker<'t>,
    three: PinnedWorker<'t>,
    four: PinnedWorker<'t>,

    waker: AtomicWaker,
}

impl<'t> WorkerGroup<'t> {
    fn all_stopped(&self) -> bool {
        if self.stopped {
            return self.stopped;
        }

        self.one.stopped && self.two.stopped && self.three.stopped && self.four.stopped
    }
}

impl<'t> Future for WorkerGroup<'t> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let span = tracing::debug_span!("WorkerGroup::poll", group_id = self.id);

        let _guard = span.enter();

        if !self.stopped {
            let stop = self.stop.load(Ordering::Acquire);

            if !stop {
                self.waker.register(ctx.waker());
            }

            // Poll each worker if there is one
            poll_worker!(self.one, ctx, stop);
            poll_worker!(self.two, ctx, stop);
            poll_worker!(self.three, ctx, stop);
            poll_worker!(self.four, ctx, stop);

            if self.all_stopped() {
                // If all workers are stopped, set worker group to stopped
                // This group will no longer be polled
                tracing::debug!("Worker group has stopped");

                self.stopped = true;

                Poll::Ready(())
            } else {
                Poll::Pending
            }
        } else {
            tracing::warn!("Worker group was polled even though it has stopped");

            Poll::Ready(())
        }
    }
}

struct Worker<'t> {
    id: usize,

    stop: Arc<AtomicBool>,
    stopped: bool,

    task: Pin<Box<dyn Future<Output = ()> + Send + Sync + 't>>,
    waker: AtomicWaker,
}

impl<'t> Future for Worker<'t> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let span = tracing::debug_span!("Worker::poll", worker_id = self.id);

        let _guard = span.enter();

        if !self.stopped {
            if let Poll::Ready(()) = self.task.as_mut().poll(ctx) {
                tracing::debug!("Worker has stopped");

                self.stopped = true;

                return Poll::Ready(());
            }
        }

        if self.stopped {
            tracing::warn!("Worker was polled even though it has stopped");

            Poll::Ready(())
        } else {
            if !self.stop.load(Ordering::Acquire) {
                self.waker.register(ctx.waker());
            }

            Poll::Pending
        }
    }
}
