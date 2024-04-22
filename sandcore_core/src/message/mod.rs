use serde::{Deserialize, Serialize};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
type HeaderType = u32;
const HEADER_SIZE: usize = core::mem::size_of::<HeaderType>();

pub trait Message: Sized + Serialize + for<'a>Deserialize<'a> {
	async fn read(stream: &mut TcpStream) -> io::Result<Self> {
		Ok(serde_json::from_slice(&get_body(stream).await?).unwrap())
	}

	async fn write(&self, stream: &mut TcpStream) -> io::Result<()> {
		let body = serde_json::to_vec(self).expect("unable to serialize message");
		let length = (body.len() as u32).to_be_bytes();

		stream.write(&length).await?;
		stream.write(&body).await?;
		Ok(())
	}
}

pub async fn get_length(stream: &mut TcpStream) -> io::Result<usize> {
	let mut header_bytes = [0; HEADER_SIZE];

	stream.read_exact(&mut header_bytes).await?;

	Ok(HeaderType::from_be_bytes(header_bytes) as usize)
}

pub async fn get_body(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
	let length = get_length(stream).await?;

	let mut body = vec![0; length];

	stream.read_exact(&mut body).await?;

	Ok(body)
}