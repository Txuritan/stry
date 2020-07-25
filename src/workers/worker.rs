use {
    crate::{backend::DataBackend, config::FourCount},
    futures::FutureExt,
    std::{
        future::Future,
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
    },
    tracing_futures::Instrument,
};

struct WorkerGroupData<'t, Fun, Task>
where
    Fun: Fn(WorkerData) -> Task,
    Task: Future<Output = anyhow::Result<()>> + Send + 't,
{
    worker_groups: usize,
    id: usize,
    task: &'t Fun,
    backend: DataBackend,
    stop: Arc<AtomicBool>,
}

impl<'t, Fun, Task> WorkerGroupData<'t, Fun, Task>
where
    Fun: Fn(WorkerData) -> Task,
    Task: Future<Output = anyhow::Result<()>> + Send + 't,
{
    fn bump(&self) -> Self {
        Self {
            worker_groups: self.worker_groups,
            id: self.id + 1,
            task: self.task,
            backend: self.backend.clone(),
            stop: self.stop.clone(),
        }
    }
}

pub struct WorkerData {
    pub id: usize,
    pub backend: DataBackend,
    pub stop: Arc<AtomicBool>,
}

impl WorkerData {
    fn bump(&self) -> Self {
        Self {
            id: self.id + 1,
            backend: self.backend.clone(),
            stop: self.stop.clone(),
        }
    }
}

#[tracing::instrument(level = "debug", skip(signal, task, backend))]
pub async fn worker<'t, Signal, Fun, Task>(
    signal: Signal,
    worker_count: FourCount,
    task: Fun,
    backend: DataBackend,
) where
    Signal: Future<Output = ()> + Send + 't,
    Fun: Fn(WorkerData) -> Task,
    Task: Future<Output = anyhow::Result<()>> + Send + 't,
{
    let stop = Arc::new(AtomicBool::new(false));

    let signal_fut = {
        let stop = stop.clone();

        async move {
            signal.await;

            stop.store(true, Ordering::Release);
        }
    };

    let worker_groups = worker_count.as_count() / 4;

    let data = WorkerGroupData {
        id: 0,
        worker_groups,
        task: &task,
        stop: stop.clone(),
        backend: backend.clone(),
    };

    let one = worker_group(data.bump());
    let two = worker_group(data.bump());
    let three = worker_group(data.bump());
    let four = worker_group(data.bump());
    let five = worker_group(data.bump());
    let six = worker_group(data.bump());
    let seven = worker_group(data.bump());
    let eight = worker_group(data.bump());

    futures::join!(signal_fut, one, two, three, four, five, six, seven, eight);
}

#[tracing::instrument(level = "debug", skip(group_data))]
async fn worker_group<'t, Fun, Task>(
    group_data: WorkerGroupData<'t, Fun, Task>,
) -> futures::future::BoxFuture<'t, ()>
where
    Fun: Fn(WorkerData) -> Task,
    Task: Future<Output = anyhow::Result<()>> + Send + 't,
{
    if group_data.worker_groups >= group_data.id {
        let worker_id = if group_data.id == 1 {
            group_data.id
        } else {
            ((group_data.id - 1) * 4) + 1
        };

        let data = WorkerData {
            id: worker_id - 1,
            backend: group_data.backend.clone(),
            stop: group_data.stop.clone(),
        };

        let one = (group_data.task)(data.bump())
            .instrument(tracing::debug_span!("Worker", worker_id = 1));

        let two = (group_data.task)(data.bump())
            .instrument(tracing::debug_span!("Worker", worker_id = 2));

        let three = (group_data.task)(data.bump())
            .instrument(tracing::debug_span!("Worker", worker_id = 3));

        let four = (group_data.task)(data.bump())
            .instrument(tracing::debug_span!("Worker", worker_id = 4));

        futures::future::join4(one, two, three, four)
            .map(|_| ())
            .instrument(tracing::debug_span!(
                "WorkerGroup",
                group_id = group_data.id
            ))
            .boxed()
    } else {
        futures::future::ready(()).boxed()
    }
}
