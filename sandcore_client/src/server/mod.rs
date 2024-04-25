use std::io;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::*;
use tokio::sync::mpsc::error::{TryRecvError, TrySendError};
use sandcore_core::message::Message;
use sandcore_core::message_client::MessageClient;
use sandcore_core::message_server::MessageServer;

#[derive(Debug)]
pub struct Server {
	sender: Sender<MessageClient>,
	receiver: Receiver<MessageServer>,

	runtime: Runtime,
}

impl Server {
	pub fn new(addr: impl ToSocketAddrs) -> io::Result<Self> {
		let runtime = Runtime::new().unwrap();

		let (reader, writer) = runtime.block_on(TcpStream::connect(addr))?.into_split();

		let (sender, receiver_client) = channel(128);
		let (sender_server, receiver) = channel(128);

		runtime.spawn(async move {
			handle_stream_read(reader, sender_server).await
		});
		runtime.spawn(async move {
			handle_stream_write(writer, receiver_client).await
		});

		Ok(Self{
			sender,
			receiver,
			runtime,
		})
	}

	pub fn try_send(&self, message: MessageClient) -> Result<(), TrySendError<MessageClient>> {
		self.sender.try_send(message)
	}

	pub fn try_recv(&mut self) -> Result<MessageServer, TryRecvError> {
		self.receiver.try_recv()
	}
}

async fn handle_stream_read(mut reader: OwnedReadHalf, mut sender: Sender<MessageServer>) {
	loop {
		if let Ok(message) = MessageServer::read(&mut reader).await {
			sender.send(message).await.unwrap();
		}
	}
}

async fn handle_stream_write(mut writer: OwnedWriteHalf, mut receiver: Receiver<MessageClient>) {
	loop {
		for message in receiver.recv().await {
			message.write(&mut writer).await.unwrap();
		}
	}
}