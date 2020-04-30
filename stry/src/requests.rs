//! mpsc_requests is a small library built on top of crossbeam-channel but with
//! the addition of the consumer responding with a message to the producer.
//! Since the producer no longer only produces and the consumer no longer only consumes, the
//! Producer is renamed to [Requester] and the Consumer is renamed to [Responder].
//!
//! mpsc_requests is small and lean by only building on top of the rust standard library
//!
//! A perfect use-case for this library is single-threaded databases which need
//! to be accessed from multiple threads (such as SQLite)
//!
//! # Examples
//! For more examples, see the examples directory
//!
//! For even more examples see the tests in the tests directory
//!
//! ## Simple echo example
//! ```rust,run
//! use std::thread;
//! use mpsc_requests::channel;
//!
//! type RequestType = String;
//! type ResponseType = String;
//! let (responder, requester) = channel::<RequestType, ResponseType>();
//! thread::spawn(move || {
//!     responder.poll_loop(|mut req| {
//!         req.respond(req.body().clone());
//!     });
//! });
//! let msg = String::from("Hello");
//! let res = requester.request(msg.clone());
//! assert_eq!(res, msg);
//! ```

#![deny(missing_docs)]

use {
    crossbeam_channel::{
        unbounded, Receiver, RecvError, RecvTimeoutError, SendError, Sender, TryRecvError,
    },
    std::time::Duration,
};

/// Create a [Requester] and a [Responder] with a channel between them
///
/// The [Requester] can be cloned to be able to do requests to the same [Responder] from multiple
/// threads.
pub fn channel<Req, Res>() -> (Responder<Req, Res>, Requester<Req, Res>) {
    let (request_sender, request_receiver) = unbounded::<Request<Req, Res>>();

    let c = Responder::new(request_receiver);
    let p = Requester::new(request_sender);

    (c, p)
}

/// Errors which can occur when a [Responder] handles a request
#[derive(Debug)]
pub enum RequestError {
    /// Error occuring when channel from [Requester] to [Responder] is broken
    RecvError,

    /// Error occuring when channel from [Responder] to [Requester] is broken
    SendError,

    TryRecvError,
    RecvTimeoutError,
}

impl From<RecvError> for RequestError {
    fn from(_err: RecvError) -> RequestError {
        RequestError::RecvError
    }
}

impl<T> From<SendError<T>> for RequestError {
    fn from(_err: SendError<T>) -> RequestError {
        RequestError::SendError
    }
}

/// A object expected tois a request which is received from the [Responder] poll method
///
/// The request body can be obtained from the body() function and before being
/// dropped it needs to send a response with the respond() function.
/// Not doing a response on a request is considered a programmer error and will result in a panic
/// when the object gets dropped
#[derive(Debug)]
pub struct Request<Req, Res> {
    request: Req,
    response_sender: Sender<Res>,
    _responded: bool,
}

impl<Req, Res> Request<Req, Res> {
    fn new(request: Req, response_sender: Sender<Res>) -> Request<Req, Res> {
        Request {
            request,
            response_sender,
            _responded: false,
        }
    }

    /// Get actual request data
    pub fn body(&self) -> &Req {
        &self.request
    }

    /// TODO
    pub fn respond(&mut self, response: Res) {
        if self._responded {
            panic!("Programmer error, same request cannot respond twice!");
        }

        match self.response_sender.send(response) {
            Ok(_) => (),
            Err(_e) => panic!("Request failed, send pipe was broken during request!"),
        }

        self._responded = true;
    }
}

impl<Req, Res> Drop for Request<Req, Res> {
    fn drop(&mut self) {
        if !self._responded {
            panic!("Dropped request without responding, programmer error!");
        }
    }
}

/// A [Responder] listens to requests of a specific type and responds back to the [Requester]
#[derive(Debug)]
pub struct Responder<Req, Res> {
    request_receiver: Receiver<Request<Req, Res>>,
}

impl<Req, Res> Responder<Req, Res> {
    fn new(request_receiver: Receiver<Request<Req, Res>>) -> Responder<Req, Res> {
        Responder { request_receiver }
    }

    /// Poll if the [Responder] has received any requests.
    /// It then returns a Request which you need to call respond() on before dropping.
    /// Not calling respond is considered a programmer error and will result in a panic
    ///
    /// This call is blocking
    pub fn poll(&self) -> Result<Request<Req, Res>, RequestError> {
        match self.request_receiver.recv() {
            Ok(r) => Ok(r),
            Err(_e) => Err(RequestError::RecvError),
        }
    }

    pub fn try_poll(&self) -> Result<Option<Request<Req, Res>>, RequestError> {
        match self.request_receiver.try_recv() {
            Ok(r) => Ok(Some(r)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(_e) => Err(RequestError::TryRecvError),
        }
    }

    pub fn poll_timeout(&self, dur: Duration) -> Result<Option<Request<Req, Res>>, RequestError> {
        match self.request_receiver.recv_timeout(dur) {
            Ok(r) => Ok(Some(r)),
            Err(RecvTimeoutError::Timeout) => Ok(None),
            Err(_e) => Err(RequestError::RecvTimeoutError),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.request_receiver.is_empty()
    }
}

/// [Requester] has a connection to a [Responder] which it can send requests to
#[derive(Clone, Debug)]
pub struct Requester<Req, Res> {
    request_sender: Sender<Request<Req, Res>>,
}

impl<Req, Res> Requester<Req, Res> {
    fn new(request_sender: Sender<Request<Req, Res>>) -> Requester<Req, Res> {
        Requester { request_sender }
    }

    /// Send request to the connected [Responder]
    pub fn request(&self, request: Req) -> Res {
        let (response_sender, response_receiver) = unbounded::<Res>();
        let full_request = Request::new(request, response_sender);

        self.request_sender.send(full_request).unwrap();

        response_receiver.recv().unwrap()
    }
}
