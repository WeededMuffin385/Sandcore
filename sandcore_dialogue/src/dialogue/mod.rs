use std::fmt::Debug;
use std::sync::mpsc;
use std::sync::mpsc::{SendError};
use tokio::sync::oneshot;

pub struct Dialogue<Request, Response: Debug> {
    pub request: Request,
    sender: oneshot::Sender<Response>,
}

impl<Request, Response: Debug> Dialogue<Request, Response> {
    pub fn request(dialogue_sender: &mut mpsc::Sender<Dialogue<Request, Response>>, request: Request) -> Result<oneshot::Receiver<Response>, SendError<Dialogue<Request, Response>>> {
        let (sender, receiver) = oneshot::channel();
        let dialogue = Self {
            request,
            sender,
        };

        dialogue_sender.send(dialogue)?;
        Ok(receiver)
    }

    pub fn response(self, response: Response) -> Result<(), Response> {
        self.sender.send(response)
    }
}