use std::io::Read;
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use tokio::runtime::Handle;
use crate::world::creatures::creature::message::Message as CreatureMessage;
use sandcore_core::message_client::MessageClient;

type HeaderType = u32;
const HEADER_SIZE: usize = core::mem::size_of::<HeaderType>();

pub struct Account {
    run: bool,
    stream: TcpStream,
    sender: Option<Sender<CreatureMessage>>
}

impl Account {
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
        let length = {
            let mut header = [0; HEADER_SIZE];
            self.stream.read_exact(&mut header).expect("unable to read");
            HeaderType::from_be_bytes(header)
        };

        let message = {
            let mut body = vec![0; length as usize];
            self.stream.read_exact(&mut body).expect("Unable to read");
            serde_json::from_slice::<MessageClient>(&body).expect("Unable to get message from client")
        };

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