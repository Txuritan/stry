use {
    crate::{
        models::{MutexSite, Site},
        requests::{channel, Requester, Responder},
    },
    crossbeam_channel::{unbounded, SendError, Sender},
    either::Either,
    parking_lot::Mutex,
    std::{
        collections::VecDeque,
        sync::Arc,
        thread::{self, JoinHandle},
        time::Duration,
    },
};

#[derive(Clone, Debug)]
pub enum Message {
    Download {
        site: Site,
        url: String,
    },
    Update {
        site: Site,
        url: String,
        chapter: usize,
    },
}

#[derive(Clone, Debug)]
pub enum Request {
    States,
}

#[derive(Clone, Debug)]
pub enum Response {
    States(Vec<WorkerState>),
}

#[derive(Clone, Debug)]
enum InternalMessage {
    Join,
}

#[derive(Clone, Debug)]
enum InternalRequest {
    Join,
    States,
    Message(Message),
    Messages(Vec<Message>),
}

#[derive(Clone, Debug)]
enum InternalResponse {
    Join,
    Blank,
    States(Vec<WorkerState>),
}

#[derive(Debug)]
pub struct WorkerPool {
    handle: JoinHandle<()>,
    requester: Requester<InternalRequest, InternalResponse>,
}

impl WorkerPool {
    pub fn new(
        worker_count: usize,
        fun: impl FnOnce(WorkerState, Message) + Copy + Send + 'static,
    ) -> Self {
        let (responder, requester) = channel();

        let handle = thread::spawn(move || {
            Self::dispatcher(responder, worker_count, fun);
        });

        Self { handle, requester }
    }

    pub fn send(&self, message: Message) {
        self.requester.request(InternalRequest::Message(message));
    }

    pub fn states(&self) -> Vec<WorkerState> {
        match self.requester.request(InternalRequest::States) {
            InternalResponse::States(states) => states,
            _ => unreachable!(),
        }
    }

    pub fn join_all(self) -> thread::Result<()> {
        let _ = self.requester.request(InternalRequest::Join);

        self.handle.join()?;

        Ok(())
    }

    fn dispatcher(
        responder: Responder<InternalRequest, InternalResponse>,
        worker_count: usize,
        fun: impl FnOnce(WorkerState, Message) + Copy + Send + 'static,
    ) {
        let mut workers: Vec<Worker> = Vec::with_capacity(worker_count);

        for _ in 0..worker_count {
            workers.push(Worker::new(fun));
        }

        let mut queue: VecDeque<Message> = VecDeque::with_capacity(100);

        loop {
            if !responder.is_empty() {
                match responder.poll() {
                    Ok(request) => {
                        if Self::handle_request(request, &mut queue, &mut workers) {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }

            if !queue.is_empty() {
                queue.retain(|message| Self::retain(&workers, message));
            }

            // TODO: Figure out why this stops the CPU from being 30%
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn handle_request(
        mut request: crate::requests::Request<InternalRequest, InternalResponse>,
        queue: &mut VecDeque<Message>,
        workers: &mut [Worker],
    ) -> bool {
        match request.body() {
            InternalRequest::Join => {
                Self::joiner(request, workers);

                return true;
            }
            InternalRequest::States => {
                request.respond(InternalResponse::States(
                    workers.iter().map(|w| w.state.clone()).collect(),
                ));
            }
            InternalRequest::Message(message) => match message {
                Message::Download { .. } | Message::Update { .. } => {
                    queue.push_back(message.clone());

                    request.respond(InternalResponse::Blank);
                }
            },
            InternalRequest::Messages(messages) => {
                for message in messages {
                    match message {
                        Message::Download { .. } | Message::Update { .. } => {
                            queue.push_back(message.clone());
                        }
                    }
                }

                request.respond(InternalResponse::Blank);
            }
        }

        false
    }

    fn joiner(
        mut request: crate::requests::Request<InternalRequest, InternalResponse>,
        workers: &mut [Worker],
    ) {
        for worker in workers {
            if worker.send(Either::Left(InternalMessage::Join)).is_ok() {
                let handle = worker.handle.take().expect("Worker has no handle");

                if let Err(err) = handle.join() {
                    eprintln!("{:?}", err);
                };
            }
        }

        request.respond(InternalResponse::Join);
    }

    fn retain(workers: &[Worker], msg: &Message) -> bool {
        match msg {
            Message::Download { site, .. } => {
                if workers.iter().any(|s| s.state.site == *site) {
                    true
                } else {
                    workers.iter().find(|s| s.state.site == None).and_then(|s| {
                        match s.send(Either::Right(msg.clone())) {
                            Ok(_) => Some(()),
                            Err(_) => None,
                        }
                    });

                    false
                }
            }
            Message::Update { site, .. } => {
                if workers.iter().any(|s| s.state.site == *site) {
                    true
                } else {
                    workers.iter().find(|s| s.state.site == None).and_then(|s| {
                        match s.send(Either::Right(msg.clone())) {
                            Ok(_) => Some(()),
                            Err(_) => None,
                        }
                    });

                    false
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Worker {
    handle: Option<JoinHandle<()>>,
    sender: Sender<Either<InternalMessage, Message>>,
    state: WorkerState,
}

impl Worker {
    pub fn new(fun: impl FnOnce(WorkerState, Message) + Copy + Send + 'static) -> Self {
        let (sender, receiver) = unbounded();

        let state = WorkerState::default();

        let handle = thread::spawn({
            let state = state.clone();

            move || loop {
                match receiver.recv() {
                    Ok(Either::Left(msg)) => match msg {
                        InternalMessage::Join => break,
                    },
                    Ok(Either::Right(msg)) => {
                        fun(state.clone(), msg);
                    }
                    Err(_) => break,
                }
            }
        });

        Self {
            handle: Some(handle),
            sender,
            state,
        }
    }

    fn send(
        &self,
        msg: Either<InternalMessage, Message>,
    ) -> Result<(), SendError<Either<InternalMessage, Message>>> {
        self.sender.send(msg)
    }
}

#[derive(Debug)]
pub struct WorkerState {
    pub site: MutexSite,

    pub author: Arc<Mutex<Option<String>>>,
    pub chapter: Arc<Mutex<Option<usize>>>,
    pub title: Arc<Mutex<Option<String>>>,
}

impl Clone for WorkerState {
    fn clone(&self) -> Self {
        Self {
            site: self.site.clone(),

            author: self.author.clone(),
            chapter: self.chapter.clone(),
            title: self.title.clone(),
        }
    }
}

impl Default for WorkerState {
    fn default() -> Self {
        Self {
            site: MutexSite::default(),

            author: Arc::new(Mutex::new(None)),
            chapter: Arc::new(Mutex::new(None)),
            title: Arc::new(Mutex::new(None)),
        }
    }
}
