use std::io::Read;
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use tokio::runtime::Handle;
use sandcore_core::message::Message;
use crate::world::creatures::creature::message::Message as CreatureMessage;
use sandcore_core::message_client::MessageClient;

type HeaderType = u32;
const HEADER_SIZE: usize = core::mem::size_of::<HeaderType>();

pub struct Client {
    run: bool,
    stream: TcpStream,
    sender: Option<Sender<CreatureMessage>>
}

impl Client {
    pub fn new(stream: TcpStream, sender: Sender<CreatureMessage>) -> Self {
        Self {
            stream,
            run: true,
            sender: Some(sender),
        }
    }

    pub fn run(mut self, runtime: &Handle) {
        runtime.spawn(async move {
            while self.run {
                self.update();
            }
        });
    }

    fn update(&mut self) {
        self.update_stream();
    }

    fn update_stream(&mut self) {
        let mut header = Message::default();
        header.read(&mut self.stream);
        let message = serde_json::from_slice::<MessageClient>(&header.earn()).expect("Unable to get message from client");


        match message {
            MessageClient::SetDirection(direction) => {
                if let Some(sender) = &self.sender {
                    sender.send(CreatureMessage::SetDirection(direction)).expect("Unable to send message");
                } else {
                    println!("Account doesn't have a creature controller");
                }
            }
        }
    }
}