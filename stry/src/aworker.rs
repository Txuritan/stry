use {
    crate::models::Site,
    enum_map::EnumMap,
    deadqueue::unlimited::Queue,
    futures::{
        channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
        stream::StreamExt,
        sink::SinkExt,
    },
    std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
        sync::Arc,
    },
};

pub fn channel<Req, Res>() -> (Responder<Req, Res>, Requester<Req, Res>) {
    let (request_sender, request_receiver) = unbounded::<Request<Req, Res>>();

    let c = Responder::new(request_receiver);
    let p = Requester::new(request_sender);

    (c, p)
}

#[derive(Debug)]
pub struct Responder<Req, Res> {
    request_receiver: UnboundedReceiver<Request<Req, Res>>,
}

impl<Req, Res> Responder<Req, Res> {
    fn new(request_receiver: UnboundedReceiver<Request<Req, Res>>) -> Responder<Req, Res> {
        Responder { request_receiver }
    }

    pub async fn get(&mut self) -> Option<Request<Req, Res>> {
        self.request_receiver.next().await
    }
}

#[derive(Clone, Debug)]
pub struct Requester<Req, Res> {
    request_sender: UnboundedSender<Request<Req, Res>>,
}

impl<Req, Res> Requester<Req, Res> {
    fn new(request_sender: UnboundedSender<Request<Req, Res>>) -> Requester<Req, Res> {
        Requester { request_sender }
    }

    pub async fn request(&mut self, request: Req) -> Res {
        let (response_sender, mut response_receiver) = unbounded::<Res>();
        let full_request = Request::new(request, response_sender);

        self.request_sender.send(full_request).await.unwrap();

        response_receiver.next().await.unwrap()
    }
}

#[derive(Debug)]
pub struct Request<Req, Res> {
    pub request: Req,
    response_sender: UnboundedSender<Res>,
}

impl<Req, Res> Request<Req, Res> {
    fn new(request: Req, response_sender: UnboundedSender<Res>) -> Request<Req, Res> {
        Request {
            request,
            response_sender,
        }
    }

    pub async fn respond(&mut self, response: Res) {
        match self.response_sender.send(response).await {
            Ok(_) => (),
            Err(_e) => panic!("Request failed, send pipe was broken during request!"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
}

#[derive(Clone, Debug)]
pub struct Transmitter {
    sender: Requester<TransmitterToScheduler, SchedulerToTransmitter>,
}

impl Transmitter {
    pub async fn send(&mut self, message: Message) {
        self.sender.request(TransmitterToScheduler::Message { message }).await;
    }
}

#[derive(Clone, Debug)]
enum TransmitterToScheduler {
    Message { message: Message },
}

#[derive(Clone, Debug)]
enum SchedulerToTransmitter {}

pub struct Scheduler {
    receiver: Responder<TransmitterToScheduler, SchedulerToTransmitter>,
    workers: EnumMap<Site, Pin<Box<Worker>>>,
}

impl Scheduler {
    pub fn new() -> impl Future<Output = ()> + Send + 'static {
        let (receiver, sender) = channel();

            Self {
                receiver,
                workers: enum_map::enum_map! {
                    Site::ArchiveOfOurOwn => Box::pin(Worker::new()),
                    Site::FanFiction => Box::pin(Worker::new()),
                },
            }
    }

    async fn inner(receiver: &mut Responder<TransmitterToScheduler, SchedulerToTransmitter>, workers: &[(Pin<Box<Worker>>, )]) {
        if let Some(msg) = receiver.get().await {
            match msg.request {
                TransmitterToScheduler::Message { .. } => {}
            }
        }
    }
}

impl Future for Scheduler {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        match self.signal.as_mut().poll(ctx) {
            Poll::Pending => {
                // keep running
                for (_, worker) in &mut self.workers {
                    let poll = worker.as_mut().poll(ctx);

                    tracing::trace!("worker polled with state: {:?}", poll);

                    if poll == Poll::Ready(()) {
                        tracing::warn!("worker has halted, stopping scheduler");

                        return Poll::Ready(())
                    }
                }

                Poll::Pending
            }
            Poll::Ready(_) => {
                // shutdown all workers
                tracing::trace!("shutdown signal received, shutting down workers");

                for worker in &mut self.workers {
                }

                Poll::Ready(())
            }
        }
    }
}

#[derive(Clone, Debug)]
enum SchedulerToWorker {
    Shutdown,
}

#[derive(Clone, Debug)]
enum WorkerToScheduler {}

struct Worker {
    queue: Arc<Queue<String>>,
}

impl Worker {
    fn new() -> Self {
        Self {
            queue: Arc::new(Queue::new()),
        }
    }
}

impl Future for Worker {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        Poll::Pending
    }
}
