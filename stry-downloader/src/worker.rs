use {
    std::{
        future::Future,
        pin::Pin,
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc, Mutex,
        },
        task::{Context, Poll, Waker},
    },
    stry_backend::DataBackend,
    stry_common::config::FourCount,
};

macro_rules! worker {
    ($id:expr, $stopping:expr, $backend:expr, $task:expr) => {
        (
            false,
            Box::pin(Worker {
                state: WorkerState {
                    id: $id,
                    finished: Arc::new(AtomicBool::new(false)),
                    waker: Arc::new(Mutex::new(None)),
                },
                task: Box::pin($task(WorkerData {
                    id: $id,
                    stopping: $stopping,
                    backend: $backend,
                })),
            }),
        )
    };
    ($pair:expr => $ctx:expr, $stopping:expr) => {{
        let stopped = &mut $pair.0;
        let worker = &mut $pair.1;

        if !*stopped {
            {
                let mut waker_lock = worker.state.waker.lock().unwrap();

                if let Some(waker) = waker_lock.take() {
                    waker.wake();
                }
            }

            if let Poll::Ready(()) = worker.as_mut().poll($ctx) {
                *stopped = true;

                if !$stopping {
                    tracing::warn!(
                        worker_id = worker.state.id,
                        "Worker has stopped before the shutdown signal, this is a problem"
                    );
                }
            }
        }
    }};

    ($ctx:expr, $workers:expr, $index:expr) => {
        if let Some(group) = $workers.get_mut($index) {
            let group: &mut PinnedWorkerGroup<'t> = group;

            if !group.stopped {
                if let Poll::Ready(()) = group.as_mut().poll($ctx) {
                    if !group.stopping.load(Ordering::SeqCst) {
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

type PinnedWorker<'t> = Pin<Box<Worker<'t>>>;

type PinnedWorkerGroup<'t> = Pin<Box<WorkerGroup<'t>>>;

pub struct WorkerData {
    pub id: usize,

    pub stopping: Arc<AtomicBool>,

    pub backend: DataBackend,
}

pub struct WorkerPool<'t, S>
where
    S: 't,
    S: Future<Output = ()> + Send + Sync,
{
    signal: Pin<Box<S>>,

    workers: Vec<PinnedWorkerGroup<'t>>,

    stopping: Arc<AtomicBool>,
}

impl<'t, S> WorkerPool<'t, S>
where
    S: 't,
    S: Future<Output = ()> + Send + Sync,
{
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
        let stopping = Arc::new(AtomicBool::new(false));

        let mut workers: Vec<PinnedWorkerGroup<'t>> = Vec::with_capacity(worker_count.as_count());

        let count = (1..=(worker_count.as_count())).collect::<Vec<usize>>();

        for chunk in count.chunks(4) {
            let (one, two, three, four) = match chunk {
                [one, two, three, four] => (Some(*one), Some(*two), Some(*three), Some(*four)),
                [one, two, three] => (Some(*one), Some(*two), Some(*three), None),
                [one, two] => (Some(*one), Some(*two), None, None),
                [one] => (Some(*one), None, None, None),
                _ => unreachable!(),
            };

            workers.push(Box::pin(WorkerGroup {
                id: (chunk[0] / 4) + 1,

                stopped: false,
                stopping: stopping.clone(),

                one: one.map(|num| worker!(num, stopping.clone(), backend.clone(), task)),
                two: two.map(|num| worker!(num, stopping.clone(), backend.clone(), task)),
                three: three.map(|num| worker!(num, stopping.clone(), backend.clone(), task)),
                four: four.map(|num| worker!(num, stopping.clone(), backend.clone(), task)),
            }));
        }

        Self {
            signal: Box::pin(signal),

            workers,

            stopping,
        }
    }
}

impl<'t, S> Future for WorkerPool<'t, S>
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

        worker!(ctx, self.workers, 0);
        worker!(ctx, self.workers, 1);
        worker!(ctx, self.workers, 2);
        worker!(ctx, self.workers, 3);
        worker!(ctx, self.workers, 4);
        worker!(ctx, self.workers, 5);
        worker!(ctx, self.workers, 6);
        worker!(ctx, self.workers, 7);

        if self.stopping.load(Ordering::SeqCst) {
            // Wait for workers to stop
            if self.workers.iter().all(|group| group.all_stopped()) {
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

struct WorkerGroup<'t> {
    id: usize,

    stopped: bool,
    stopping: Arc<AtomicBool>,

    one: Option<(bool, PinnedWorker<'t>)>,
    two: Option<(bool, PinnedWorker<'t>)>,
    three: Option<(bool, PinnedWorker<'t>)>,
    four: Option<(bool, PinnedWorker<'t>)>,
}

impl<'t> WorkerGroup<'t> {
    fn all_stopped(&self) -> bool {
        if self.stopped {
            return self.stopped;
        }

        match (
            self.one.as_ref(),
            self.two.as_ref(),
            self.three.as_ref(),
            self.four.as_ref(),
        ) {
            (Some(one), Some(two), Some(three), Some(four)) => one.0 && two.0 && three.0 && four.0,
            (Some(one), Some(two), Some(three), None) => one.0 && two.0 && three.0,
            (Some(one), Some(two), None, None) => one.0 && two.0,
            (Some(one), None, None, None) => one.0,
            _ => true,
        }
    }
}

impl<'t> Future for WorkerGroup<'t> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.stopped {
            let stopping = self.stopping.load(Ordering::SeqCst);

            if let Some(pair) = self.one.as_mut() {
                worker!(pair => ctx, stopping);
            }

            if let Some(pair) = self.two.as_mut() {
                worker!(pair => ctx, stopping);
            }

            if let Some(pair) = self.three.as_mut() {
                worker!(pair => ctx, stopping);
            }

            if let Some(pair) = self.four.as_mut() {
                worker!(pair => ctx, stopping);
            }

            if self.all_stopped() {
                tracing::info!(group_id = self.id, "Worker group has stopped");

                self.stopped = true;

                Poll::Ready(())
            } else {
                Poll::Pending
            }
        } else {
            Poll::Ready(())
        }
    }
}

struct Worker<'t> {
    state: WorkerState,
    task: Pin<Box<dyn Future<Output = ()> + Send + Sync + 't>>,
}

struct WorkerState {
    id: usize,
    finished: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl<'t> Future for Worker<'t> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(()) = self.task.as_mut().poll(ctx) {
            self.state.finished.store(true, Ordering::SeqCst);
        }

        if self.state.finished.load(Ordering::SeqCst) {
            tracing::info!(worker_id = self.state.id, "Worker has stopped");

            Poll::Ready(())
        } else {
            let mut state_waker = self.state.waker.lock().unwrap();

            *state_waker = Some(ctx.waker().clone());

            Poll::Pending
        }
    }
}
