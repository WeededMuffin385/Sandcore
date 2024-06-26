use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use crate::message_client::MessageClient;
use crate::message_server::MessageServer;

/// Represents the length of a message body in bytes.
type HeaderType = u32;
///  The size in bytes of the type used to represent the length of a message body.
const HEADER_SIZE: usize = core::mem::size_of::<HeaderType>();


/// A trait providing an interface for reading and writing messages asynchronously.
pub trait Message: Serialize + for<'a> Deserialize<'a> {
	async fn read<A: AsyncReadExt + Unpin>(reader: &mut A) -> Result<Self> {
		Ok(bincode::deserialize(&get_body(reader).await?).unwrap())
	}

	async fn write<A: AsyncWriteExt + Unpin>(&self, writer: &mut A) -> Result<()> {
		let body = bincode::serialize(self).unwrap();
		let length = (body.len() as u32).to_be_bytes();

		writer.write(&length).await?;
		writer.write(&body).await?;
		Ok(())
	}
}

async fn get_length<A: AsyncReadExt + Unpin>(reader: &mut A) -> Result<usize> {
	let mut header_bytes = [0; HEADER_SIZE];
	reader.read_exact(&mut header_bytes).await?;
	Ok(HeaderType::from_be_bytes(header_bytes) as usize)
}


async fn get_body<A: AsyncReadExt + Unpin>(reader: &mut A) -> Result<Vec<u8>> {
	let mut body = vec![0; get_length(reader).await?];
	reader.read_exact(&mut body).await?;
	Ok(body)
}


impl Message for MessageClient {}
impl Message for MessageServer {}