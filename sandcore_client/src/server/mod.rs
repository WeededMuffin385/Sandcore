use std::io;
use std::net::{TcpStream, ToSocketAddrs};

#[derive(Debug)]
pub struct Server {
	stream: TcpStream,
}

impl Server {
	pub fn new(addr: impl ToSocketAddrs) -> io::Result<Self> {
		let stream = TcpStream::connect(addr)?;

		Ok(Self {
			stream,
		})
	}
}