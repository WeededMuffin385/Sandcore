use serde::{Deserialize, Serialize};
use std::io::{Read, Write, Result};
use crate::message::{HEADER_SIZE, HeaderType};
pub use crate::message_client::MessageClient;
pub use crate::message_server::MessageServer;


/// A trait providing an interface for reading and writing messages synchronously.
pub trait Message: Serialize + for<'a> Deserialize<'a> {
	fn read<A: Read>(reader: &mut A) -> Result<Self> {
		Ok(bincode::deserialize(&get_body(reader)?).unwrap())
	}

	fn write<A: Write>(&self, writer: &mut A) -> Result<()> {
		let body = bincode::serialize(self).unwrap();
		let length = (body.len() as u32).to_be_bytes();

		writer.write(&length)?;
		writer.write(&body)?;
		Ok(())
	}
}

fn get_length<A: Read>(reader: &mut A) -> Result<usize> {
	let mut header_bytes = [0; HEADER_SIZE];
	reader.read_exact(&mut header_bytes)?;
	Ok(HeaderType::from_be_bytes(header_bytes) as usize)
}

fn get_body<A: Read>(reader: &mut A) -> Result<Vec<u8>> {
	let mut body = vec![0; get_length(reader)?];
	reader.read_exact(&mut body)?;
	Ok(body)
}


impl Message for MessageServer {}
impl Message for MessageClient {}